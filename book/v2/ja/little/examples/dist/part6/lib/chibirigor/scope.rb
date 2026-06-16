# frozen_string_literal: true

module Chibirigor
  # A variable name (symbol) → type mapping = the type environment.
  # Immutable: adding a binding returns a "new" Scope and leaves the original unchanged.
  class Scope
    def initialize(locals = {})
      @locals = locals.freeze
    end

    def local(name)
      @locals[name]
    end

    def with_local(name, type)
      Scope.new(@locals.merge(name => type))
    end
  end
end
