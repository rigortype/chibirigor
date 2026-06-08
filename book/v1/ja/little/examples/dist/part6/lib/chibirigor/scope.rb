# frozen_string_literal: true

module Chibirigor
  # 変数名（シンボル）→ 型 の対応＝型環境。
  # 不変：束縛を足すと「新しい」Scope を返し、元は変えない。
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
