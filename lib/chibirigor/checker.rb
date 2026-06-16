# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Type-check the source and return the array of diagnostics found.
  # Thread the scope through statement by statement (assignments grow the type environment).
  # Don't stop on exceptions; read all the way through (don't halt, don't frighten).
  # Pass already-accepted diagnostics as baseline and they're subtracted out, returning *only the new ones*.
  # Matching is by "line + message" (no column or length — so editing the same line and
  # shifting columns doesn't knock the baseline loose).
  # explain: true also returns the points where inference fell back to untyped (fail-soft) as :info
  #   diagnostics (a tiny version of real Rigor's `rigor check --explain`).
  # unreachable: true also returns provably unreachable branches as :info diagnostics (a scaled-down ADR-47).
  # Both default to false, leaving behavior unchanged = returning only real type errors.
  def check(source, baseline = [], rbs: nil, explain: false, unreachable: false)
    program = Prism.parse(source).value
    diagnostics = []
    scope = Scope.new
    program.statements.body.each do |stmt|
      _type, scope = eval_statement(stmt, scope, diagnostics)
    end

    # Return-type checking (opt-in): run ⇐ only when rbs: is passed.
    # A def with no declaration, or an untyped declaration, passes silently (the gradual guarantee).
    if rbs
      user_sigs = Rbs.load(rbs)
      program.statements.body.each do |node|
        next unless node.is_a?(Prism::DefNode)

        sig = user_sigs.find { |(_klass, meth), _| meth == node.name }&.last
        next unless sig

        # Just synthesize the body's type (discard diagnostics — body errors were collected above).
        body_type = method_return_type(node, scope, [])
        check_against(node, sig[:returns], body_type, diagnostics)
      end
    end

    # Keep incidental events (those with :kind) separate from real type errors.
    special, errors = diagnostics.partition { |d| d[:kind] }

    seen = baseline.map { |d| d.slice(:line, :message) }
    result = errors.reject { |d| seen.include?(d.slice(:line, :message)) }

    # dump_type(expr) is a core feature: always include it, no flag needed (:info, type printout).
    result += special.select { |d| d[:kind] == :dump_type }.uniq
    # Include the rest only when the flag is set (dedupe only fully identical events).
    result += special.select { |d| d[:kind] == :fail_soft }.uniq if explain
    result += special.select { |d| d[:kind] == :unreachable }.uniq if unreachable
    result
  end
end
