# frozen_string_literal: true

module Chibirigor
  # A tiny plugin hook. A single extension point for adding type info from outside, without editing Dispatch::METHODS.
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
