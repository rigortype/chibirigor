# frozen_string_literal: true

module Chibirigor
  # Typing of method sends. Since everything in Ruby is a method send, we hold
  # "which method of which class takes what arguments and returns what" in a table.
  # In Part 7 we swapped this table from hand-written to RBS-derived (Rbs.load).
  module Dispatch
    # [receiver class, method name] => { params: [arg types...], returns: return type }
    METHODS = Rbs.load(Rbs::CORE)

    # Foldable operations: when both operands are "Const of known value," actually compute and fold to Const.
    FOLD = {
      %i[Integer +] => ->(a, b) { a + b },
      %i[Integer -] => ->(a, b) { a - b },
      %i[Integer *] => ->(a, b) { a * b },
      %i[String +] => ->(a, b) { a + b },
      %i[String *] => ->(a, b) { a * b } # "a" * 3 → "aaa"
    }.freeze

    # Widening rules to prevent Const explosion (a minimal normalization budget). Past these, round instead of fold.
    INT_LIMIT = 1_000_000
    STR_LIMIT = 100
    MEMBER_LIMIT = 4 # Union member-count budget. Past this, round Const to its class.

    # Predicates narrowing treats specially (their return is conceptually bool). chibirigor has no bool
    # type so it returns Dynamic, but this is not "lost the type" — it's just "we don't model the predicate."
    # So we don't list them on the fail-soft map (check --explain) — don't emit a false silence site.
    GUARD_PREDICATES = %i[nil? is_a? kind_of? instance_of?].freeze

    module_function

    # Round a type to its "class name (symbol)." Used to match the table's keys.
    def class_of(type)
      case type
      when Type::Const   then type.value.class.name.to_sym
      when Type::Nominal then type.name
      when Type::Generic then type.name # Array[Integer] → :Array
      end # Dynamic etc. → nil (= can't dispatch)
    end

    # Return the folded Const if foldable (a Union folded per member if an argument is a Union),
    # or nil if it can't be folded / should be rounded.
    # E.g. 1 + (1 | 2) → 2 | 3. If the combinations exceed MEMBER_LIMIT, round instead of fold.
    def foldable_result(receiver_type, name, arg_types)
      op = FOLD[[class_of(receiver_type), name]]
      return nil unless op && receiver_type.is_a?(Type::Const)

      combinations = const_combinations(arg_types)
      return nil if combinations.nil? || combinations.size > MEMBER_LIMIT

      members = combinations.map do |args|
        result = begin
          op.call(receiver_type.value, *args.map(&:value))
        rescue StandardError
          return nil # if any combination's computation fails, don't fold (the type error is diagnosed separately; the return is left to rounding)
        end
        return nil if widen?(result)

        Type::Const[result]
      end
      Type.union(members)
    end

    # Expand each argument into its "list of Const members" and return the product. nil if anything but Const is mixed in (can't fold).
    def const_combinations(arg_types)
      member_lists = arg_types.map do |type|
        members = type.is_a?(Type::Union) ? type.members : [type]
        return nil unless members.all?(Type::Const)

        members
      end
      member_lists.reduce([[]]) { |acc, members| acc.product(members).map { |combo, m| combo + [m] } }
    end

    # Don't fold a value that's too big (over budget → round)
    def widen?(value)
      case value
      when Integer then value.abs > INT_LIMIT
      when String  then value.length > STR_LIMIT
      else true
      end
    end

    def dispatch(receiver_type, name, arg_types, node, diagnostics)
      # A Union receiver dispatches per member and combines the results (distribution).
      return dispatch_union(receiver_type, name, arg_types, node, diagnostics) if receiver_type.is_a?(Type::Union)

      key = [class_of(receiver_type), name]
      signature = Plugin.registry[key] || METHODS[key]
      unless signature # unknown method → don't frighten it (this is the fail-soft site)
        # Record the site we fell back to untyped as provenance (check --explain maps it).
        # A normal check discards this :fail_soft, so diagnostics don't increase (behavior unchanged).
        # A narrowing predicate (is_a? etc.) hasn't lost the type, so it isn't put on the map.
        unless GUARD_PREDICATES.include?(name)
          diagnostics << Chibirigor.diagnostic(node, "fell to untyped here (can't look up the type of `#{name}`)")
                                   .merge(kind: :fail_soft, severity: :info)
        end
        return Type::Dynamic.new
      end

      if arg_types.size != signature[:params].size
        diagnostics << Chibirigor.diagnostic(
          node, "wrong number of arguments for #{name} (#{signature[:params].size} expected, #{arg_types.size} given)"
        )
        return signature[:returns]
      end

      signature[:params].zip(arg_types).each do |param, arg|
        # Complain only on :no (definitely doesn't fit). Both :yes and :maybe stay quiet.
        next unless Accepts.call(param, arg) == :no

        diagnostics << Chibirigor.diagnostic(node, "expected #{param} but got #{arg}")
      end

      # Fold if foldable (Const); otherwise round to the table's return type.
      foldable_result(receiver_type, name, arg_types) || signature[:returns]
    end

    # Distributive dispatch for a Union receiver. At run time it can be any member,
    # so an error surfaces only when "all members failed" (a partial failure is :maybe = stay quiet).
    # :info (the fail-soft map) is provenance, so pass it through as is (just drop duplicates).
    def dispatch_union(receiver_type, name, arg_types, node, diagnostics)
      buffers = []
      results = receiver_type.members.map do |member|
        buffers << (buffer = [])
        dispatch(member, name, arg_types, node, buffer)
      end
      diagnostics.concat(merge_member_diagnostics(buffers))
      budgeted_union(results)
    end

    def merge_member_diagnostics(buffers)
      infos, errors = buffers.flatten.partition { |d| d[:severity] == :info }
      merged = infos.uniq
      merged.concat(errors.uniq) if buffers.all? { |b| b.any? { |d| d[:severity] != :info } }
      merged
    end

    # Union member-count budget. Past MEMBER_LIMIT, round Const to its class and rebuild
    # (the Union version of the same "normalization budget" pattern as widen?).
    def budgeted_union(types)
      result = Type.union(types)
      return result unless result.is_a?(Type::Union) && result.members.size > MEMBER_LIMIT

      Type.union(result.members.map { |m| m.is_a?(Type::Const) ? Type::Nominal[class_of(m)] : m })
    end
  end
end
