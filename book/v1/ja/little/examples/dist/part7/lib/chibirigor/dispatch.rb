# frozen_string_literal: true

module Chibirigor
  # メソッド送信の型付け。手書きのディスパッチ表＋三値受理判定（Accepts）。
  module Dispatch
    I = Type::Nominal[:Integer]
    S = Type::Nominal[:String]

    METHODS = {
      %i[Integer +]      => { params: [I], returns: I },
      %i[Integer -]      => { params: [I], returns: I },
      %i[Integer *]      => { params: [I], returns: I },
      %i[Integer to_s]   => { params: [], returns: S },
      %i[String +]       => { params: [S], returns: S },
      %i[String *]       => { params: [I], returns: S },
      %i[String length]  => { params: [], returns: I },
      %i[String upcase]  => { params: [], returns: S }
    }.freeze

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
          node, "#{name} の引数の数が違います（#{signature[:params].size} 個必要、#{arg_types.size} 個渡された）"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        next unless Accepts.call(param, arg) == :no

        diagnostics << Chibirigor.diagnostic(node, "#{param} が必要ですが #{arg} が渡されました")
      end

      signature[:returns]
    end
  end
end
