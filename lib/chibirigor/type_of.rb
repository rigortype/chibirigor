# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Find a type from an expression (a Prism node). The heart of the type checker.
  # scope is the type environment (variable name → type). If unknown, return Dynamic (don't frighten).
  def type_of(node, scope, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::NilNode     then Type::Const[nil]
    when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
    when Prism::HashNode then type_of_hash(node, scope, diagnostics)
    when Prism::ArrayNode then Type::Tuple[node.elements.map { |el| type_of(el, scope, diagnostics) }.freeze]
    when Prism::CallNode then type_of_call(node, scope, diagnostics)
    when Prism::IfNode then type_of_if(node, scope, diagnostics)
    when Prism::ParenthesesNode then type_of_body(node.body, scope, diagnostics)
    when Prism::DefNode then type_of_def(node, scope, diagnostics)
    else Type::Dynamic.new
    end
  end

  # Method definition. Type-check the body and return the def expression's value (the method-name symbol).
  def type_of_def(node, scope, diagnostics)
    method_return_type(node, scope, diagnostics) # type-check the body (collect diagnostics)
    Type::Const[node.name]
  end

  # Synthesize a method's return type from its body. Params are untyped (the main volume doesn't infer arguments).
  def method_return_type(node, scope, diagnostics)
    body_scope = method_param_names(node).reduce(scope) { |s, name| s.with_local(name, Type::Dynamic.new) }
    type_of_body(node.body, body_scope, diagnostics)
  end

  def method_param_names(node)
    node.parameters&.requireds&.map(&:name) || []
  end

  # Hash literal → HashShape (remembers symbol keys only).
  def type_of_hash(node, scope, diagnostics)
    fields = {}
    node.elements.each do |assoc|
      next unless assoc.is_a?(Prism::AssocNode) && assoc.key.is_a?(Prism::SymbolNode)

      fields[assoc.key.unescaped.to_sym] = type_of(assoc.value, scope, diagnostics)
    end
    Type::HashShape[fields.freeze]
  end

  # if / ternary. Combine both branches' types, narrowing the type per branch.
  def type_of_if(node, scope, diagnostics)
    type_of(node.predicate, scope, diagnostics) # type-check the condition too (catch nested errors)

    # Record a provably-unreachable branch (surfaces only with check(unreachable: true); harmless by default).
    if Narrowing.unreachable_branch?(scope, node.predicate, true)
      diagnostics << unreachable_diagnostic(node.statements || node, true)
    end
    then_type = type_of_body(node.statements, Narrowing.narrow(scope, node.predicate, true), diagnostics)

    else_type =
      if node.subsequent
        if Narrowing.unreachable_branch?(scope, node.predicate, false)
          diagnostics << unreachable_diagnostic(node.subsequent.statements || node.subsequent, false)
        end
        type_of_body(node.subsequent.statements, Narrowing.narrow(scope, node.predicate, false), diagnostics)
      else
        Type::Const[nil] # no else → nil when false
      end

    Type.union([then_type, else_type])
  end

  # Unreachable-arm diagnostic (a reduced version of ADR-47). Held as :info, kind :unreachable.
  # A truthy=true branch means "the condition is always false"; a false branch "always true (the else dies)."
  def unreachable_diagnostic(node, truthy)
    reason = truthy ? "the condition is always false" : "the condition is always true"
    diagnostic(node, "this branch is unreachable (#{reason})").merge(kind: :unreachable, severity: :info)
  end

  # Type printout for dump_type(expr) (:info, kind :dump_type). The value passes through, so it isn't a type error.
  # A basic feature check always co-emits with no flag (corresponds to real Rigor's Rigor::Testing.dump_type).
  def dump_type_diagnostic(node, type)
    diagnostic(node, "dump_type: #{type}").merge(kind: :dump_type, severity: :info)
  end

  # Evaluate a branch (a statement sequence) and return the last statement's type. Threads scope inside branches too.
  def type_of_body(statements_node, scope, diagnostics)
    return Type::Const[nil] if statements_node.nil?

    last = Type::Const[nil]
    statements_node.body.each { |stmt| last, scope = eval_statement(stmt, scope, diagnostics) }
    last
  end

  # Method send. Find the types of the receiver and each argument, then hand them to the dispatch table.
  # (Part 1's ad-hoc `+` special case was generalized into a hand-written table in Part 2.)
  def type_of_call(node, scope, diagnostics)
    # dump_type(expr) — a basic feature printing the inferred type at that position as :info (corresponds to
    # real Rigor's Rigor::Testing.dump_type). At run time it returns the value as is, so the type is the argument's type.
    if node.receiver.nil? && node.name == :dump_type && (node.arguments&.arguments || []).size == 1
      t = type_of(node.arguments.arguments.first, scope, diagnostics)
      diagnostics << dump_type_diagnostic(node, t)
      return t
    end

    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_nodes = node.arguments&.arguments || []

    # Block iteration (generics 5b/5c): for a known array, push the element type down to the block parameter.
    if node.block.is_a?(Prism::BlockNode)
      blocked = type_of_block(receiver, node.name, node.block, scope, diagnostics)
      return blocked if blocked
    end

    # Index read of a structural type (h[:k] / a[0]) — special-case only a literal key/index.
    if node.name == :[] && arg_nodes.size == 1
      indexed = read_index(receiver, arg_nodes.first)
      return indexed if indexed
    end

    # Element-type read (generics 5a): read the element type Elem from a known array / hash.
    # arr.first / arr.last / non-literal index arr[i] → element type; h.values / h.keys → value / key type.
    element = element_read(receiver, node.name, arg_nodes)
    return element if element

    arg_types = arg_nodes.map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  # Element-type read (generics 5a). nil if not readable (falls through to ordinary dispatch).
  # Reads the element type only from a *known shape* — Tuple/Array[Elem]/HashShape. A raw Dynamic
  # receiver isn't picked up here and falls back to untyped (untyped if it can't be filled = zero false positives).
  def element_read(receiver, name, arg_nodes)
    if (elem = element_type_of(receiver))
      case name
      when :first, :last then elem if arg_nodes.empty?
      when :[] then elem if arg_nodes.size == 1
      end
    elsif receiver.is_a?(Type::HashShape)
      case name
      when :values then hash_value_type(receiver) if arg_nodes.empty?
      when :keys then hash_key_type(receiver) if arg_nodes.empty?
      end
    end
  end

  # An array's element type Elem. A Tuple gathers all elements (literal precision rounded to a class);
  # an Array[Elem] is its type argument. An empty array's element is unknown, so untyped (FP-safe).
  # If it's not an array, nil (= "not a shape whose elements can be read").
  def element_type_of(receiver)
    case receiver
    when Type::Tuple then Type.union(receiver.elements.map { |t| widen_element(t) })
    when Type::Generic then receiver.args.first if receiver.name == :Array
    end
  end

  def hash_value_type(shape)
    Type.union(shape.fields.values.map { |t| widen_element(t) })
  end

  # We remember symbol keys only (Part 5), so the key type is Symbol. Empty → untyped.
  def hash_key_type(shape)
    shape.fields.empty? ? Type::Dynamic.new : Type::Nominal[:Symbol]
  end

  # For element types, widen "this value itself (Const)" to its class (`[1,2].first` is Integer, not `1`).
  def widen_element(type)
    type.is_a?(Type::Const) ? Type::Nominal[Dispatch.class_of(type)] : type
  end

  # Handle only iterators where "the first parameter = the element" is certain (FP-safe).
  # each_with_index (|x, i|) and reduce (|acc, x|) have different parameter meanings, so they aren't included.
  ELEMENT_ITERATORS = %i[map collect each select filter reject find_all].freeze

  # Typing of block iteration (generics 5b/5c). Push a known array's element type Elem down to the block
  # parameter and type-check the body. If it's not an array / an unknown iterator, nil (to ordinary dispatch).
  #   map/collect → Array[body's type] (5c return polymorphism) / each → the receiver (returns self) /
  #   select/filter/reject/find_all → Array[Elem] (element type unchanged).
  def type_of_block(receiver, name, block, scope, diagnostics)
    return nil unless ELEMENT_ITERATORS.include?(name)

    elem = element_type_of(receiver)
    return nil if elem.nil? # don't touch it unless it's a known array (fall back to untyped = no FP)

    body_type = type_of_body(block.body, bind_block_params(block, elem, scope), diagnostics)

    case name
    when :map, :collect then Type::Generic[:Array, [widen_element(body_type)].freeze]
    when :each then receiver
    else Type::Generic[:Array, [elem].freeze]
    end
  end

  # Return a scope with the block parameters bound. First parameter = element type Elem; the rest are untyped (safe side).
  def bind_block_params(block, elem, scope)
    block_param_names(block).each_with_index.reduce(scope) do |s, (name, index)|
      s.with_local(name, index.zero? ? elem : Type::Dynamic.new)
    end
  end

  def block_param_names(block)
    block.parameters&.parameters&.requireds&.map(&:name) || []
  end

  # Reading out of a structural type. nil if not readable (falls through to ordinary dispatch).
  def read_index(receiver, arg_node)
    if receiver.is_a?(Type::HashShape) && arg_node.is_a?(Prism::SymbolNode)
      # unknown key is nil (because real Ruby returns nil. don't error)
      return receiver.fields.fetch(arg_node.unescaped.to_sym, Type::Const[nil])
    end
    if receiver.is_a?(Type::Tuple) && arg_node.is_a?(Prism::IntegerNode)
      return receiver.elements.fetch(arg_node.value, Type::Const[nil])
    end

    nil
  end

  # ⇐ subsumption (checking mode): check actual against the expected type.
  # Emit a diagnostic only on :no. Stay quiet if untyped is involved (gradual's promise).
  # The first mouth where `⇐` emits a diagnostic. The call-argument check (dispatch) runs on the same principle.
  def check_against(node, expected, actual, diagnostics)
    return if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)
    return unless Accepts.call(expected, actual) == :no

    diagnostics << diagnostic(node, "return type #{expected} is declared but #{actual} is returned")
  end

  # A diagnostic is "which line, what's wrong." The position (line, column, length) is used for the caret display.
  def diagnostic(node, message)
    location = node.location
    {
      line: location.start_line,
      column: location.start_column,
      length: location.length,
      message: message
    }
  end
end
