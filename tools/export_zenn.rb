#!/usr/bin/env ruby
# frozen_string_literal: true

# tools/export_zenn.rb
#
# book/v2/ja の前編（little）＋付録（appendix）＋用語集を、Zenn の本
# （the-little-chibirigor）へ書き出す。再実行可能なエクスポート手続き。
#
# 変換の要点：
#   - frontmatter を Zenn 章用（title だけ）に置換。本文先頭の H1 は落とす（title が見出しになる）。
#   - GitHub アラート（> [!NOTE] 等）を Zenn 記法へ：
#       標題付き（先頭が単独の **題**）の囲みコラム → :::details 題
#       [!WARNING]/[!CAUTION]                      → :::message alert
#       それ以外（短い補足・要点）                 → :::message
#   - 図 ![..](../figures/svg/X.svg) → ![..](/images/the-little-chibirigor/X.png)、
#     直後の「> ▼ 図…」キャプションは Zenn の *…*（画像直下のイタリック）へ。SVG は inkscape で PNG 化。
#   - 章をまたぐ相対リンク（.md / 付録ディレクトリ等）は本文テキストへ落とす（Zenn 本の章間相対リンクは不可）。
#   - <!-- run: --> / <!-- include: --> の検査用ディレクティブは削除。
#   - 全角スペース（U+3000）は半角スペースへ（コードフェンス内は除く）。
#
# usage: ruby tools/export_zenn.rb

require 'fileutils'

ROOT     = File.expand_path('..', __dir__)
SRC      = File.join(ROOT, 'book/v2/ja')
FIG_SRC  = File.join(SRC, 'figures/svg')
ZBOOK    = '/Users/megurine/repo/site/zenn-book/books/the-little-chibirigor'
ZIMG_DIR = '/Users/megurine/repo/site/zenn-book/images/the-little-chibirigor'
IMG_BASE = '/images/the-little-chibirigor'

# [slug, 相対ソース, 種別(:about or nil), Zenn 章タイトルの上書き(nil=ソースの title を使う)]
CHAPTERS = [
  ['about',                       'README.md',                          :about, 'この本について'],
  ['part0-introduction',          'little/part0-introduction.md',          nil, nil],
  ['part1-literals-and-arithmetic','little/part1-literals-and-arithmetic.md', nil, nil],
  ['part2-method-dispatch',       'little/part2-method-dispatch.md',       nil, nil],
  ['part3-scope-and-statements',  'little/part3-scope-and-statements.md',  nil, nil],
  ['part4-union',                 'little/part4-union.md',                 nil, nil],
  ['part5-narrowing',             'little/part5-narrowing.md',             nil, nil],
  ['part6-hash-and-tuple',        'little/part6-hash-and-tuple.md',        nil, nil],
  ['part7-accepts-and-trinary',   'little/part7-accepts-and-trinary.md',   nil, nil],
  ['part8-rbs-and-signatures',    'little/part8-rbs-and-signatures.md',    nil, nil],
  ['part9-gradual-philosophy',    'little/part9-gradual-philosophy.md',    nil, nil],
  ['a1-special-types',            'appendix/a1-special-types.md',          nil, nil],
  ['a2-narrowing-patterns',       'appendix/a2-narrowing-patterns.md',     nil, nil],
  ['a3-tooling',                  'appendix/a3-tooling.md',                nil, nil],
  ['a4-bibliography',             'appendix/a4-bibliography.md',           nil, nil],
  ['a5-other-languages',          'appendix/a5-other-languages.md',        nil, nil],
  ['glossary',                    'glossary.md',                           nil, nil],
].freeze

