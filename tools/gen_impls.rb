#!/usr/bin/env ruby
# frozen_string_literal: true

# Stage-snapshot generator (turning the trees into generated artifacts to reduce accidents).
#
# The source of truth is impls/steps/partN/lib/ -- each chapter holds *only the files that
# changed or are new*. This generator stacks part1..partN in order (later overrides earlier)
# and writes the full tree for each stage to impls/dist/partN/. dist/ is generated, so never
# edit it by hand.
#
# Usage:
#   ruby tools/gen_impls.rb            # generate only
#   ruby tools/gen_impls.rb --verify   # generate + run each stage's test_stage.rb
#
# CI gate example (impls-check in the Makefile): generate, then `git diff --exit-code impls/dist`
#   to verify that dist is in sync with steps (i.e. has not been hand-edited).

require 'fileutils'

ROOT         = File.expand_path('..', __dir__)
STEPS_DIR    = File.join(ROOT, 'impls', 'steps')
DIST_DIR     = File.join(ROOT, 'impls', 'dist')
EXAMPLES_DIR = File.join(ROOT, 'book', 'v1', 'ja', 'little', 'examples', 'dist')

steps = Dir[File.join(STEPS_DIR, 'part*')]
        .select { |d| File.directory?(d) }
        .sort_by { |d| File.basename(d)[/\d+/].to_i }
abort "no stage sources found: #{STEPS_DIR}" if steps.empty?

verify = ARGV.include?('--verify')
failed = []

FileUtils.rm_rf(DIST_DIR)
FileUtils.rm_rf(EXAMPLES_DIR)

steps.each_index do |idx|
  cumulative = steps[0..idx]
  name = File.basename(steps[idx])
  dest = File.join(DIST_DIR, name)

  # Stack the lib/ trees of part1..partN in order (a later stage overrides = replaces the same path)
  cumulative.each do |sdir|
    libsrc = File.join(sdir, 'lib')
    next unless File.directory?(libsrc)

    Dir.glob(File.join(libsrc, '**', '*')).each do |f|
      next if File.directory?(f)

      rel    = f.sub("#{libsrc}/", '')
      target = File.join(dest, 'lib', rel)
      FileUtils.mkdir_p(File.dirname(target))
      FileUtils.cp(f, target)
    end
  end

  # Also sync into book/v1/ja/little/examples/dist/partN/lib (so the examples are self-contained)
  ex_dest = File.join(EXAMPLES_DIR, name, 'lib')
  FileUtils.mkdir_p(ex_dest)
  FileUtils.cp_r(File.join(dest, 'lib', '.'), ex_dest)

  files = Dir.glob(File.join(dest, 'lib', '**', '*')).count { |f| File.file?(f) }
  puts "generated #{name}: #{files} files -> #{dest.sub("#{ROOT}/", '')}"

  next unless verify

  test = File.join(steps[idx], 'test_stage.rb')
  unless File.exist?(test)
    puts "  (#{name}: no test_stage.rb -- skipped)"
    next
  end
  ok = system('ruby', '-I', File.join(dest, 'lib'), test)
  failed << name unless ok
end

if verify && !failed.empty?
  warn "STAGE TEST FAILURES: #{failed.join(', ')}"
  exit 1
end

puts(verify ? 'all stage tests passed.' : 'generated all stages.')
