---
title: Part 6 — The complete FactStore
description: Extend the Little volume's naive Scope and narrow into Rigor's real FactStore (the six-bucket design) and read it.
sidebar:
  order: 16
---

# The Seasoned chibirigor Part 6 — The complete FactStore

> References (optional): the general theory of flow analysis (dataflow analysis). The type-theory
> textbooks (TAPL / 『しくみ』) have no directly corresponding chapter — this is the own terrain of
> gradual, practical checkers. This chapter extends the Little volume's naive `Scope` / `narrow` into
> Rigor's real **FactStore.**

In Little [Part 3 (Scope)](../little/part3-scope-and-statements.md) we held the type environment as a
`Scope` (a variable-name → type Hash), and in Little
[Part 5 (Narrowing)](../little/part5-narrowing.md) we implemented narrowing with the naive mechanism
"swap the `Scope` per branch." At practical scale this isn't enough. We look at what's missing, and
how Rigor filled it.

In the same spirit as filling in arguments in Seasoned
[Part 5 (Real type inference)](part5-real-inference.md), here we generalize the flow-sensitive type
environment into a design that withstands practical use.

> **Individual narrowing patterns go to appendix a2.** This chapter narrows to the skeleton of six
> buckets + stability + join. The **individual narrowing patterns** — the stacking/unstacking of
> `&&`/`||`, regexp captures, refinement carriers (`non-empty-string`, etc.), ivar unions, escaping
> blocks — are consolidated in appendix
> [a2 — Narrowing patterns](../appendix/a2-narrowing-patterns.md). When you wonder "how does this
> narrowing concretely work," look there.

---

## 6-1. Hold "facts," not just "types"

The Little volume's `Scope` held **types** like "`x` is `Integer`." But in real code you want to flow
finer **facts**:

- `x` is not nil (after passing `if x`)
- `h` has the key `:name` (after `h.key?(:name)`)
- `arr` is not empty (after `arr.empty?` is false)
- `x` and `y` are the same value (after `x == y`)

These are *propositions holding at that point* rather than "types." The FactStore generalizes the
type environment into **a flow-sensitive set of facts.** In real Rigor, each fact (`FactStore::Fact`)
has a **`bucket`, `target`, `predicate`, `payload`, `polarity`, and `stability`.** This chapter
minimizes to three of these: `bucket`, `target`, `predicate`.

The precise type representing "the set of values satisfying a predicate" that `payload` carries —
the **refinement carriers** `non-empty-array`, `positive-int`, and the like — is a different concept
from Little [Part 1](../little/part1-literals-and-arithmetic.md)'s `Const[42]` (one specific value).
For details see a2-6 (refinement carrier) in appendix
[a2 — Narrowing patterns](../appendix/a2-narrowing-patterns.md).

---

## 6-2. Six "stores" (buckets)

Facts are held split into **buckets** by the kind of subject. Rigor has six kinds:

