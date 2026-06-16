# frozen_string_literal: true

require 'prism'

module Chibirigor
  # Narrow a variable's type per conditional branch (narrowing).
  # A condition that can't be narrowed returns the scope unchanged (claim nothing = don't frighten).
  module Narrowing
    module_function

    def narrow(scope, cond, truthy)
      return scope unless cond.is_a?(Prism::CallNode)

      receiver = cond.receiver
      return scope unless receiver.is_a?(Prism::LocalVariableReadNode)

      current = scope.local(receiver.name)
      return scope if current.nil?

      narrowed = narrow_type(current, cond, truthy)
      narrowed ? scope.with_local(receiver.name, narrowed) : scope
    end

    def narrow_type(current, cond, truthy)
      case cond.name
      when :nil?
        truthy ? Type::Nominal[:NilClass] : remove_nil(current)
      when :is_a?, :kind_of?, :instance_of?
        klass = class_argument(cond)
        klass && truthy && possible?(current, klass) ? Type::Nominal[klass] : nil
      end
    end

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
