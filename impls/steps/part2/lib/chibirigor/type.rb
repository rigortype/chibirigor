# frozen_string_literal: true

module Chibirigor
  # 2-0 で型キャリアを `Type::` 名前空間にまとめ直す（Part 1 の素朴な定義を置換）。
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = 'untyped' }
  end
end
