# frozen_string_literal: true

# 環境の LANG が UTF-8 でなくても日本語の本文を読めるよう、外部入力を UTF-8 に固定する。
Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

# check_docs.rb ― 本文（前編 *.md）のコードブロックと、動く example の乖離（ドリフト）を防ぐ。
#
# 仕組み（後編版と同じ 3 つ、パス解決だけ汎用化）:
#   1. examples/*.rb の自己チェックが全部緑か（`ruby file.rb` が exit 0）。
#   2. <!-- include: path#region --> 付きの ```code ブロックが、その path の
#      「# region <id> … # endregion」区間と *バイト一致* するか（コードの逐語同期）。
#   3. <!-- run: file.rb --> 付きの ```text ブロックの各行が、その file の実出力に
#      *そのまま含まれる*か（出力の逐語同期。subset 可）。
#
# パス解決:
#   - include の path が `../` や絶対パスなら md ファイルからの相対で解決（lib/ 参照に使う）。
#   - それ以外（ファイル名のみ）は examples/ からの相対（後編互換）。
#
# 使い方:
#   ruby check_docs.rb         # チェック（ドリフトがあれば exit 1）
#   ruby check_docs.rb --fix   # include ブロックを region から再生成して同期
#
# 依存ゼロ・stdlib のみ。CI でも手元でも回る。

HERE = __dir__
REPO_ROOT = File.expand_path('../../../..', HERE)

# 章だけを対象に（`_` 始まりのメタ文書・README は除外）
CHAPTERS = Dir[File.join(HERE, '..', '*.md')].sort.reject { |f| File.basename(f).start_with?('_') }
EXAMPLES = Dir[File.join(HERE, '*.rb')].sort.reject { |f| File.basename(f) == File.basename(__FILE__) }
FIX = ARGV.include?('--fix')

problems = []
fixed = 0

# --- 1. examples が全部緑か + 出力をキャッシュ ------------------------------
outputs = {}
EXAMPLES.each do |path|
  out = `ruby #{path} 2>&1`
  outputs[File.basename(path)] = out
  problems << "example not green: #{File.basename(path)} (exit #{$?.exitstatus})" unless $?.success?
end

# 「# region <id> … # endregion」区間を取り出す（マーカー行は含まない）
def extract_region(file, id)
  lines = File.readlines(file, chomp: true)
  s = lines.index { |l| l.strip == "# region #{id}" }
  return nil unless s

  e = (s + 1...lines.size).find { |i| lines[i].strip == '# endregion' }
  return nil unless e

  lines[(s + 1)...e].map(&:rstrip)
end

# include パスの解決:
#   - 先頭が "/" or "../" or "./" → md ファイルからの相対
#   - それ以外 → examples/ からの相対（後編互換）
def resolve_include_path(md_file, path_str)
  if path_str.start_with?('/', '../', './')
    File.expand_path(path_str, File.dirname(md_file))
  else
    File.join(HERE, path_str)
  end
end

# --- 2 & 3. 各章の include / run ディレクティブをチェック -----------------------
CHAPTERS.each do |md|
  lines = File.readlines(md, chomp: true)
  i = 0
  while i < lines.size
    m = lines[i].match(/<!--\s*(include|run):\s*([^\s#]+)(?:#(\S+))?\s*-->/)
    unless m
      i += 1
      next
    end
    kind   = m[1]
    path   = m[2]
    region = m[3]
    src    = resolve_include_path(md, path)

    open_idx  = (i + 1...lines.size).find { |j| lines[j].start_with?('```') }
    close_idx = open_idx && (open_idx + 1...lines.size).find { |j| lines[j].strip == '```' }
    unless open_idx && close_idx
      problems << "#{File.basename(md)}: ディレクティブの直後に閉じた ``` ブロックがない (#{kind}: #{path})"
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
        problems << "#{label}: example #{File.basename(src)} が examples/ に見つからない"
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
  puts "OK: 本文コード/出力は examples と同期しています（chapters: #{CHAPTERS.size}, examples: #{EXAMPLES.size}）。"
  exit 0
else
  warn 'DRIFT detected:'
  problems.each { |p| warn "  - #{p}" }
  exit 1
end
