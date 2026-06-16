# frozen_string_literal: true

module Chibirigor
  # Delegate method-send typing to a "hand-written dispatch table" (new in Part 2).
  module Dispatch
    I = Type::Nominal[:Integer]
    S = Type::Nominal[:String]

    # [receiver class, method name] => { params: [arg types...], returns: return type }
    METHODS = {
      %i[Integer +]      => { params: [I], returns: I },
      %i[Integer to_s]   => { params: [], returns: S },
      %i[String +]       => { params: [S], returns: S },
      %i[String length]  => { params: [], returns: I }
    }.freeze

    module_function

    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end # Dynamic etc. → nil (= can't look it up)
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      signature = METHODS[[class_of(receiver_type), name]]
      return Type::Dynamic.new unless signature # unknown method → don't frighten (2-5)

      if arg_types.size != signature[:params].size
        diagnostics << Chibirigor.diagnostic(
          node,
          "wrong number of arguments for #{name} (#{signature[:params].size} expected, #{arg_types.size} given)"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        next if matches?(param, arg)

        diagnostics << Chibirigor.diagnostic(node, "expected #{param} but got #{arg}")
      end

      signature[:returns]
    end

    def matches?(param, arg)
      return true if param.is_a?(Type::Dynamic) || arg.is_a?(Type::Dynamic) # unknown passes

      class_of(param) == class_of(arg)
    end
  end
end
