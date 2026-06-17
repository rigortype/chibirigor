# frozen_string_literal: true

# The Seasoned chibirigor Part 6 - a minimal sketch of "reassignment erases facts" (the core of FactStore).
# Zero dependencies, runs standalone. `ruby fact_invalidation.rb` self-checks green.
#
# Real Rigor's FactStore has 6 buckets + payload + polarity + stability, but here we minimize it
# to just "immutable store" and "bucket-scoped invalidation" to watch facts disappear on reassignment.

Fact = Struct.new(:bucket, :target, :predicate)

# region factstore
# An immutable bundle of facts. with_fact / invalidate_target return a *new* store.
class FactStore
  def initialize(facts = [])
    @facts = facts.freeze
  end

  def with_fact(bucket, target, predicate)
    FactStore.new(@facts + [Fact.new(bucket, target, predicate)])
  end

  # A new store with facts about target removed. If buckets is given, only those buckets are cleared.
  def invalidate_target(target, buckets: nil)
    kept = @facts.reject do |f|
      f.target == target && (buckets.nil? || buckets.include?(f.bucket))
    end
    FactStore.new(kept)
  end

  def predicates_for(target)
    @facts.select { |f| f.target == target }.map(&:predicate)
  end
end
# endregion

# --- Self-check --------------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  results = {}

  # After passing if x (a nil check): add the local_binding fact that x is non-nil.
  store = FactStore.new.with_fact(:local_binding, :x, 'non-nil')
  results['fact is present after narrowing'] = store.predicates_for(:x) == ['non-nil']

  # Reassignment x = ... -> x's local_binding fact is cleared.
  after_reassign = store.invalidate_target(:x, buckets: [:local_binding])
  results["reassignment clears x's local_binding fact"] = after_reassign.predicates_for(:x) == []

  # Bucket scoping: a method call on obj casts doubt only on object_content.
  # local_binding (the binding of obj itself) survives.
  mixed = FactStore.new
                   .with_fact(:local_binding, :obj, 'is-a User')
                   .with_fact(:object_content, :obj, 'name is set')
  after_call = mixed.invalidate_target(:obj, buckets: [:object_content])
  results['method call drops object_content but keeps local_binding'] =
    after_call.predicates_for(:obj) == ['is-a User']

  results.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(results.values.all? ? 0 : 1)
end
