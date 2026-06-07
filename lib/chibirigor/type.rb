# frozen_string_literal: true

module Chibirigor
  # 型カリア（型を表すデータ）。Part 1 ではこの 3 つだけ。
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

    module_function

    # 整数とみなせる型か（Const[1] も Nominal[:Integer] も真）
    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
