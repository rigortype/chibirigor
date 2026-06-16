# frozen_string_literal: true

module Chibirigor
  # The type of "this exact value." e.g. Const[1], Const["hi"]
  Const = Data.define(:value) do
    def to_s = value.inspect
  end

  # A named class. e.g. Nominal[:Integer] (used by the "rounding" in 1-2)
  Nominal = Data.define(:name) do
    def to_s = name.to_s
  end

  # "Unknown / unknowable" (this becomes the linchpin later)
  Dynamic = Data.define do
    def to_s = 'untyped'
  end
end
