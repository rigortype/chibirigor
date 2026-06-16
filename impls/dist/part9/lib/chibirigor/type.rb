# frozen_string_literal: true

module Chibirigor
  # Type carriers (data that represents a type).
  module Type
    Const     = Data.define(:value) { def to_s = value.inspect }
    Nominal   = Data.define(:name)  { def to_s = name.to_s }
    Dynamic   = Data.define         { def to_s = 'untyped' }
    Union     = Data.define(:members) { def to_s = members.map(&:to_s).join(' | ') }
    HashShape = Data.define(:fields) { def to_s = "{#{fields.map { |key, type| "#{key}: #{type}" }.join(', ')}}" }
    Tuple     = Data.define(:elements) { def to_s = "[#{elements.map(&:to_s).join(', ')}]" }

    module_function

    # Fold an array of types into one. Flatten nested Unions and drop duplicates.
    def union(types)
      flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
      return Dynamic.new if flat.empty?
      return Dynamic.new if flat.any?(Dynamic) # any untyped makes the whole thing untyped (gradual)
      return flat.first if flat.size == 1

      Union[flat.freeze]
    end

    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
