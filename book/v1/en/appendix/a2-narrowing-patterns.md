---
title: Appendix a2 — Narrowing patterns
description: A reference appendix factored out of the main line of Seasoned Part 6 (the complete FactStore), listing individual narrowing patterns by "when a fact is born and when it dies."
sidebar:
  order: 22
---

# Appendix a2 — Narrowing patterns

> **This appendix is a warm-up for the Seasoned volume. If you're reading only the Little volume,
> feel free to skip it** (the Little volume is complete without appendix a2).
>
> **Pointer back to the main line:** this appendix is a collection of individual narrowing
> patterns factored out of the main line of Seasoned
> [Part 6 — The complete FactStore](../seasoned/part6-fact-store.md) (the six buckets, stability,
> join). If, while reading Part 6, you wonder "how does this narrowing concretely work," look here.
> Each pattern is summed up concisely by **when a fact is born / when it dies.** For the presumed
> naive narrowing (`narrow` / `if x.nil?` / `is_a?` / reassignment reset) see Little
> [Part 5 — Narrowing](../little/part5-narrowing.md); for terms, the [glossary](../glossary.md).

The FactStore generalizes the type environment into **a flow-sensitive set of facts** (Seasoned
Part 6). A "fact" here is a proposition holding at that point — "`x` is not nil," "`arr` is not
empty," and so on — held in six buckets by the kind of subject (`local_binding` / `captured_local`
/ `object_content` / `global_storage` / `relational` / `dynamic_origin`). This appendix gathers, in
one place, the individual patterns of **how those facts stack up and how they die.**

---

## a2-1. `&&` stacks facts up, `||` shaves them off

Because the `&&` operator is **evaluated sequentially** left to right, the FactStore's facts too
stack up in order from the left.

```ruby
if x.is_a?(Integer) && x > 0
  # here two facts are stacked in local_binding
  #   1. x is_a? Integer   (is_a? narrowing)
  #   2. x > 0             (comparison predicate)
  # composed, it reads as x : positive-int
end
```

The moment the left `is_a?(Integer)` passes, `x`'s type is narrowed to `Integer`, and in that state
the right `x > 0` is evaluated. Since "`Integer` and `> 0`" stack up, Rigor can treat this as a
**`positive-int` refinement** (→ a2-6).

Conversely, a `||` chain is "if either one holds," so at the merge point a join runs (keep only the
common facts; the join of Seasoned Part 6), and a fact present on only one side dies.

| Operator | Effect on facts | When born / dies |
|---|---|---|
| `&&` | *adds* | stacks sequentially from the left. evaluates the right with the left having passed |
| `\|\|` | *removes* | join at the merge. only facts common to both sides remain |

`&&` adds, `||` removes — this is why the FactStore doesn't treat left and right symmetrically.

---

## a2-2. A regexp's named captures produce String after a match

Ruby's `=~` and named captures (`(?<name>...)`) have a behavior almost unique to it — **on a
successful match they bind `String` to local variables.**

```ruby
if /(?<year>\d{4})-(?<month>\d{2})/ =~ str
  # year, month are bound as String
  year.upcase   # OK (year is String)
end
```

Rigor recognizes this as **named-capture narrowing.** Inside the `if` block it adds a `String` fact
to `year`'s and `month`'s `local_binding`.

- **A fact is born:** when `=~`'s left side is a regexp literal containing named captures, on the
  match-success side (inside the `if` block) a `String` fact enters the local of the same name as
  the capture.
