# frozen_string_literal: true

module Chibirigor
  # 極小プラグインフック。Dispatch::METHODS を編集せず、外から型情報を足す一点拡張。
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
