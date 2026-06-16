# frozen_string_literal: true

module Chibirigor
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = 'untyped' }

    # When the type doesn't collapse to one. e.g. Integer | String
    Union = Data.define(:members) do
      def to_s = members.map(&:to_s).join(' | ')
    end

    module_function

    # Fold an array of types into one. Flatten nested Unions and drop duplicates.
    def union(types)
      flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
      return Dynamic.new if flat.empty?
      return flat.first if flat.size == 1

      Union[flat.freeze]
    end

    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
