# frozen_string_literal: true

module Chibirigor
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = 'untyped' }
    Union   = Data.define(:members) { def to_s = members.map(&:to_s).join(' | ') }

    # The shape of a hash: remembers a type per key. e.g. {foo: 1, bar: "a"}
    HashShape = Data.define(:fields) do
      def to_s = "{#{fields.map { |key, type| "#{key}: #{type}" }.join(', ')}}"
    end

    # An array remembered "type per position." e.g. [1, "a"]
    Tuple = Data.define(:elements) do
      def to_s = "[#{elements.map(&:to_s).join(', ')}]"
    end

    module_function

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
