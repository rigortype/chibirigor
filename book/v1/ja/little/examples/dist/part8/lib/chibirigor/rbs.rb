# frozen_string_literal: true

module Chibirigor
  # A tiny RBS loader. Instead of the real rbs gem, read the bare minimum ourselves.
  # A miniature of the Ruby/RBS worldview where "types live in a separate file (RBS)."
  # Part 2's hand-written table is replaced by one generated from this RBS text.
  module Rbs
    module_function

    CLASS_LINE = /\A\s*class\s+(\S+)\s*\z/
    DEF_LINE   = /\A\s*def\s+(\S+):\s*\((.*)\)\s*->\s*(\S+)\s*\z/

    CORE = <<~RBS
      class Integer
        def +: (Integer) -> Integer
        def -: (Integer) -> Integer
        def *: (Integer) -> Integer
        def to_s: () -> String
      end
      class String
        def +: (String) -> String
        def *: (Integer) -> String
        def length: () -> Integer
        def upcase: () -> String
      end
    RBS

    def load(source)
      table = {}
      current = nil
      source.each_line do |line|
        if (m = CLASS_LINE.match(line))
          current = m[1].to_sym
        elsif current && (m = DEF_LINE.match(line))
          params = m[2].split(',').map(&:strip).reject(&:empty?).map { |t| Type::Nominal[t.to_sym] }
          ret = m[3] == 'untyped' ? Type::Dynamic.new : Type::Nominal[m[3].to_sym]
          table[[current, m[1].to_sym]] = { params: params.freeze, returns: ret }
        end
      end
      table.freeze
    end
  end
end
