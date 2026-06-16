#!/usr/bin/env ruby
# frozen_string_literal: true

# tools/ja_format.rb
#
# 和文（ひらがな・カタカナ・漢字・長音符・々）と半角（ASCII 英数および
# 和文に隣接しうるインライン記号 ` * _ [ ] ( ) < >）の境界にある半角スペースを
# 詰める Markdown 正規化ツール。文章規範 japanese-tech-writing の整形ステップ。
#
# 本文だけを対象にし、次は保護して触らない：
#   - YAML frontmatter（先頭 --- から次の --- まで）
#   - コードフェンス（``` / ~~~ で囲まれた範囲）
#   - インラインコード（`...`）の内側
#   - 表の行（先頭が | の行）
#   - 行頭の Markdown マーカーと、その直後の必須スペース
#       （見出し #、箇条書き - * +、番号付き 1.、引用 >、それらのネスト）
#
# 折り返し改行（一文一行）は対象外。これは本文を手で書くときの構造判断に任せる。
#
# usage:
#   ruby tools/ja_format.rb --check PATH...   # 変更箇所を報告（変更があれば exit 1）
#   ruby tools/ja_format.rb --write PATH...   # 実際に書き換える
# PATH はファイルかディレクトリ（ディレクトリは *.md を再帰探索）。

require 'find'

JP_CLASS   = /[\p{Hiragana}\p{Katakana}\p{Han}ー々〻]/.freeze
HALF_CLASS = /[A-Za-z0-9`*_\[\]()<>]/.freeze

# 行頭の保護プレフィックス（インデント + ネストした > + 1 つのマーカー + 必須スペース）。
# このプレフィックスのスペースは触らず、後続の本文だけを詰める。
LINE_MARKER = /\A([ \t]*(?:>[ \t]*)*(?:[-*+][ \t]|\d+\.[ \t]|\#{1,6}[ \t])?)(.*)\z/m

# 本文 1 行ぶんを詰める。code span の内側は保護する。
def tighten_content(text)
  protected_idx = Array.new(text.length, false)
  text.to_enum(:scan, /`[^`]*`/).each do
    m = Regexp.last_match
    (m.begin(0)...m.end(0)).each { |k| protected_idx[k] = true }
  end

  chars = text.chars
  out = +''
  i = 0
  n = chars.length
  while i < n
    if chars[i] == ' ' && !protected_idx[i]
      j = i
      j += 1 while j < n && chars[j] == ' ' && !protected_idx[j]
      before = i.positive? ? chars[i - 1] : nil
      after  = j < n ? chars[j] : nil
      boundary =
        before && after &&
        ((before.match?(JP_CLASS) && after.match?(HALF_CLASS)) ||
         (before.match?(HALF_CLASS) && after.match?(JP_CLASS)))
      out << chars[i...j].join unless boundary # 境界なら詰める（何も足さない）
      i = j
    else
      out << chars[i]
      i += 1
    end
  end
  out
end

def tighten_line(line)
  # 表行は保護（引用でネストした「> | ... |」も含む）
  return line if line.lstrip.sub(/\A(?:>[ \t]*)+/, '').start_with?('|')

  prefix, rest = line.match(LINE_MARKER).captures
  # 見出しでは、先頭の節番号やラベル（0.1／2-0.／前編／後編／付録）の直後スペースは
  # ラベル区切りなので保護。見出し中の本文側の和欧境界（不変 Scope→不変Scope、9 章→9章 など）は詰める。
  if prefix.include?('#') && (m = rest.match(/\A(?:\d[\d.\-]*\.?|前編|後編|付録)[ \t]+/))
    prefix += m[0]
    rest = m.post_match
  end
  prefix + tighten_content(rest)
end

# ファイル全体を行ごとに処理。frontmatter とコードフェンスはまるごと保護。
def tighten_document(src)
  lines = src.split("\n", -1)
  in_front = lines.first == '---'
  in_fence = false
  fence = nil

  lines.each_with_index.map do |line, idx|
    if in_front
      in_front = false if idx.positive? && line == '---' # 先頭以外の最初の --- で閉じる
      next line
    end
    if in_fence
      in_fence = false if line.lstrip.start_with?(fence)
      next line
    end
    if (m = line.lstrip.match(/\A(```+|~~~+)/))
      in_fence = true
      fence = m[1] # 開きフェンスの種類・長さをそのまま保持（閉じは同種で同長以上）
      next line
    end
    tighten_line(line)
  end.join("\n")
end

def collect_targets(paths)
  files = []
  paths.each do |p|
    if File.directory?(p)
      Find.find(p) { |q| files << q if q.end_with?('.md') }
    elsif File.file?(p)
      files << p
    else
      warn "skip (not found): #{p}"
    end
  end
  files.sort.uniq
end

mode = ARGV.first
paths = ARGV[1..] || []
unless %w[--check --write].include?(mode) && !paths.empty?
  warn 'usage: ruby tools/ja_format.rb (--check|--write) PATH...'
  exit 2
end

changed = []
collect_targets(paths).each do |file|
  src = File.read(file, encoding: 'UTF-8')
  dst = tighten_document(src)
  next if dst == src

  changed << file
  diff_lines = src.split("\n", -1).zip(dst.split("\n", -1))
                  .each_with_index
                  .select { |(a, b), _| a != b }
  if mode == '--check'
    puts "~ #{file}"
    diff_lines.first(6).each do |(a, b), n|
      puts "  L#{n + 1}- #{a}"
      puts "  L#{n + 1}+ #{b}"
    end
    puts "  …（ほか #{diff_lines.size - 6} 行）" if diff_lines.size > 6
  else
    File.write(file, dst, encoding: 'UTF-8')
    puts "wrote #{file} (#{diff_lines.size} 行)"
  end
end

if changed.empty?
  puts 'OK: 和欧境界の詰めは不要（すべて整形済み）。'
  exit 0
end

if mode == '--check'
  warn "NG: #{changed.size} ファイルに未整形の和欧境界スペースがあります（--write で詰めます）。"
  exit 1
end
puts "done: #{changed.size} ファイルを整形しました。"
