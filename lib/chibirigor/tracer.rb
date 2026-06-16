# frozen_string_literal: true

module Chibirigor
  # trace — record the steps of type inference as an event stream and replay them frame by frame in the terminal.
  #
  # How it works: the core (type_of / eval_statement / Type.union / Dispatch.dispatch) is left
  # untouched; hooks are inserted with Module#prepend. When Tracer.current (the recorder) is
  # nil, the hooks immediately super, so check / annotate etc. behave unchanged.
  #
  # Event kinds (= candidate frames of the animation):
  #   :stmt     a top-level statement's evaluation begins
  #   :enter    type_of entered a node (not shown in the default replay; shown with --verbose)
  #   :result   a node's type was fixed
  #   :bind     an assignment added a binding to the type environment (scope)
  #   :union    Type.union combined several types into one
  #   :dispatch a method send was looked up in the table (folding / fail-soft show up here too)
  module Tracer
    class << self
      attr_accessor :current
    end

    # Place a recorder on Tracer.current only while tracing
    def self.with(recorder)
      self.current = recorder
      yield
    ensure
      self.current = nil
    end

    # The recorder that accumulates events. Types and scopes are recorded as display strings
    # (so the data is self-contained — emittable later to JSON or to the terminal).
    class Recorder
      attr_reader :events

      def initialize
        @events = []
        @stack = [] # labels of the nodes being evaluated (breadcrumbs)
        @nodes = [] # the nodes being evaluated themselves (used to position a union)
        @scope = {} # a snapshot of the most recent scope
      end

      def stmt(node, index, scope)
        @scope = snapshot(scope)
        emit(:stmt, node, index: index)
      end

      def enter(node, scope)
        @stack.push(label(node))
        @nodes.push(node)
        @scope = snapshot(scope)
        emit(:enter, node, label: label(node))
      end

      def leave(node, type)
        emit(:result, node, label: label(node), node: short(node), type: type.to_s)
        @stack.pop
        @nodes.pop
      end

      def bind(node, type, new_scope)
        @scope = snapshot(new_scope)
        emit(:bind, node, name: node.name.to_s, type: type.to_s)
      end

      def union(inputs, result)
        emit(:union, @nodes.last, inputs: inputs.map(&:to_s), result: result.to_s)
      end

      def dispatch(node, receiver, name, args, result, folded:, fail_soft:, distributed:)
        emit(:dispatch, node,
             receiver: receiver.to_s, method: name.to_s, args: args.map(&:to_s),
             result: result.to_s, folded: folded, fail_soft: fail_soft, distributed: distributed)
      end

      private

      def emit(kind, node, **details)
        location = node&.location
        @events << {
          kind: kind,
          line: location&.start_line, column: location&.start_column,
          end_line: location&.end_line, end_column: location&.end_column,
          depth: @stack.size, stack: @stack.dup,
          scope: @scope,
          **details
        }
      end

      def snapshot(scope)
        scope.instance_variable_get(:@locals).to_h { |name, type| [name.to_s, type.to_s] }
      end

      def label(node)
        case node
        when Prism::CallNode then "call to #{node.name}"
        when Prism::IfNode then 'if (incl. ternary)'
        when Prism::LocalVariableReadNode then "variable #{node.name}"
        when Prism::LocalVariableWriteNode then "assignment #{node.name}"
        else short(node).sub(/Node\z/, '')
        end
      end

      def short(node)
        node.class.name.split('::').last
      end
    end

    # ── Hooks (cut in with prepend; immediately super if there's no recorder) ──────────────

    module TypeOfHook
      def type_of(node, scope, diagnostics)
        return super unless (recorder = Tracer.current)

        recorder.enter(node, scope)
        type = super
        recorder.leave(node, type)
        type
      end

      def eval_statement(node, scope, diagnostics)
        return super unless (recorder = Tracer.current)

        type, new_scope = super
        recorder.bind(node, type, new_scope) if node.is_a?(Prism::LocalVariableWriteNode)
        [type, new_scope]
      end
    end

    module UnionHook
      def union(types)
        result = super
        # A call that just returns a single type as is isn't a "combine," so don't record it
        Tracer.current&.union(types, result) if types.size >= 2
        result
      end
    end

    module DispatchHook
      def dispatch(receiver_type, name, arg_types, node, diagnostics)
        return super unless (recorder = Tracer.current)

        distributed = receiver_type.is_a?(Type::Union) # a Union receiver is distributed to its members
        key = [Dispatch.class_of(receiver_type), name]
        signature = Plugin.registry[key] || Dispatch::METHODS[key]
        result = super
        recorder.dispatch(node, receiver_type, name, arg_types, result,
                          folded: Dispatch::FOLD.key?(key) && const_only?(result),
                          fail_soft: !distributed && signature.nil?,
                          distributed: distributed)
        result
      end

      # A Const, or a "Union of only Consts" (the result of folding per member)
      def const_only?(type)
        type.is_a?(Type::Const) ||
          (type.is_a?(Type::Union) && type.members.all?(Type::Const))
      end
    end

    Chibirigor.singleton_class.prepend(TypeOfHook)
    Type.singleton_class.prepend(UnionHook)
    Dispatch.singleton_class.prepend(DispatchHook)

    # ── Replay (terminal animation) ────────────────────────────────────────────

    # Events not shown in the default replay: enter (the "in" of in/out), and
    # the result of a trivial literal (that 1 is Const[1] carries no information to look at).
    LITERAL_NODES = %w[IntegerNode FloatNode StringNode SymbolNode TrueNode FalseNode NilNode].freeze

    module_function

    def skip_by_default?(event)
      event[:kind] == :enter ||
        (event[:kind] == :result && LITERAL_NODES.include?(event[:node]))
    end

    # Replay the event stream frame by frame.
    #   delay: given a number of seconds, auto-play. nil → step with the Enter key.
    #   verbose: true shows enter / literal results in full too.
    # If out is not a terminal (a pipe, etc.), print every frame in order without clearing the screen.
    def play(source, events, delay: nil, verbose: false, out: $stdout)
      frames = verbose ? events : events.reject { |e| skip_by_default?(e) }
      lines = source.lines.map(&:chomp)
      animate = out.respond_to?(:tty?) && out.tty?

      frames.each_with_index do |event, index|
        out.print "\e[H\e[2J" if animate
        draw(out, lines, event, index, frames.size)
        if delay
          sleep delay
        elsif animate && $stdin.tty?
          out.print "\n[Enter] next / [q] quit > "
          input = $stdin.gets
          break if input.nil? || input.strip == 'q'
        end
      end
      out.puts "\n── playback done (#{frames.size} steps total) ──"
    end

    def draw(out, lines, event, index, total)
      bar = '─' * 64
      out.puts "chibirigor trace ─ step #{index + 1}/#{total}"
      out.puts bar
      lines.each_with_index do |text, i|
        out.puts format('%3d  %s', i + 1, highlight(text, i + 1, event))
      end
      out.puts bar
      scope = event[:scope]
      out.puts "type env   : #{scope.empty? ? '(empty)' : scope.map { |k, v| "#{k}: #{v}" }.join('   ')}"
      out.puts "evaluating : #{event[:stack].empty? ? '(top level)' : event[:stack].join(' › ')}"
      out.puts "► #{message(event)}"
    end

    # Highlight the event's range (start..end line/column) in inverse video
    def highlight(text, lineno, event)
      return text unless event[:line] && lineno.between?(event[:line], event[:end_line])

      from = lineno == event[:line] ? event[:column] : 0
      to = lineno == event[:end_line] ? event[:end_column] : text.length
      from = from.clamp(0, text.length)
      to = to.clamp(from, text.length)
      "#{text[0...from]}\e[7m#{text[from...to]}\e[0m#{text[to..]}"
    end

    def message(event)
      case event[:kind]
      when :stmt then "start evaluating statement #{event[:index]}"
      when :enter then "entering #{event[:label]}"
      when :result then "#{event[:label]} ⇒ #{event[:type]}"
      when :bind then "bind: #{event[:name]} ← #{event[:type]} (added to type env)"
      when :union then "union: #{event[:inputs].join(' , ')} → #{event[:result]}"
      when :dispatch
        note = if event[:distributed] then '(distribute Union to members)'
               elsif event[:folded] then '(constant folding)'
               elsif event[:fail_soft] then '(not in table → fail-soft to untyped)'
               else '(round to the table\'s return type)'
               end
        "dispatch: #{event[:receiver]}.#{event[:method]}(#{event[:args].join(', ')}) → #{event[:result]} #{note}"
      else event[:kind].to_s
      end
    end
  end

  module_function

  # Infer the source while recording the event stream and return it (the body of the trace command).
  # The return value is an array of self-contained Hashes (JSON-serializable as is).
  def trace(source)
    program = Prism.parse(source).value
    recorder = Tracer::Recorder.new
    Tracer.with(recorder) do
      scope = Scope.new
      diagnostics = []
      program.statements.body.each_with_index do |stmt, index|
        recorder.stmt(stmt, index + 1, scope)
        _type, scope = eval_statement(stmt, scope, diagnostics)
      end
    end
    recorder.events
  end
end
