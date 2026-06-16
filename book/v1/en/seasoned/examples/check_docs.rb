# frozen_string_literal: true

# Pin external input to UTF-8 so the Japanese prose reads even when the env's LANG isn't UTF-8.
Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

# check_docs.rb - guards against drift between the prose code blocks (seasoned *.md) and the runnable examples.
#
# How it works (just 3 things):
#   1. Every examples/*.rb self-check is green (`ruby file.rb` exits 0).
#   2. Each ```code block tagged with <!-- include: file.rb#region --> *byte-matches* that file's
#      "# region <id> … # endregion" span (verbatim code sync).
#   3. Each line of a ```text block tagged with <!-- run: file.rb --> appears *as-is*
#      in that file's actual output (verbatim output sync; subset allowed).
#
# Usage:
#   ruby check_docs.rb         # check (exit 1 on drift)
#   ruby check_docs.rb --fix   # regenerate include blocks from regions and sync
#
# Zero dependencies, stdlib only. Runs in CI or locally.

HERE = __dir__
# Chapters only (exclude meta docs like `_`-prefixed handoff notes).
CHAPTERS = Dir[File.join(HERE, '..', '*.md')].sort.reject { |f| File.basename(f).start_with?('_') }
EXAMPLES = Dir[File.join(HERE, '*.rb')].sort.reject { |f| File.basename(f) == File.basename(__FILE__) }
FIX = ARGV.include?('--fix')

problems = []
fixed = 0

# --- 1. Are all examples green? + cache their output -------------------------
outputs = {}
EXAMPLES.each do |path|
  out = `ruby #{path} 2>&1`
  outputs[File.basename(path)] = out
  problems << "example not green: #{File.basename(path)} (exit #{$?.exitstatus})" unless $?.success?
end

# Extract the "# region <id> … # endregion" span (excluding the marker lines).
def extract_region(file, id)
  lines = File.readlines(file, chomp: true)
  s = lines.index { |l| l.strip == "# region #{id}" }
  return nil unless s

  e = (s + 1...lines.size).find { |i| lines[i].strip == '# endregion' }
  return nil unless e

  lines[(s + 1)...e].map(&:rstrip)
end

# --- 2 & 3. Check each chapter's include / run directives ---------------------
CHAPTERS.each do |md|
  lines = File.readlines(md, chomp: true)
  i = 0
  while i < lines.size
    m = lines[i].match(/<!--\s*(include|run):\s*([^\s#]+)(?:#(\S+))?\s*-->/)
    unless m
      i += 1
      next
    end
    kind = m[1]
    file = m[2]
    region = m[3]
    src = File.join(HERE, file)
    open_idx = (i + 1...lines.size).find { |j| lines[j].start_with?('```') }
    close_idx = open_idx && (open_idx + 1...lines.size).find { |j| lines[j].strip == '```' }
    unless open_idx && close_idx
      problems << "#{File.basename(md)}: no closed ``` block right after the directive (#{kind}: #{file})"
      i += 1
      next
    end
    body = lines[(open_idx + 1)...close_idx].map(&:rstrip)
    label = "#{File.basename(md)} [#{kind}: #{file}#{"##{region}" if region}]"

    if kind == 'include'
      want = extract_region(src, region)
      if want.nil?
        problems << "#{label}: region '#{region}' not found in #{file}"
      elsif body != want
        if FIX
          lines[(open_idx + 1)...close_idx] = want
          File.write(md, "#{lines.join("\n")}\n")
          fixed += 1
        else
          problems << "#{label}: prose code differs from region (sync with --fix)"
        end
      end
    else # run
      out = outputs[file]
      if out.nil?
        problems << "#{label}: example #{file} not found"
      else
        out_lines = out.lines.map(&:chomp)
        missing = body.reject(&:empty?).reject { |l| out_lines.include?(l) }
        problems << "#{label}: lines not in output: #{missing.inspect}" unless missing.empty?
      end
    end
    i = close_idx + 1
  end
end

puts "synced #{fixed} include block(s) from regions." if FIX && fixed.positive?

if problems.empty?
  puts "OK: prose code/output is in sync with the examples (chapters: #{CHAPTERS.size}, examples: #{EXAMPLES.size})."
  exit 0
else
  warn 'DRIFT detected:'
  problems.each { |p| warn "  - #{p}" }
  exit 1
end
