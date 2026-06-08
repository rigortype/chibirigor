# frozen_string_literal: true

Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

# check_docs.rb ― little/ の章コードと段スナップショットのドリフトを防ぐ。
#
# 仕組み:
#   <!-- run: examples/partN.rb --> 付きの ```text ブロックの各行が、
#   dist/partN/lib をロードした partN.rb の実出力に含まれるか（subset 照合）。
#   dist/ は examples/ 内に同梱（make impls で生成）。
#
# 実行（examples/ ディレクトリ単体で完結）:
#   ruby check_docs.rb
#   ruby check_docs.rb --fix   # text ブロックを実出力で上書き
#
# 依存ゼロ・stdlib のみ。

HERE    = __dir__
LITTLE  = File.expand_path('..', HERE)
FIX     = ARGV.include?('--fix')

CHAPTERS = Dir[File.join(LITTLE, 'part*.md')].sort
  .reject { |f| File.basename(f).start_with?('_') }

EXAMPLES = Dir[File.join(HERE, 'part*.rb')].sort

# 「# region <id> … # endregion」区間を取り出す（マーカー行は含まない）
def extract_region(file, id)
  lines = File.readlines(file, chomp: true)
  s = lines.index { |l| l.strip == "# region #{id}" }
  return nil unless s

  e = (s + 1...lines.size).find { |i| lines[i].strip == '# endregion' }
  return nil unless e

  lines[(s + 1)...e].map(&:rstrip)
end

# パス解決：examples/ からの相対
def resolve_path(path_str)
  File.join(HERE, path_str)
end

# example を実行して出力を得る。ロードパスは impls/dist/partN/lib から推定。
def run_example(example_path)
  # ファイル内の $LOAD_PATH 設定を尊重してそのまま実行
  out = `ruby "#{example_path}" 2>&1`
  [out, $?.success?]
end

problems = []
fixed    = 0

# --- example 単体が緑か + 出力をキャッシュ -----------------------------------
outputs = {}
EXAMPLES.each do |path|
  out, ok = run_example(path)
  outputs[File.basename(path)] = out
  problems << "example not green: #{File.basename(path)}" unless ok
end

# --- 章ごとに run: ディレクティブを確認 -------------------------------------
CHAPTERS.each do |md|
  lines = File.readlines(md, chomp: true)
  i = 0
  while i < lines.size
    m = lines[i].match(/<!--\s*(run|include):\s*([^\s#]+)(?:#(\S+))?\s*-->/)
    unless m
      i += 1
      next
    end
    kind   = m[1]
    path   = m[2]
    region = m[3]
    src    = resolve_path(path)

    open_idx  = (i + 1...lines.size).find { |j| lines[j].start_with?('```') }
    close_idx = open_idx && (open_idx + 1...lines.size).find { |j| lines[j].strip == '```' }
    unless open_idx && close_idx
      problems << "#{File.basename(md)}: ディレクティブ後に閉じた ``` ブロックがない (#{path})"
      i += 1
      next
    end
    body  = lines[(open_idx + 1)...close_idx].map(&:rstrip)
    label = "#{File.basename(md)} [#{kind}: #{path}#{"##{region}" if region}]"

    if kind == 'include'
      want = extract_region(src, region)
      if want.nil?
        problems << "#{label}: region '#{region}' が #{path} に見つからない"
      elsif body != want
        if FIX
          lines[(open_idx + 1)...close_idx] = want
          File.write(md, "#{lines.join("\n")}\n")
          fixed += 1
        else
          problems << "#{label}: 本文コードが region と不一致（--fix で同期可）"
        end
      end
    else # run
      out = outputs[File.basename(src)]
      if out.nil?
        problems << "#{label}: #{File.basename(src)} が examples/ に見つからない"
      else
        out_lines = out.lines.map(&:chomp)
        missing = body.reject(&:empty?).reject { |l| out_lines.include?(l) }
        problems << "#{label}: 出力に無い行: #{missing.inspect}" unless missing.empty?
      end
    end
    i = close_idx + 1
  end
end

puts "synced #{fixed} include block(s) from regions." if FIX && fixed.positive?

if problems.empty?
  puts "OK: v1/ja/little/ のコード/出力は段スナップショットと同期（chapters: #{CHAPTERS.size}, examples: #{EXAMPLES.size}）。"
  exit 0
else
  warn 'DRIFT detected:'
  problems.each { |p| warn "  - #{p}" }
  exit 1
end
