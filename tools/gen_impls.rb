#!/usr/bin/env ruby
# frozen_string_literal: true

# 段スナップショット生成器（accidents を減らすための「生成物」化）。
#
# 真実の源は impls/steps/partN/lib/ ―― 各章で *変わった/新しいファイルだけ* を置く。
# この生成器が part1..partN を順に重ねて（later overrides earlier）、各到達段階の
# 完全なツリーを impls/dist/partN/ に出力する。dist/ は生成物なので手で編集しない。
#
# 使い方：
#   ruby tools/gen_impls.rb            # 生成のみ
#   ruby tools/gen_impls.rb --verify   # 生成 ＋ 各段の test_stage.rb を実行
#
# CI ゲート例（Makefile の impls-check）：生成して `git diff --exit-code impls/dist`
#   で「dist が steps と同期しているか（手編集されていないか）」を検証する。

require 'fileutils'

ROOT      = File.expand_path('..', __dir__)
STEPS_DIR = File.join(ROOT, 'impls', 'steps')
DIST_DIR  = File.join(ROOT, 'impls', 'dist')

steps = Dir[File.join(STEPS_DIR, 'part*')]
        .select { |d| File.directory?(d) }
        .sort_by { |d| File.basename(d)[/\d+/].to_i }
abort "段ソースが見つかりません: #{STEPS_DIR}" if steps.empty?

verify = ARGV.include?('--verify')
failed = []

FileUtils.rm_rf(DIST_DIR)

steps.each_index do |idx|
  cumulative = steps[0..idx]
  name = File.basename(steps[idx])
  dest = File.join(DIST_DIR, name)

  # part1..partN の lib/ を順に重ねる（同じパスは後の段が上書き＝置換）
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

  files = Dir.glob(File.join(dest, 'lib', '**', '*')).count { |f| File.file?(f) }
  puts "generated #{name}: #{files} files -> #{dest.sub("#{ROOT}/", '')}"

  next unless verify

  test = File.join(steps[idx], 'test_stage.rb')
  unless File.exist?(test)
    puts "  (#{name}: test_stage.rb なし ― スキップ)"
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
