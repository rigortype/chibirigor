# frozen_string_literal: true

module Chibirigor
  # Acceptance check: answer "does actual fit where expected is wanted?" in three values.
  # :yes (definitely fits) / :no (definitely doesn't) / :maybe (unknown).
  module Accepts
    module_function

    def call(expected, actual)
      # If untyped is involved, don't commit either way (the core of gradual typing).
      return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)

      # actual is a Union: :yes only if every member passes. Take the weakest conclusion.
      return weakest(actual.members.map { |member| call(expected, member) }) if actual.is_a?(Type::Union)

      # expected is a Union: fitting any one member is enough. Take the strongest conclusion.
      return strongest(expected.members.map { |member| call(member, actual) }) if expected.is_a?(Type::Union)

      # Naive subtyping: do the classes match? (class_of rounds a Const to its class.)
      Dispatch.class_of(expected) == Dispatch.class_of(actual) ? :yes : :no
    end

    # :no if any :no, else :maybe if any :maybe, else :yes.
    def weakest(results)
      return :no if results.include?(:no)
      return :maybe if results.include?(:maybe)

      :yes
    end

    # :yes if any :yes, else :maybe if any :maybe, else :no.
    def strongest(results)
      return :yes if results.include?(:yes)
      return :maybe if results.include?(:maybe)

      :no
    end
  end
end
