# frozen_string_literal: true

module Chibirigor
  # 型キャリア（型を表すデータ）。
  module Type
    Const     = Data.define(:value) { def to_s = value.inspect }
    Nominal   = Data.define(:name)  { def to_s = name.to_s }
    Dynamic   = Data.define         { def to_s = 'untyped' }
    Union     = Data.define(:members) { def to_s = members.map(&:to_s).join(' | ') }
    HashShape = Data.define(:fields) { def to_s = "{#{fields.map { |key, type| "#{key}: #{type}" }.join(', ')}}" }
    Tuple     = Data.define(:elements) { def to_s = "[#{elements.map(&:to_s).join(', ')}]" }

    module_function

    # 型の配列を 1 つの型にまとめる。入れ子の Union をならし、重複を消す。
    def union(types)
      flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
      return Dynamic.new if flat.empty?
      return Dynamic.new if flat.any?(Dynamic) # untyped が混じれば全体 untyped（gradual）
      return flat.first if flat.size == 1

      Union[flat.freeze]
    end

    def integerish?(type)
      (type.is_a?(Const) && type.value.is_a?(Integer)) || type == Nominal[:Integer]
    end
  end
end
