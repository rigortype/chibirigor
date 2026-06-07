# frozen_string_literal: true

module Chibirigor
  # メソッド送信の型付け。Ruby は何でもメソッド送信なので、
  # 「どのクラスのどのメソッドが、どんな引数を取り、何を返すか」を表で持つ。
  # Part 7 でこの表を手書きから RBS 由来（Rbs.load）に差し替えた。
  module Dispatch
    # [レシーバのクラス, メソッド名] => { params: [引数の型...], returns: 戻り型 }
    METHODS = Rbs.load(Rbs::CORE)

    module_function

    # 型を「クラス名（シンボル）」に丸める。表のキー照合に使う。
    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end # Dynamic などは nil（＝ディスパッチできない）
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
        # 怒るのは :no（確実に合わない）のときだけ。:yes も :maybe も黙る。
        next unless Accepts.call(param, arg) == :no

        diagnostics << Chibirigor.diagnostic(node, "#{param} が必要ですが #{arg} が渡されました")
      end

      signature[:returns]
    end
  end
end
