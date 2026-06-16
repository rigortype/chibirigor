# frozen_string_literal: true

module Chibirigor
  # Method-send typing. An RBS-derived table plus constant folding plus a plugin extension.
  # In Part 7 this table was swapped from hand-written to RBS-derived (Rbs.load).
  module Dispatch
    METHODS = Rbs.load(Rbs::CORE)

    FOLD = {
      %i[Integer +] => ->(a, b) { a + b },
      %i[Integer -] => ->(a, b) { a - b },
      %i[Integer *] => ->(a, b) { a * b },
      %i[String +]  => ->(a, b) { a + b },
      %i[String *]  => ->(a, b) { a * b }
    }.freeze

    INT_LIMIT = 1_000_000
    STR_LIMIT = 100

    module_function

    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end
    end

    def foldable_result(receiver_type, name, arg_types)
      op = FOLD[[class_of(receiver_type), name]]
      return nil unless op
      return nil unless receiver_type.is_a?(Type::Const) && arg_types.all?(Type::Const)

      result = op.call(receiver_type.value, *arg_types.map(&:value))
      widen?(result) ? nil : Type::Const[result]
    rescue StandardError
      nil
    end

    def widen?(value)
      case value
      when Integer then value.abs > INT_LIMIT
      when String  then value.length > STR_LIMIT
      else true
      end
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      key = [class_of(receiver_type), name]
      signature = Plugin.registry[key] || METHODS[key]
      return Type::Dynamic.new unless signature

      if arg_types.size != signature[:params].size
        diagnostics << Chibirigor.diagnostic(
          node, "wrong number of arguments for #{name} (#{signature[:params].size} expected, #{arg_types.size} given)"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        next unless Accepts.call(param, arg) == :no

        diagnostics << Chibirigor.diagnostic(node, "expected #{param} but got #{arg}")
      end

      foldable_result(receiver_type, name, arg_types) || signature[:returns]
    end
  end
end
