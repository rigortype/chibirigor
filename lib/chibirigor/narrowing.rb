# frozen_string_literal: true

require 'prism'

module Chibirigor
  # Per branch of a conditional, narrow a variable's type (narrowing).
  # If the condition can't narrow, return the scope unchanged (claim nothing = don't frighten).
  module Narrowing
    # Concrete classes (leaves) we can be sure are pairwise disjoint. The is_a?
    # unreachability check only asserts "can't happen" for *distinct* classes within
    # this set. Superclasses like Numeric/Object have no ancestry table here, so we
    # don't assert on them (avoids false positives).
    DISJOINT_LEAVES = %i[Integer Float String Symbol NilClass TrueClass FalseClass Array Hash].freeze

    module_function

    # Return the scope for the branch where cond came out true (truthy=true) or false (false).
    def narrow(scope, cond, truthy)
      return scope unless cond.is_a?(Prism::CallNode)

      receiver = cond.receiver
      return scope unless receiver.is_a?(Prism::LocalVariableReadNode)

      current = scope.local(receiver.name)
      return scope if current.nil?

      narrowed = narrow_type(current, cond, truthy)
      narrowed ? scope.with_local(receiver.name, narrowed) : scope
    end

    # Return the narrowed type if we can narrow, otherwise nil.
    def narrow_type(current, cond, truthy)
      case cond.name
      when :nil?
        truthy ? Type::Nominal[:NilClass] : remove_nil(current)
      when :is_a?, :kind_of?, :instance_of?
        klass = class_argument(cond)
        # Narrow only the true branch. Leave the false branch conservatively untouched (FP-safe).
        # And only narrow when "that class is possible." Forcing Integer down to String
        # would type a dead branch and produce a false positive.
        # Don't narrow Dynamic either (like Rigor, post-guard narrowing on it is too FP-heavy).
        klass && truthy && possible?(current, klass) ? Type::Nominal[klass] : nil
      end
    end

    # Whether klass is possible under the current type (does a Union member have that class?).
    def possible?(current, klass)
      return false if current.is_a?(Type::Dynamic)

      members(current).any? { |member| Dispatch.class_of(member) == klass }
    end

    # Whether this branch is *provably* unreachable (condition always false when truthy=true /
    # always true when truthy=false). We can only assert truth or falsity for a
    # "closed known type" (one that contains no untyped).
    # If anything is uncertain, return false = don't report (zero false positives = don't frighten working code).
    def unreachable_branch?(scope, cond, truthy)
      return false unless cond.is_a?(Prism::CallNode)

      receiver = cond.receiver
      return false unless receiver.is_a?(Prism::LocalVariableReadNode)

      current = scope.local(receiver.name)
      return false if current.nil? || !closed?(current)

      case cond.name
      when :nil?
        # True branch = impossible unless non-nil / false branch = impossible unless nil.
        truthy ? members(current).none? { |m| nil_type?(m) } : members(current).all? { |m| nil_type?(m) }
      when :is_a?, :kind_of?, :instance_of?
        unreachable_is_a?(current, class_argument(cond), truthy)
      else
        false
      end
    end

    # Whether the is_a? branch is provably empty. True branch = every member is a leaf
    # disjoint from klass / false branch = every member is exactly klass (condition is a tautology).
    def unreachable_is_a?(current, klass, truthy)
      return false unless klass

      if truthy
        leaf?(klass) && members(current).all? { |m| c = Dispatch.class_of(m); leaf?(c) && c != klass }
      else
        members(current).all? { |m| Dispatch.class_of(m) == klass }
      end
    end

    # Whether the type is closed, containing no untyped at all (if it does, we can't decide either way = don't assert).
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
