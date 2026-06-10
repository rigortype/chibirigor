# frozen_string_literal: true

module Chibirigor
  # メソッド送信の型付け。Ruby は何でもメソッド送信なので、
  # 「どのクラスのどのメソッドが、どんな引数を取り、何を返すか」を表で持つ。
  # Part 7 でこの表を手書きから RBS 由来（Rbs.load）に差し替えた。
  module Dispatch
    # [レシーバのクラス, メソッド名] => { params: [引数の型...], returns: 戻り型 }
    METHODS = Rbs.load(Rbs::CORE)

    # 畳める演算：両オペランドが「既知値の Const」のとき、実際に計算して Const に畳む。
    FOLD = {
      %i[Integer +] => ->(a, b) { a + b },
      %i[Integer -] => ->(a, b) { a - b },
      %i[Integer *] => ->(a, b) { a * b },
      %i[String +] => ->(a, b) { a + b },
      %i[String *] => ->(a, b) { a * b } # "a" * 3 → "aaa"
    }.freeze

    # Const 爆発を防ぐ広げ規則（正規化予算の最小版）。これを超えたら畳まず丸める。
    INT_LIMIT = 1_000_000
    STR_LIMIT = 100

    # ナローイングが特別扱いする述語（戻りは概念上 bool）。chibirigor は bool 型を持たないので
    # Dynamic を返すが、これは「型を見失った」のではなく「述語をモデル化していない」だけ。
    # よって fail-soft 地図（check --explain）には載せない（誤った沈黙地点を出さない）。
    GUARD_PREDICATES = %i[nil? is_a? kind_of? instance_of?].freeze

    module_function

    # 型を「クラス名（シンボル）」に丸める。表のキー照合に使う。
    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      end # Dynamic などは nil（＝ディスパッチできない）
    end

    # 畳めるなら畳んだ Const を、畳めない/丸めるべきなら nil を返す。
    def foldable_result(receiver_type, name, arg_types)
      op = FOLD[[class_of(receiver_type), name]]
      return nil unless op
      return nil unless receiver_type.is_a?(Type::Const) && arg_types.all?(Type::Const)

      result = op.call(receiver_type.value, *arg_types.map(&:value))
      widen?(result) ? nil : Type::Const[result]
    rescue StandardError
      nil # 計算が失敗するなら畳まない（型エラーは別途診断され、戻りは丸めに委ねる）
    end

    # 大きすぎる値は畳まない（予算超過 → 丸める）
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
      unless signature # 知らないメソッド → 脅かさない（ここが fail-soft 地点）
        # untyped に倒した地点を provenance として記録（check --explain が地図化する）。
        # 通常の check はこの :fail_soft を捨てるので、診断は増えない（挙動不変）。
        # ナローイングの述語（is_a? 等）は型を見失ったわけではないので地図に載せない。
        unless GUARD_PREDICATES.include?(name)
          diagnostics << Chibirigor.diagnostic(node, "ここで untyped に倒しました（`#{name}` の型が引けません）")
                                   .merge(kind: :fail_soft, severity: :info)
        end
        return Type::Dynamic.new
      end

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

      # 畳めれば畳む（Const）、無理なら表の戻り型に丸める。
      foldable_result(receiver_type, name, arg_types) || signature[:returns]
    end
  end
end
