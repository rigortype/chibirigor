# frozen_string_literal: true

module Chibirigor
  # A mapping of variable name (symbol) -> type = the type environment.
  # Immutable: adding a binding returns a "new" Scope and leaves the original unchanged.
  class Scope
    def initialize(locals = {})
      @locals = locals.freeze
    end

    # The type for that name (nil if unbound).
    def local(name)
      @locals[name]
    end

    # A new scope with one binding added.
    def with_local(name, type)
      Scope.new(@locals.merge(name => type))
    end
  end
end
