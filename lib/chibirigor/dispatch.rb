# frozen_string_literal: true

module Chibirigor
  # メソッド送信の型付け。Ruby は何でもメソッド送信なので、
  # 「どのクラスのどのメソッドが、どんな引数を取り、何を返すか」を手書きの表で持つ。
  module Dispatch
    I = Type::Nominal[:Integer]
    S = Type::Nominal[:String]

    # [レシーバのクラス, メソッド名] => { params: [引数の型...], returns: 戻り型 }
    METHODS = {
      %i[Integer +] => { params: [I], returns: I },
      %i[Integer -] => { params: [I], returns: I },
      %i[Integer *] => { params: [I], returns: I },
      %i[Integer to_s] => { params: [], returns: S },
      %i[String +] => { params: [S], returns: S },
      %i[String *] => { params: [I], returns: S },
      %i[String length] => { params: [], returns: I },
      %i[String upcase] => { params: [], returns: S }
    }.freeze

    module_function

    # 型を「クラス名（シンボル）」に丸める。表のキー照合に使う。
    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end # Dynamic などは nil（＝ディスパッチできない）
    end

    # 引数の型が仮引数の型に合うかの、手書きの判定。
    # （Part 6 でこれを三値の accepts に置き換える。いまは素朴なクラス一致。）
    def matches?(param, arg)
      return true if param.is_a?(Type::Dynamic) || arg.is_a?(Type::Dynamic)

      class_of(param) == class_of(arg)
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      signature = METHODS[[class_of(receiver_type), name]]
      return Type::Dynamic.new unless signature # 知らないメソッド → 脅かさない

      if arg_types.size != signature[:params].size
        diagnostics << Chibirigor.diagnostic(
          node, "#{name} の引数の数が違います（#{signature[:params].size} 個必要、#{arg_types.size} 個渡された）"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        next if matches?(param, arg)

        diagnostics << Chibirigor.diagnostic(node, "#{param} が必要ですが #{arg} が渡されました")
      end

      signature[:returns]
    end
  end
end