- **A fact dies (doesn't reach):** on a match failure the bindings are `nil`. So outside the `if`,
  both stay `String | nil`.

In Prism, when `=~`'s left side is a `RegexpNode` containing named captures, Rigor reads out the
capture-group names and inserts facts directly into the FactStore. The same mechanism as `is_a?`'s
type predicate and `nil?`'s nil guard, but special in that **the variable names come from the
regexp's body.**

| Pattern | Narrowing target | Fact added |
|---|---|---|
| `is_a?(String)` | the left-side variable | `String` |
| `nil?` negation | the left-side variable | `non-nil` |
| `=~` named capture | the capture-named variable | `String` |

---

## a2-3. Facts die in an escaping block

`each` and `map` blocks are immediately invoked, so narrowing facts can be mostly retained until
after the block ends. The problem is an **escaping block** — when the block "escapes" outside the
caller.

```ruby
if x.is_a?(Integer)
  # here "is Integer" enters x's local_binding
  Thread.new { x.some_integer_method }   # ← captures x and sends it to another thread
end
# when the Thread runs is unknown → keeping x's narrowing is dangerous
```

The block passed to `Thread.new` runs at *any timing.* We can't rule out that `x` was reassigned by
then, or is already a different type.

- **A fact dies:** when the FactStore detects this "escape," it **conservatively invalidates** the
  `captured_local` facts of every variable that block captured.
- **Target patterns:** `Thread.new`, `define_method`, `Proc.new` / `Fiber.new`, and other patterns
  that "save the block as an object / call it later."

Whether a block is "immediately invoked" or "called later" Rigor infers from RBS signature
annotations (whether `&block` is `Proc` or `yield`, etc.). When it can't decide, it treats it as an
escape, adopting the **when-in-doubt-erase** (fall to the looser side) policy.

> Also, when a block **rewrites an outer local** (`x = nil; [1,2,3].each { |i| x = i }`), its
> `captured_local` fact is invalidated too — if the block could reassign `x`, the narrowing fact is
> in jeopardy. This is treated in the main line of closure capture in Seasoned Part 6.

---

## a2-4. An ivar's type is "the union of all assignments"

The `object_content` bucket holds ivar (instance variable) types. Rigor **collects every**
assignment to `@x` within the class and takes the union of those types as `@x`'s type.

```ruby
class Foo
  def initialize
    @x = 1          # Const[1]
  end

  def reset
    @x = nil        # Const[nil]
  end

  def use
    @x              # => Integer | nil (union of all assignments)
  end
end
```

If the only place `@x` is written is `initialize`, it's `Integer`; add `reset` and it becomes
`Integer | nil`. "If `nil` could be assigned somewhere, it contains `nil` wherever you read it" —
this is conservative, but because an ivar's visibility (which methods can write it) can span files
and can't be fully traced, **the union of all visible assignments** is the safe approximation.

- **A fact is born:** from each assignment to `@x` within the class, types gather in the
  `object_content` bucket, and at a read point it's their union.
- **A fact dies:** in the pattern of initializing `@x` to `nil` and setting it right away, the
  standard move is to branch on `@x.nil?` before reading and add a `non-nil` fact (the `nil?`
  guard).

---

## a2-5. Reassignment resets narrowing

A generalization of Little Part 5's "reassignment reset." A **reassignment** to a variable resets
every fact stacked on that variable before it.

```ruby
x = find_user        # x : User | nil
if x
  # x : User (a non-nil fact was stacked)
  x = find_other     # ★ reassignment — all of x's local_binding facts vanish here
  # x : User | nil (back to find_other's return type; no narrowing memory)
end
```

Facts attach not to "the variable name" but to "facts fixed at that scope position." The moment you
write `x = something_else`, all narrowing memory about `x` vanishes, and it restarts from the new
right-hand side's type.

- **A fact dies:** `x = …` erases `x`'s `local_binding` facts (what Seasoned Part 6's stability
  calls "invalidation by reassignment").
- Note that invalidation timing differs per bucket: reassignment doubts `local_binding`, a method
  call (`obj.mutate!`) doubts `obj`'s `object_content` — it erases narrowly by target.

---

## a2-6. Why a refinement carrier is a Difference type (set difference)

Types **narrowed by a predicate**, like `non-empty-string`, `positive-int`, `literal-string`, Rigor
calls **refinement carriers.** The `s` after passing `unless s.empty?` becomes `non-empty-string` —
they arise automatically from flow facts (the value a `payload` carries in Seasoned Part 6).

This is a different concept from Little Part 1's `Const[42]` (the hyper-precise type "the value is
42"). `Const` represents *one specific value*; a refinement carrier represents *the set of values
satisfying a predicate.*

**Why a "set difference (Difference type)."** `non-empty-string` is implemented internally as
`String - ""` — "the set of `String` values with the empty string `""` subtracted" (glossary,
"`Difference` type"). "Non-empty" is nothing but "the remainder with the value-the-empty-string
*removed* from the set of all strings." Named though it is, its substance is one of the
set-theoretic type operations alongside union and intersection — **set difference.** The chibirigor
body doesn't handle it, but the answer to "why that name" for this kind of carrier is here.

> **But not every refinement carrier is a set difference.** Real Rigor is two-layered (ADR-3), and
> only a **point removal** like `non-empty-string` is a `Difference`. A **predicate subset** like
> `lowercase-string` / `numeric-string` is a separate carrier, `Refined`, and a **ranged integer**
> like `Integer[1..10]` is `IntegerRange` (the value notation is `Integer[1..10]`; `int<min,max>`
> is used in directive vocabulary and internal display). The carriers in the table below are a mix
> of these three.

