# frozen_string_literal: true

require 'prism'

module Chibirigor
  # 条件分岐の枝ごとに、変数の型を絞る（ナローイング）。
  # 絞れない条件はスコープをそのまま返す（何も主張しない＝脅かさない）。
  module Narrowing
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

      members = current.is_a?(Type::Union) ? current.members : [current]
      members.any? { |member| Dispatch.class_of(member) == klass }
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
