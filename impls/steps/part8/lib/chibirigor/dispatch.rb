# frozen_string_literal: true

module Chibirigor
  # Method-send typing. Part 2's hand-written table is replaced by one derived from RBS.
  module Dispatch
    METHODS = Rbs.load(Rbs::CORE)

    module_function

    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      signature = METHODS[[class_of(receiver_type), name]]
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

      signature[:returns]
    end
  end
end