- **A fact is born:** in a branch that passes a predicate guard — `unless s.empty?` / `if n > 0` / a
  `&&` chain (a2-1) — that variable's `payload` becomes a more precise refinement carrier.
- **A fact dies:** an operation invalidating the target fact — reassignment (a2-5), escape (a2-3) —
  returns it to the original coarse type (`String`, `Integer`).

### PHPStan-vocabulary correspondence

Rigor's main built-in refinement carriers and the corresponding vocabulary of PHP's checker
PHPStan. A deliberate naming correspondence to lower the learning cost by having different language
checkers express the same predicate with the same name (recap from the glossary's "refinement
carrier").

| Rigor | PHPStan | Meaning |
|---|---|---|
| `non-empty-string` | `non-empty-string` | a non-empty string |
| `numeric-string` | `numeric-string` | a string convertible to a number (`"42"`, etc.) |
| `literal-string` | `literal-string` | a string composed only of source-code literals |
| `non-empty-literal-string` | — | the intersection of the two above |
| `positive-int` | `positive-int` | an integer greater than 0 |
| `negative-int` | `negative-int` | an integer less than 0 |
| `non-zero-int` | `non-zero-int` | an integer that isn't 0 |
| `non-negative-int` | `non-negative-int` | an integer 0 or greater |
| `Integer[1..9]` (`IntegerRange`) | `int<m, n>` | a range-specified integer (e.g. `Integer[1..9]`) |
| `non-empty-array` | `non-empty-array<T>` | an array with one or more elements |
| `non-empty-hash` | — | a hash with one or more keys |
| `lowercase-string` | `lowercase-string` | a string of only ASCII lowercase |
| `uppercase-string` | — | a string of only ASCII uppercase |

### a2-6x. A note: chibirigor's `Tuple` is effectively a `non-empty-array`

The `non-empty-array` (an array with one or more elements) of the table above appears in chibirigor
too, **as a structure** — without adding a new carrier. Little Part 5's `Tuple` (an array that
remembers a type per position) is "non-empty" **determined by shape** as long as it has even one
element. Combined with the element-type read of generics 5a (Seasoned Part 3 "3-6x"), it comes out
like this:

```console
$ printf '[1, 2].first\n[].first\n' | ruby exe/chibirigor annotate /dev/stdin
1: Integer
2: untyped
```

`[1,2].first` is `Integer` (**containing no nil**). Where a general `Array[Elem]#first` would be
`Elem | nil` (because "there might be no element"), chibirigor reads from a `Tuple` (= a shape known
to be non-empty), so it doesn't mix in `nil` — this is **the same effect** as real Rigor's
`non-empty-array` refinement narrowing `first` to `Elem` (non-nil) (the origin differs — see below).
Conversely, an empty array `[]` has unknown elements and unknown non-emptiness, so `first` is
`untyped` (untyped if it can't be filled).

But chibirigor's is **a by-product of `Tuple`, not a dedicated carrier.** Real Rigor *generates* a
`non-empty-array` carrier from a flow fact like `unless arr.empty?`, and *erases* it on reassignment
or escape (the "a fact is born / dies" above). chibirigor's `Tuple` just statically holds
non-emptiness from a literal's shape — the dynamic refinement carriers born from a predicate guard
are handed to the Seasoned volume (Part 6).

---

## Summary

| Pattern | A fact is born | A fact dies |
|---|---|---|
| `&&` / `\|\|` (a2-1) | `&&` stacks sequentially from the left | `\|\|` keeps only the common ones at the merge join |
| regexp capture (a2-2) | `String` to the capture name on the match-success side | `String \| nil` on the match-failure side / outside the block |
| escaping block (a2-3) | an immediately-invoked block retains facts | invalidate `captured_local` on escape detection |
| ivar union (a2-4) | the union of all assignments into `object_content` | contains `nil` until a `nil?` guard adds `non-nil` |
| reassignment reset (a2-5) | facts into `local_binding` on guard passage | `x = …` erases all of `x`'s facts |
| refinement carrier (a2-6) | a precise `payload` on predicate-guard passage | back to the coarse type on reassignment / escape |

Every pattern runs on the core of Seasoned Part 6 — "**when in doubt, erase** (fall to the looser
side)," "narrowing only *adds* a fact," "the FactStore is immutable and flow-sensitive." Return to
[Seasoned Part 6](../seasoned/part6-fact-store.md) to confirm the main line.
