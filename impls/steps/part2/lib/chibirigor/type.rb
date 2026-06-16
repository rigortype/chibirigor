# frozen_string_literal: true

module Chibirigor
  # 2-0 regroups the type carriers under the `Type::` namespace (replacing Part 1's naive definitions).
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = 'untyped' }
  end
end
