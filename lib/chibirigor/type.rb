# frozen_string_literal: true

module Chibirigor
  # 型キャリア（型を表すデータ）。Part 1 ではこの 3 つだけ。
  module Type
    # 「この値そのもの」を表す型。例: Const[1], Const["hi"]
    Const = Data.define(:value) do
      def to_s = value.inspect
    end

    # 名前付きクラスを表す型。例: Nominal[:Integer]
    Nominal = Data.define(:name) do
      def to_s = name.to_s
    end

    # 「知らない・確かめようがない」を表す型（gradual の要）
    Dynamic = Data.define do
      def to_s = 'untyped'
    end

    # 型が一本に決まらないとき。例: Integer | String
    Union = Data.define(:members) do
      def to_s = members.map(&:to_s).join(' | ')
    end

    # ハッシュの構造。キーごとの型を覚える。例: {foo: 1, bar: "a"}
    HashShape = Data.define(:fields) do
      def to_s = "{#{fields.map { |key, type| "#{key}: #{type}" }.join(', ')}}"
    end

    # 配列を「位置ごとの型」で覚える。例: [1, "a"]
    Tuple = Data.define(:elements) do
      def to_s = "[#{elements.map(&:to_s).join(', ')}]"
    end

    module_function

    # 型の配列を 1 つの型にまとめる。入れ子の Union をならし、重複を消す。
    def union(types)
      flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
      return Dynamic.new if flat.empty? # 空＝行き止まり。安全側に倒す
      return Dynamic.new if flat.any?(Dynamic) # untyped が混じれば全体 untyped（gradual）
      return flat.first if flat.size == 1

      Union[flat.freeze]
    end

    # 整数とみなせる型か（Const[1] も Nominal[:Integer] も真）
    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