ALERT_RE = /\A> \[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*\z/
FENCE_RE = /\A\s*(```|~~~)/

def split_frontmatter(text)
  lines = text.split("\n", -1)
  return [{}, text] unless lines.first == '---'
  endi = lines[1..].index('---')
  return [{}, text] unless endi
  fm = {}
  lines[1...(endi + 1)].each do |l|
    if (m = l.match(/\A([a-z_]+):\s*(.*)\z/))
      fm[m[1]] = m[2].sub(/\A"(.*)"\z/, '\1').sub(/\A'(.*)'\z/, '\1')
    end
  end
  [fm, lines[(endi + 2)..].join("\n")]
end

# 章リード・図キャプション以外の `>` 引用に紛れた GitHub アラートを Zenn 記法へ。
def convert_alerts(body)
  lines = body.split("\n", -1)
  out = []
  i = 0
  while i < lines.length
    line = lines[i]
    if (m = line.match(ALERT_RE))
      type = m[1]
      i += 1
      blk = []
      while i < lines.length && lines[i].start_with?('>')
        blk << lines[i].sub(/\A> ?/, '')
        i += 1
      end
      fnz = blk.find_index { |l| !l.strip.empty? }
      title = nil
      if fnz && (tm = blk[fnz].match(/\A\*\*(.+?)\*\*\z/))
        title = tm[1]
        blk.delete_at(fnz)
        blk.delete_at(fnz) if blk[fnz]&.strip&.empty?
      end
      blk.shift while blk.first && blk.first.strip.empty?
      blk.pop while blk.last && blk.last.strip.empty?
      header = if title
                 ":::details #{title}"
               elsif %w[WARNING CAUTION].include?(type)
                 ':::message alert'
               else
                 ':::message'
               end
      out << header
      out.concat(blk)
      out << ':::'
    else
      out << line
      i += 1
    end
  end
  out.join("\n")
end

# 図 svg→png＋キャプション、検査ディレクティブ削除。使った svg 名を svgs に集める。
def convert_figures_and_directives(body, svgs)
  lines = body.split("\n", -1)
  out = []
  fence = false
  i = 0
  while i < lines.length
    line = lines[i]
    if line =~ FENCE_RE
      fence = !fence
      out << line
      i += 1
      next
    end
    unless fence
      if line =~ /\A<!--\s*(run|include):/
        i += 1
        next
      end
      if (m = line.match(%r{\A!\[(.*?)\]\(\.\./figures/svg/([^)]+)\.svg\)\s*\z}))
        alt = m[1]
        name = m[2]
        svgs << name
        out << "![#{alt}](#{IMG_BASE}/#{name}.png)"
        if lines[i + 1] && (c = lines[i + 1].match(/\A> ▼ (.+?)\s*\z/))
          out << "*#{c[1]}*"
          i += 2
          next
        end
        i += 1
        next
      end
    end
    out << line
    i += 1
  end
  out.join("\n")
end

# 相対リンク（http でも /images でもない）を本文テキストへ。全角→半角（フェンス外）。
def tidy_text(body)
  lines = body.split("\n", -1)
  fence = false
  lines.map! do |line|
    if line =~ FENCE_RE
      fence = !fence
      next line
    end
    next line if fence
    # 画像でない [text](relative) を text に落とす
    line = line.gsub(/(?<!!)\[([^\]\n]+)\]\((?!https?:\/\/|\/images)[^)\n]+\)/, '\1')
    line.gsub("　", ' ')
  end
  lines.join("\n")
end

# 中身が空になった Zenn ブロック（:::message 直後に ::: 等）を取り除く。
def remove_empty_zenn_blocks(body)
  lines = body.split("\n", -1)
  out = []
  i = 0
  while i < lines.length
    if lines[i] =~ /\A:::(message|details)/ && lines[i + 1] == ':::'
      i += 2
      next
    end
    out << lines[i]
    i += 1
  end
  out.join("\n")
end

def drop_leading_h1(body)
  lines = body.split("\n", -1)
  idx = lines.find_index { |l| l.start_with?('# ') }
  if idx && lines[0...idx].all? { |l| l.strip.empty? }
    lines.delete_at(idx)
    lines.shift while lines.first && lines.first.strip.empty?
  end
  lines.join("\n")
end

# about 章だけ：目次セクション（## 目次 〜 次の ## まで）と repo 内部メモを落とす。
def about_cleanup(body)
  lines = body.split("\n", -1)
  out = []
  skip = false
  lines.each do |l|
    if l =~ /\A## 目次/
      skip = true
      next
    end
    skip = false if skip && l =~ /\A## / && l !~ /\A## 目次/
    next if skip
    next if l.include?('清書版') && l.include?('draft') # 原稿リポジトリの内部メモ
    out << l
  end
  out.join("\n")
end

def transform(text, kind, title_override)
  fm, body = split_frontmatter(text)
  title = title_override || fm['title'].to_s.gsub("　", ' ')
  body = about_cleanup(body) if kind == :about
  body = drop_leading_h1(body)
  body = convert_alerts(body)
  svgs = []
  body = convert_figures_and_directives(body, svgs)
  body = tidy_text(body)
  body = remove_empty_zenn_blocks(body)
  body = body.gsub(/\n{3,}/, "\n\n").strip
  out = "---\ntitle: \"#{title.gsub('"', '\\"')}\"\n---\n\n#{body}\n"
  [out, svgs]
end

def convert_svg(name)
  src = File.join(FIG_SRC, "#{name}.svg")
  dst = File.join(ZIMG_DIR, "#{name}.png")
  raise "missing svg: #{src}" unless File.exist?(src)

  ok = system('inkscape', src, '--export-type=png',
              "--export-filename=#{dst}", '--export-dpi=192',
              out: File::NULL, err: File::NULL)
  raise "inkscape failed for #{name}" unless ok && File.exist?(dst)

  dst
end

FileUtils.mkdir_p(ZIMG_DIR)
all_svgs = []
slugs = []
CHAPTERS.each do |slug, rel, kind, title_override|
  src = File.join(SRC, rel)
  out, svgs = transform(File.read(src, encoding: 'UTF-8'), kind, title_override)
  File.write(File.join(ZBOOK, "#{slug}.md"), out, encoding: 'UTF-8')
  all_svgs.concat(svgs)
  slugs << slug
  puts "chapter: #{slug}.md  (#{out.lines.size} lines, svg: #{svgs.join(',')})"
end

all_svgs.uniq.each { |n| convert_svg(n); puts "image:   #{n}.png" }

config = <<~YAML
  title: "最小のRuby型チェッカーを作りながら学ぶ（The Little chibirigor）"
  summary: "本物の漸進的型チェッカー Rigor を「小さく作り直して」学ぶ前編。数十〜数百行の Ruby で check / annotate が動く最小の型チェッカーを Part 0〜9 で作り切り、付録と用語集まで収めた一冊。"
  topics: ["ruby", "rbs", "型システム", "prism", "rigor"]
  published: false
  price: 0
  chapters:
  #{slugs.map { |s| "  - #{s}" }.join("\n")}
YAML
File.write(File.join(ZBOOK, 'config.yaml'), config, encoding: 'UTF-8')
puts "config:  config.yaml (#{slugs.size} chapters)"

# 雛形ファイルの掃除
%w[example1.md example2.md].each do |f|
  p = File.join(ZBOOK, f)
  File.delete(p) if File.exist?(p)
end
puts 'done.'
