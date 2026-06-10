# frozen_string_literal: true

require 'prism'

module Chibirigor
  # 条件分岐の枝ごとに、変数の型を絞る（ナローイング）。
  # 絞れない条件はスコープをそのまま返す（何も主張しない＝脅かさない）。
  module Narrowing
    # 互いに素だと確実に言える具象クラス（葉）。is_a? の到達不能判定は、
    # この集合の中で*異なる*クラス同士のときだけ「起き得ない」と断言する。
    # Numeric/Object のような上位クラスは祖先表を持たないので断言しない（FP 回避）。
    DISJOINT_LEAVES = %i[Integer Float String Symbol NilClass TrueClass FalseClass Array Hash].freeze

    module_function

    # cond が真（truthy=true）／偽（false）になった枝のスコープを返す。
    def narrow(scope, cond, truthy)
      return scope unless cond.is_a?(Prism::CallNode)

      receiver = cond.receiver
      return scope unless receiver.is_a?(Prism::LocalVariableReadNode)

      current = scope.local(receiver.name)
      return scope if current.nil?

      narrowed = narrow_type(current, cond, truthy)
      narrowed ? scope.with_local(receiver.name, narrowed) : scope
    end

    # 絞れたら新しい型を、絞れなければ nil を返す。
    def narrow_type(current, cond, truthy)
      case cond.name
      when :nil?
        truthy ? Type::Nominal[:NilClass] : remove_nil(current)
      when :is_a?, :kind_of?, :instance_of?
        klass = class_argument(cond)
        # 真の枝だけ絞る。偽の枝は保守的に触らない（FP 安全）。
        # しかも「そのクラスがあり得るとき」だけ絞る。Integer を String に
        # 無理に絞ると、起き得ない枝（dead branch）を型付けして誤検知になる。
        # Dynamic も絞らない（Rigor と同じく post-guard narrowing は FP 過多）。
        klass && truthy && possible?(current, klass) ? Type::Nominal[klass] : nil
      end
    end

    # current 型で klass があり得るか（Union のメンバにそのクラスがあるか）。
    def possible?(current, klass)
      return false if current.is_a?(Type::Dynamic)

      members(current).any? { |member| Dispatch.class_of(member) == klass }
    end

    # その枝が*証明可能に*到達不能か（条件が必ず偽 truthy=true／必ず真 truthy=false）。
    # 真偽を断言できるのは「閉じた既知型（untyped を含まない）」のときだけ。
    # 少しでも分からなければ false＝報告しない（誤検知ゼロ＝動くコードを脅かさない）。
    def unreachable_branch?(scope, cond, truthy)
      return false unless cond.is_a?(Prism::CallNode)

      receiver = cond.receiver
      return false unless receiver.is_a?(Prism::LocalVariableReadNode)

      current = scope.local(receiver.name)
      return false if current.nil? || !closed?(current)

      case cond.name
      when :nil?
        # 真の枝＝nil でないと不可能／偽の枝＝nil でしか不可能。
        truthy ? members(current).none? { |m| nil_type?(m) } : members(current).all? { |m| nil_type?(m) }
      when :is_a?, :kind_of?, :instance_of?
        unreachable_is_a?(current, class_argument(cond), truthy)
      else
        false
      end
    end

    # is_a? の枝が証明可能に空か。真の枝＝全メンバが klass と互いに素な葉／
    # 偽の枝＝全メンバがちょうど klass（条件が恒真）。
    def unreachable_is_a?(current, klass, truthy)
      return false unless klass

      if truthy
        leaf?(klass) && members(current).all? { |m| c = Dispatch.class_of(m); leaf?(c) && c != klass }
      else
        members(current).all? { |m| Dispatch.class_of(m) == klass }
      end
    end

    # untyped を一切含まない閉じた型か（含むと白黒つけられない＝断言しない）。
    def closed?(type)
      return false if type.is_a?(Type::Dynamic)

      members(type).none? { |m| m.is_a?(Type::Dynamic) }
    end

    def members(type)
      type.is_a?(Type::Union) ? type.members : [type]
    end

    def leaf?(klass)
      DISJOINT_LEAVES.include?(klass)
    end

    def remove_nil(type)
      return type unless type.is_a?(Type::Union)

      Type.union(type.members.reject { |member| nil_type?(member) })
    end

    def nil_type?(type)
      Dispatch.class_of(type) == :NilClass
    end

    def class_argument(cond)
      arg = cond.arguments&.arguments&.first
      arg.is_a?(Prism::ConstantReadNode) ? arg.name : nil
    end
  end
end
