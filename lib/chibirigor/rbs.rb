# frozen_string_literal: true

module Chibirigor
  # ごく小さな RBS 読み込み。本物の rbs gem の代わりに、最小限を自前で読む。
  # 「型は別ファイル（RBS）に書く」という Ruby/RBS の世界観の縮図。
  # Part 2 の手書き表を、この RBS テキストから生成した表に差し替える。
  module Rbs
    module_function

    CLASS_LINE = /\A\s*class\s+(\S+)\s*\z/
    DEF_LINE   = /\A\s*def\s+(\S+):\s*\((.*)\)\s*->\s*(\S+)\s*\z/

    # コア型のシグネチャ。Part 2 の手書き表と同じ内容（差し替えても診断は変わらない）。
    CORE = <<~RBS
      class Integer
        def +: (Integer) -> Integer
        def -: (Integer) -> Integer
        def *: (Integer) -> Integer
        def to_s: () -> String
      end
      class String
        def +: (String) -> String
        def *: (Integer) -> String
        def length: () -> Integer
        def upcase: () -> String
      end
    RBS

    # RBS テキストを [クラス, メソッド] => { params:, returns: } の表にする。
    def load(source)
      table = {}
      current = nil
      source.each_line do |line|
        if (m = CLASS_LINE.match(line))
          current = m[1].to_sym
        elsif current && (m = DEF_LINE.match(line))
          params = m[2].split(',').map(&:strip).reject(&:empty?).map { |t| Type::Nominal[t.to_sym] }
          table[[current, m[1].to_sym]] = { params: params.freeze, returns: Type::Nominal[m[3].to_sym] }
        end
      end
      table.freeze
    end
  end
end
