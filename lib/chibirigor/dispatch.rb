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
    MEMBER_LIMIT = 4 # Union のメンバ数予算。超えたら Const をクラスに丸める

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
      when Type::Generic then type.name # Array[Integer] → :Array
      end # Dynamic などは nil（＝ディスパッチできない）
    end

    # 畳めるなら畳んだ Const（引数に Union があればメンバごとに畳んだ Union）を、
    # 畳めない/丸めるべきなら nil を返す。
    # 例: 1 + (1 | 2) → 2 | 3。組み合わせが MEMBER_LIMIT を超えたら畳まず丸める。
    def foldable_result(receiver_type, name, arg_types)
      op = FOLD[[class_of(receiver_type), name]]
      return nil unless op && receiver_type.is_a?(Type::Const)

      combinations = const_combinations(arg_types)
      return nil if combinations.nil? || combinations.size > MEMBER_LIMIT

      members = combinations.map do |args|
        result = begin
          op.call(receiver_type.value, *args.map(&:value))
        rescue StandardError
          return nil # 計算が失敗する組があれば畳まない（型エラーは別途診断され、戻りは丸めに委ねる）
        end
        return nil if widen?(result)

        Type::Const[result]
      end
      Type.union(members)
    end

    # 各引数を「Const のメンバ列」に展開して直積を返す。Const 以外が混じれば nil（畳めない）。
    def const_combinations(arg_types)
      member_lists = arg_types.map do |type|
        members = type.is_a?(Type::Union) ? type.members : [type]
        return nil unless members.all?(Type::Const)

        members
      end
      member_lists.reduce([[]]) { |acc, members| acc.product(members).map { |combo, m| combo + [m] } }
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
      # Union レシーバはメンバごとにディスパッチして結果をまとめる（分配）。
      return dispatch_union(receiver_type, name, arg_types, node, diagnostics) if receiver_type.is_a?(Type::Union)

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

    # Union レシーバの分配ディスパッチ。実行時はどのメンバにもなり得るので、
    # エラーは「全メンバで失敗した」ときだけ表に出す（一部の失敗は :maybe ＝黙る）。
    # :info（fail-soft 地図）は provenance なのでそのまま通す（重複だけ消す）。
    def dispatch_union(receiver_type, name, arg_types, node, diagnostics)
      buffers = []
      results = receiver_type.members.map do |member|
        buffers << (buffer = [])
        dispatch(member, name, arg_types, node, buffer)
      end
      diagnostics.concat(merge_member_diagnostics(buffers))
      budgeted_union(results)
    end

    def merge_member_diagnostics(buffers)
      infos, errors = buffers.flatten.partition { |d| d[:severity] == :info }
      merged = infos.uniq
      merged.concat(errors.uniq) if buffers.all? { |b| b.any? { |d| d[:severity] != :info } }
      merged
    end

    # Union のメンバ数予算。MEMBER_LIMIT を超えたら Const をクラスに丸めて作り直す
    # （widen? と同じ「正規化予算」パターンの Union 版）。
    def budgeted_union(types)
      result = Type.union(types)
      return result unless result.is_a?(Type::Union) && result.members.size > MEMBER_LIMIT

      Type.union(result.members.map { |m| m.is_a?(Type::Const) ? Type::Nominal[class_of(m)] : m })
    end
  end
end
