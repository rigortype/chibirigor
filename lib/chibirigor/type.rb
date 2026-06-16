# frozen_string_literal: true

module Chibirigor
  # Type carriers (data that represents a type). Part 1 has only these three.
  module Type
    # The type of "this exact value." e.g. Const[1], Const["hi"]
    Const = Data.define(:value) do
      def to_s = value.inspect
    end

    # A named class. e.g. Nominal[:Integer]
    Nominal = Data.define(:name) do
      def to_s = name.to_s
    end

    # "Unknown / unknowable" — the linchpin of gradual typing.
    Dynamic = Data.define do
      def to_s = 'untyped'
    end

    # When the type doesn't collapse to one. e.g. Integer | String
    Union = Data.define(:members) do
      def to_s = members.map(&:to_s).join(' | ')
    end

    # The shape of a hash: remembers a type per key. e.g. {foo: 1, bar: "a"}
    HashShape = Data.define(:fields) do
      def to_s = "{#{fields.map { |key, type| "#{key}: #{type}" }.join(', ')}}"
    end

    # An array remembered "type per position." e.g. [1, "a"]
    Tuple = Data.define(:elements) do
      def to_s = "[#{elements.map(&:to_s).join(', ')}]"
    end

    # A generic type with type arguments. e.g. Array[Integer] (an array whose
    # element type Elem is rounded to one). We forget the positions and just say
    # "the element is Elem" (the fallback from generics 5b/5c).
    Generic = Data.define(:name, :args) do
      def to_s = "#{name}[#{args.map(&:to_s).join(', ')}]"
    end

    module_function

    # Fold an array of types into one. Flatten nested Unions and drop duplicates.
    def union(types)
      flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
      return Dynamic.new if flat.empty? # empty = dead end; fall back to the safe side
      return Dynamic.new if flat.any?(Dynamic) # any untyped makes the whole thing untyped (gradual)
      return flat.first if flat.size == 1

      Union[flat.freeze]
    end

    # Whether the type counts as an integer (true for both Const[1] and Nominal[:Integer]).
    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