1. **local_binding** — local variables (the Little volume's `Scope#locals` is this).
2. **captured_local** — a local captured by a block.
3. **object_content** — an object's contents (ivars, hash keys, etc.).
4. **global_storage** — globals, class variables, etc.
5. **dynamic_origin** — the origin of `untyped` (where the type was lost).
6. **relational** — relations *between* variables (`x == y`, etc.).

```text
   FactStore (immutable)
   ├ local_binding    : x is non-nil          ┐
   ├ captured_local   : y written by a block   │ which scope the fact is
   ├ object_content   : obj.name is set        │ about — split into 5
   ├ global_storage   : $cfg is a Hash         │
   ├ relational       : a == b                 ┘
   └ dynamic_origin   : z made untyped at line N  ← different from the 5 (origin tracking)
```

![Figure 6-1 — the FactStore's six buckets](../figures/svg/seasoned-6-1.svg)
> ▼ Figure 6-1 — the FactStore's six buckets. The top 5 split by "*which subject* the fact attaches
> to." `dynamic_origin` alone is the odd one out, tracking not a target scope but "*where* `untyped`
> was born."

Why split — because the **timing of invalidation differs.** A reassignment to a local need only
erase that local_binding fact, but a method call must doubt object_content broadly. The bucket names
match the formal names in real Rigor's internal spec (`inference-engine.md`).[^buckets]

Note that the ivar (instance variable) type that enters the `object_content` bucket becomes "the
union of all visible assignments" (if `@x` could be made `nil` somewhere, a conservative
approximation containing `nil` wherever you read it). This individual pattern goes to a2-4 (ivar
union) in appendix [a2 — Narrowing patterns](../appendix/a2-narrowing-patterns.md).

[^buckets]: Of the six buckets, `local_binding`/`captured_local`/`object_content`/`global_storage`/
    `relational` split by "the subject a fact attaches to," whereas `dynamic_origin` alone is a
    separate lineage "tracking the *origin* of `untyped`." Note that its standing differs (it lines
    up as the sixth in the real spec too, but its role is separate).

---

## 6-3. When facts die (stability)

The most important concept the Little volume's naive `Scope` lacked is **stability = a fact's
lifetime.** A fact gained by narrowing *breaks under some operation*:

- **Reassignment:** `x = …` erases local_binding facts about `x` (we did this in the Little volume
  too).
- **Method call:** `obj.mutate!` doubts `obj`'s object_content facts (the contents may have changed).
- **Escape:** when a variable is passed to a block or another method, when it's modified is
  unreadable → conservatively erase.

Each fact holds "until when it's valid," and on the corresponding operation it is **conservatively
invalidated.** **When in doubt, erase** (fall to the looser side) — because rather than trusting an
old fact and emitting a false positive, throwing the fact away and returning to `untyped` is safer.
It precisely refines Little [Part 5 (Narrowing)](../little/part5-narrowing.md)'s "narrowing only adds
a fact; loosen if you got it wrong," down to the lifetime.

"Escape" is the case where a block that captured a variable **escapes outside the caller** (passing
the block to `Thread.new`, `define_method`, `Enumerator`, etc., where it's saved). An
immediately-invoked block like `each`/`map` can mostly retain facts, but an escaping block whose run
timing is unreadable conservatively invalidates the captured variables' `captured_local` facts — the
details of this "immediate vs. escape" judgment are consolidated in a2-3 (escaping block) in appendix
[a2 — Narrowing patterns](../appendix/a2-narrowing-patterns.md).

The standalone minimal sketch making this "immutable store + bucket-specified invalidation" run is
[`examples/fact_invalidation.rb`](examples/fact_invalidation.rb). `with_fact` / `invalidate_target`
return a *new* store (immutable):

```ruby
# An immutable bundle of facts. with_fact / invalidate_target return a *new* store.
class FactStore
  def initialize(facts = [])
    @facts = facts.freeze
  end

  def with_fact(bucket, target, predicate)
    FactStore.new(@facts + [Fact.new(bucket, target, predicate)])
  end

  # a new store with facts about target removed. if buckets is given, remove only those buckets.
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
```

With `ruby fact_invalidation.rb`, that **reassignment erases `x`'s fact**, and that **a method call
drops only `object_content` and keeps `local_binding`**, go **green**:

```text
PASS: fact is present after narrowing
PASS: reassignment clears x's local_binding fact
PASS: method call drops object_content but keeps local_binding
```

---

## 6-4. The hard spot of closure capture

Ruby's blocks can **capture and rewrite** outer locals:

```ruby
x = nil
[1, 2, 3].each { |i| x = i }   # the block rewrites x
# x is not necessarily nil here
```

Even if narrowing narrows `x` to "not nil," if the block could reassign `x`, that fact is *in
jeopardy.* The FactStore detects that a block **writes an outer local** and invalidates its
captured_local fact.

In addition to the escape seen in §6-3 (the *timing* of being called is unreadable), the difficulty
specific to capture is this "the block **rewrites** the outer." Even a block called immediately like
`each`, if it assigns to an outer local, breaks the narrowing. Handling differs by whether the
captured variable is **only read** or **written**, and if written, it drops the `captured_local` fact
even for an immediate call.

The Little volume's naive `Scope` handled this *not at all* (which is why the main volume didn't step
into narrowing inside blocks like `each`). In practice this is a hotbed of false positives, and the
spot Rigor is most careful with.

---

## 6-5. Confluence (join) — when branches merge

After an `if`'s two branches merge, which facts survive? The answer is "**only facts holding on both
branches**":

```ruby
if cond
  # branch A: x is Integer
else
  # branch B: x is String
end
# after the merge: x is Integer | String (the "common part" of both branches' facts = join)
```

The FactStore's `join` keeps only the *common part* of the two entrances' fact sets (types union,
facts intersect). "join" is dataflow analysis's idiom (the operation at a merge point), corresponding
not to the type lattice's join (upper bound) but to the fact lattice's meet (common part). The Little
volume combined an `if`'s result type with `Type.union`, but didn't do join at the fact level. The
Seasoned volume's FactStore generalizes that.

---

## 6-6. Still immutable, still flow-sensitive

Even with all this added, the design's core is the same as the Little volume:

- **Immutable:** the FactStore, like `Scope`, is immutable. `with_fact`/`invalidate` return a *new*
  store. "What holds at which point" is traceable without destroying state.
- **Flow-sensitive:** facts differ at each point of the program. The same `x` holds different facts
  inside and outside an `if`.
- **Narrowing only *adds* a fact:** not type substitution but the addition of a fact (still Little
  [Part 5 (Narrowing)](../little/part5-narrowing.md)'s policy).

---

## 6-7. Summary

- Generalize the type environment into a flow-sensitive **set of facts** = the FactStore.
- Facts split into **six buckets** by the kind of subject, and split the **timing of invalidation.**
- **stability (lifetime):** conservatively erase on reassignment, method call, escape. When in doubt,
  erase.
- **Closure capture:** if a block writes the outer, invalidate the fact. Handling differs by how it's
  called (immediate / deferred).
- **join:** at a branch merge, keep only facts holding on both branches.
- Immutable, flow-sensitive, "only adds a fact" — invariant from the Little volume.
- The individual narrowing patterns (`&&`/`||`, regexp captures, refinement carriers, ivar unions,
  escaping blocks) are consolidated in appendix [a2](../appendix/a2-narrowing-patterns.md).

## Exercises

1. **Reassignment erases a fact:** in `examples/fact_invalidation.rb`, state in which bucket and why
   `x`'s "non-nil" fact should be invalidated after `x = nil; arr.each { |i| x = i }`
   (`local_binding` or `captured_local`).
2. **Bucket-specified invalidation:** in one sentence, why it's safe for `obj.mutate!` to drop only
   `obj`'s `object_content` facts and keep `local_binding` (that `obj` is a User, etc.). And what
   goes wrong if you erase everything.
3. **Trace a join:** after the merge of `if cond; x=1 else x="a" end`, what facts remain about `x`
   (the intersection of both branches)? If you kept "facts holding on one side" too, state why it
   becomes a false positive.

---

**Next chapter (Part 7):** so far we've stacked "when in doubt, erase" and "deliberately loosen." Next
chapter we take those head-on — **soundness and normalization**, and the four places Rigor makes
*deliberately* unsound. We gather gradual's two disciplines (consistency and guarantee) and
"coinduction vs. budget" in one place, pairing with Seasoned Part 1's bidirectional map.

---

> **This chapter's design sketch** → [`examples/fact_invalidation.rb`](examples/fact_invalidation.rb) (self-checks with `ruby fact_invalidation.rb`)
