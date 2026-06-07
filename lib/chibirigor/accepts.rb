# frozen_string_literal: true

module Chibirigor
  # 受理判定：「expected の所に actual を渡して合うか」を三値で答える。
  # :yes（確実に合う）/ :no（確実に合わない）/ :maybe（わからない）。
  module Accepts
    module_function

    def call(expected, actual)
      # untyped が絡んだら白黒つけない（gradual の核）。
      return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)

      # actual が Union：全メンバが通って初めて :yes。一番弱い結論を採る。
      return weakest(actual.members.map { |member| call(expected, member) }) if actual.is_a?(Type::Union)

      # expected が Union：どれか 1 つに合えばよい。一番強い結論を採る。
      return strongest(expected.members.map { |member| call(member, actual) }) if expected.is_a?(Type::Union)

      # 素朴な部分型：クラスが一致するか（Const は class_of がクラスに丸める）。
      Dispatch.class_of(expected) == Dispatch.class_of(actual) ? :yes : :no
    end

    # :no があれば :no、無ければ :maybe があれば :maybe、でなければ :yes
    def weakest(results)
      return :no if results.include?(:no)
      return :maybe if results.include?(:maybe)

      :yes
    end

    # :yes があれば :yes、無ければ :maybe があれば :maybe、でなければ :no
    def strongest(results)
      return :yes if results.include?(:yes)
      return :maybe if results.include?(:maybe)

      :no
    end
  end
end
