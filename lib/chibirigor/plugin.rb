# frozen_string_literal: true

module Chibirigor
  # A minimal plugin hook. A single extension point that adds type info from
  # outside, without editing Dispatch::METHODS.
  # A hands-on demo of real Rigor's ADR-2 (the extension API).
  #
  # Usage:
  #   Chibirigor.register_method(:MyClass, :my_method,
  #     params:  [Type::Nominal[:Integer]],
  #     returns: Type::Nominal[:String])
  module Plugin
    @registry = {}

    module_function

    def register_method(klass, name, params:, returns:)
      @registry[[klass, name]] = { params: Array(params), returns: returns }
    end

    def registry = @registry

    def reset! = @registry.clear
  end

  def self.register_method(klass, name, params:, returns:)
    Plugin.register_method(klass, name, params: params, returns: returns)
  end
end
