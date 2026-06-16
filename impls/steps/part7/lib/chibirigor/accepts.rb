# frozen_string_literal: true

module Chibirigor
  # Acceptance check: answers "does passing actual where expected is wanted fit?" with three values.
  # :yes (definitely fits) / :no (definitely doesn't) / :maybe (unknown).
  module Accepts
    module_function

    def call(expected, actual)
      return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)

      return weakest(actual.members.map { |member| call(expected, member) }) if actual.is_a?(Type::Union)

      return strongest(expected.members.map { |member| call(member, actual) }) if expected.is_a?(Type::Union)

      Dispatch.class_of(expected) == Dispatch.class_of(actual) ? :yes : :no
    end

    def weakest(results)
      return :no if results.include?(:no)
      return :maybe if results.include?(:maybe)

      :yes
    end

    def strongest(results)
      return :yes if results.include?(:yes)
      return :maybe if results.include?(:maybe)

      :no
    end
  end
end
