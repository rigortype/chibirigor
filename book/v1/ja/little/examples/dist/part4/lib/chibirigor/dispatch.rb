# frozen_string_literal: true

module Chibirigor
  # メソッド送信の型付けを「手書きのディスパッチ表」に委ねる（Part 2 新設）。
  module Dispatch
    I = Type::Nominal[:Integer]
    S = Type::Nominal[:String]

    # [レシーバのクラス, メソッド名] => { params: [引数の型...], returns: 戻り型 }
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
      end # Dynamic などは nil（＝引けない）
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      signature = METHODS[[class_of(receiver_type), name]]
      return Type::Dynamic.new unless signature # 知らないメソッド → 脅かさない（2-5）

      if arg_types.size != signature[:params].size
        diagnostics << Chibirigor.diagnostic(
          node,
          "#{name} の引数の数が違います（#{signature[:params].size} 個必要、#{arg_types.size} 個渡された）"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        next if matches?(param, arg)

        diagnostics << Chibirigor.diagnostic(node, "#{param} が必要ですが #{arg} が渡されました")
      end

      signature[:returns]
    end

    def matches?(param, arg)
      return true if param.is_a?(Type::Dynamic) || arg.is_a?(Type::Dynamic) # 不明は通す

      class_of(param) == class_of(arg)
    end
  end
end
