---
title: Part 5 — Real type inference: filling in arguments
description: "Treat the constraint-based inference that derives the argument types the Little volume fell back to `untyped`, from how they're used in the body."
sidebar:
  order: 15
---

# The Seasoned chibirigor Part 5 — Real type inference: filling in arguments

> References (optional): TAPL ch. 22 "Type reconstruction" / 『しくみ』 ch. 9 exercises, afterword.
> This chapter derives the *argument* types the Little volume fell back to `untyped`, from how they're
> used in the body — the frontier of inference, where 『しくみ』 gave up the answer as "we don't know
> the solution."

In Part 4 we treated recursive types, where a type references *itself*. From here we change
direction and go to fill in the arguments we *fell back to untyped* in the Little volume.

In Little Part 8, a method's **return type** could be synthesized from the body
(`def shout; "hi".upcase; end` → `() -> String`). But the **argument** stayed `untyped` —
`def double(n); n * 2; end` was `(untyped) -> untyped`.

This chapter goes to fill in that `untyped`. This is the biggest mountain we *deliberately avoided*
in the Little volume, and the doorway to real type inference.

---

## 5-1. Why arguments are hard

The return type "comes out by synthesizing (`⇒`) the body" — one direction, bottom-up. Arguments are
the reverse. `n`'s type is known only by gathering **how `n` is used** in the body. Seeing `n * 2`,
you need to *work backward* to "`n` is something you can pass an integer to with `*`." This runs
against synthesis `⇒`'s straightforward flow, so a different tool is needed.

There are broadly two roads. **(A) Gather capabilities from usage** (capability/duck inference), and
**(B) Place type variables and solve constraints** (constraint-based = TAPL ch. 22's type
reconstruction). The Seasoned volume treats (A) mainly, (B) only at a beginner's level.

---

## 5-2. Road A — gather "capabilities" from usage

Ruby is a duck-typing language. More essential than "what class is `n`" is "what can `n` **respond
to**." So we infer arguments not as concrete types but as **capabilities (interfaces).**

We sweep the body once and gather the messages sent to that argument:

```ruby
def greet(user)
  "Hello, " + user.name        # .name on user (→ something returning String)
end
# capabilities gathered: { name: () -> String }
# inference: user : something satisfying { name: () -> String } (a structural interface)
```

This is a **structural type** continuous with Little Part 6's `HashShape`, corresponding in Rigor to
**capability roles** (`_ToS`, `_Each[T]`, and other capability roles). Receiving by "does it respond
to `name`" rather than "is it `String`," it can type without *breaking* duck typing.

**The crux of FP safety:** gather only "messages definitely sent." Messages that are **doubtful to be
sent** — deep inside a conditional, inside a `respond_to?` guard — are *not added* to the
capabilities (when in doubt, don't demand). Demand too much and you get a false positive that rejects
a working call.

---

## 5-3. Road B — type variables and constraints (the basics of type reconstruction)

Going one step further, place **type variables**, gather **constraints**, and solve them together.
This is TAPL ch. 22's **type reconstruction**, the gist of so-called Hindley–Milner.

```ruby
def id(x)
  x            # let x's type be an unknown type variable X
end
# constraint: (the body just returns x) → no constraint
# solution: id : (X) -> X   (= generic! the proper path of the type variable built in Part 3)
```

```ruby
def inc(n)
  n + 1        # n can be passed to Integer#+ → constraint X <: (accepts Integer#+'s argument)
end
# solution: n is narrowed toward Integer
```

The procedure is three stages: **(1) assign a type variable to each unknown**, **(2) walk the body
and gather constraints (equations, subtyping)**, **(3) solve with unification** (= looking at the
gathered equations, find the type-variable assignment that lets two types be "the same"). If no
constraint remains, as for `id`, that type variable becomes *generic as-is* (`(X) -> X`) — the type
variables and substitution used here are the ones built in Part 3.

The standalone minimal sketch making this "gather constraints and solve by unification" core run is
[`examples/unification.rb`](examples/unification.rb). In a world of just type variables `TVar` and
base types `TCon`, unification is only this:

```ruby
# follow type through the substitution subst, resolving until it can't be followed further
def resolve(type, subst)
  type.is_a?(TVar) && subst.key?(type.name) ? resolve(subst[type.name], subst) : type
end

class UnifyError < StandardError; end

# return the substitution making a and b equal (UnifyError if impossible)
# note: this sketch omits the occurs-check (a self-reference like unify(X, X->X) would pass).
# in a TVar/TCon-only world we don't build function types, so the self-check stays green, but
# in real HM the occurs-check is essential to termination and soundness.
def unify(a, b, subst)
  a = resolve(a, subst)
  b = resolve(b, subst)
  return subst if a == b
  return subst.merge(a.name => b) if a.is_a?(TVar) # bind variable a to b
  return subst.merge(b.name => a) if b.is_a?(TVar) # bind variable b to a

  raise UnifyError, "#{a.name} and #{b.name} don't match"
end

# unify the constraints (= pairs of types to make equal) in order
def solve(constraints)
  constraints.reduce({}) { |subst, (a, b)| unify(a, b, subst) }
end
```

With `ruby unification.rb`, that `id` has zero constraints → `X` stays free (generic), `inc` resolves
to `N = Integer`, and a conflicting constraint becomes `UnifyError` go **green**:

```text
PASS: id has no constraint (X stays generic)
PASS: inc resolves N to Integer from n + 1
PASS: conflicting constraints raise UnifyError
```

> [!TIP]
> **Reference note.** TAPL ch. 22 develops constraint generation and unification carefully. What we
> build here is up to "unification + constraints," and doesn't step into **ML's `let`-polymorphism**
> (the HM-specific mechanism of generalizing the type per binding) — a matter involving the
> generalization of universal types (∀), which this book treated in Part 3. 『しくみ』 doesn't step
> into this inference either, leaving it as an exercise for the reader in ch. 9 — the Seasoned volume
> fills it in here.

---

## 5-4. Why we don't do "all of it"

Full HM inference solves types globally with *zero annotations*, like ML. Why don't chibirigor (and
Rigor) go that far? Three reasons:

1. **Determinism and speed:** global unification tends to explode in constraints across methods. At
   real-code scale, it's expensive.
2. **False positives:** the stronger you infer, the more you hit "works but has no type" code and
   emit `:no`. Against Rigor's highest value (don't frighten).
3. **The boundary exists:** Ruby has RBS, a *boundary of declaration.* Rather than filling everything
   with inference, "look up where there are types (RBS), infer modestly where there aren't, and
   escape to `untyped` at the end" fits reality.

So the policy is **"infer only the obvious range."** An identity like `id` is `(X) -> X`, an obvious
usage like `n + 1` is narrowed by capability — but if doubtful, it stays `untyped`. This is the same
stance as 『しくみ』 ch. 9's exercise advice ("it's good to limit inference to the obvious cases").

> [!NOTE]
> **There isn't just one reason HM can't be used**
>
> To "why not use full HM," you can answer "because it's slow" or "because false positives increase,"
> but in the language of type theory, three *separate problems* each require a separate remedy.
>
> **① The decidability problem**
> Here "rank" means how deeply polymorphism `∀` nests on a function's *argument* side (rank 1 = `∀`
> only at the head, the HM world; it rises as `∀` dives into the argument's argument and so on).
> HM-proper (rank 1) inference is decidable, up to rank 2 is decidable, and **polymorphic type
> inference at rank 3 or above is undecidable** (Kfoury–Wells 1994). Note that Wells 1994/1999 showed
> that typeability of *unrestricted* System F itself, regardless of rank, is undecidable. Trying to
> step beyond HM into higher-order polymorphism, you enter the realm "this inference fundamentally
> doesn't terminate." Remedy: restrict the use of type variables (stay at rank 1, narrow to local),
> or don't even ask, with `untyped`.
>
> **② The reachability problem**
> Ruby has many structures where the AST alone can't read "what gets defined" — `define_method`,
> `method_missing`, macro expansion. However many constraints you gather, you can't find the type of
> a method whose existence you don't know. Remedy: plugins (ADR-2, 16) supplement "knowledge outside
> the AST."
>
> **③ The precision problem**
> A blunt join (confluence of types) produces wide types. After seeing both `foo(1)` and `foo("a")`,
> joining the argument to `Integer | String` is needlessly wide — because actually "the callers that
> pass only one or the other are separate." Remedy: keep meet/join minimal, degrade the rest to
> `untyped`.
>
> chibirigor's judgment "arguments are `untyped`" is the most conservative answer that avoids all
> three at once. Rigor narrowing its inference to "local + catalog" is the same answer to the same
> three problems.

---

## 5-4a. Comparison with TypeProf — whole-program vs. local+catalog

To sharpen the answer to "why not do all of it," let's compare with Ruby's official type inference
tool, **TypeProf.** TypeProf is a type-level abstract interpreter bundled with Ruby core, designed by
*mametter (TypeProf's author)* (details: the Rigor handbook, appendix-typeprof).

> [!NOTE]
> **TypeProf's way — whole-program abstract interpretation**
>
> TypeProf "executes at the type level" from the program's entry. Finding a method call, it chases
> the argument's abstract type *from the caller*, interprets the callee's body, and works backward to
> the return type. By repeating this, it can generate RBS like `def foo: (Integer) -> String` from
> zero-annotation code. Its greatest strength is that, looking at callers, it can **infer argument
> types from the call site.**

The *direction* of what it does (raising argument types from the caller) is the same as the previous
section's "Road B," but TypeProf's *way* isn't unification of type variables + constraints; it's
**abstract execution of the program at the type level** (a separate lineage from HM's unification).
Interpreting the whole program at once, it tends to combinatorially explode at scale, and TypeProf
itself is positioned as "for small files, prototype use."

> [!NOTE]
> **Rigor's way — local+catalog**
>
> Rigor infers **one method at a time.** Where it calls another method, rather than re-interpreting
> the body, it looks up the return type from a prepared **catalog** (the RBS of core, stdlib, gems,
> and plugin contributions). An unsolvable argument it drops to `Dynamic[Top]` (= `untyped`) and cuts
> off, securing a cost ceiling with an **inference budget** (Part 6's cache works too). Its goal is
> to run on the whole codebase every push, so it prioritizes scale above all.

| | TypeProf | Rigor |
|---|---|---|
| Unit of analysis | the whole program (from the entry) | one method at a time |
| Other methods' return type | re-interpret the body | reference the catalog |
| Infer argument types from the call site? | **can** (its greatest strength) | doesn't (`untyped` is the default) |
| Scale goal | small files, prototypes | the whole codebase, CI always-on |
| Main output | RBS signatures (errors are a by-product) | diagnostics (only certain bugs; FP-zero-ism) |

When the previous section said "keep argument inference to the obvious range," this comparison is the
reason. TypeProf's whole-program method is high-precision but weak at scale. Rigor prioritizes scale
with local+catalog, and honestly makes the unsolvable `untyped` — **two answers to the same "infer
arguments" challenge, solved with different values: scale, and FP-zero.**

Note that `rigor sig-gen` (Chapter 11) does the same job as TypeProf (generate RBS from zero
annotations), but its handling of argument types differs in the policy "don't make an observed call
site the default" (ADR-5) — where TypeProf emits the types it saw as-is, Rigor doesn't fix "this
argument only accepts that," leaving it `untyped` for the human.

---

## 5-5. Seen bidirectionally

In Part 1's map, argument inference is the work of **widening synthesis `⇒`'s coverage.** The Little
volume gave up on `⇒` at arguments and returned `untyped`. The Seasoned volume uses the body's usage
(Road A) or constraints (Road B) to fill in that `⇒` *as far as it can*. Where it can't fill, honestly
`untyped` — the bidirectional framework stays the same, only `⇒`'s precision rises.

And the important thing is that **precision rising doesn't increase diagnostics.** Even if an argument
type narrows from `untyped` to `{name: () -> String}`, that's a matter of *the synthesis side.* A
diagnostic still appears only at a checking `⇐` position (where there's an RBS declaration) — the
"don't frighten" structure seen in Part 1 doesn't break when you add inference.

---

## 5-6. Inside Rigor

- **capability role:** capability roles like `_ToS`, `_Each[T]`, `_Reader` are the substance of
  Road A's "gathered capabilities." Used in the acceptance check as structural interfaces.
- **Generics specialization:** at the call site, bind type variables from the actual arguments
  (a limited version of Road B's unification). Solving type variables from a block's return type, etc.
- **fail-soft:** an unsolvable type variable degrades to `Dynamic[Top]` (= `untyped`). Same as the
  Little volume's "untyped if it can't be filled."

---

## 5-6x. A note: push-down to a block parameter entered lib (generics 5b · 5c)

Road B's core — flowing the element type `Elem` into a block parameter — has been promoted into the
chibirigor body (`type_of_block` in `lib/chibirigor/type_of.rb`). With the **push-down (5b)** that
follows Seasoned Part 3 "3-6x"'s **read (5a)**, generics' read and push-down become one continuous
thing in lib (the full unification that solves the general case where the element is an *unknown type
variable* = §5-3 is still a design sketch):

```console
$ printf '[1, 2].map { |x| x + 1 }\n[1, 2].map { |x| x.to_s }\n[1, 2].select { |x| x }\n' | ruby exe/chibirigor annotate /dev/stdin
1: Array[Integer]
2: Array[String]
3: Array[Integer]
```

`map`'s block parameter `x` is bound to the **element type `Integer`**, and since the body `x.to_s`
is `String`, `map`'s return is `Array[String]` (the return is an element-typed array too = **return
polymorphism**, which is 5c). The block body is **type-checked** under `x : Elem` — so
`[1,2].map { |x| x + true }` emits one "can't add true to an Integer" (proof the push-down works).
`each` returns the receiver (self), and `select`/`reject` keep the element type.

What we used here is **direct substitution, not unification.** chibirigor's arrays have a *concrete*
element type (`Tuple` or `Array[Elem]`), so `x := Elem` is exactly §5-3's unification's **trivial
special case** — corresponding to the case where the constraint is just one `[[X, Integer]]` (the RHS
`Elem` is ground = contains no free variable, so unification *degenerates to substitution*). Since the
binding is decided without even calling `solve`, we settled it with direct substitution, adding no
plumbing (minimal, budget-first). `examples/unification.rb`'s **full constraint-solving** is needed
for the general case where the element type is an *unknown type variable* and you *work it backward*
from the block's usage — that's left as a design sketch (the "don't do all of it" judgment of Road
A/B's §5-4).

Zero false positives stays invariant too: an empty array `[].map { ... }` has `untyped` elements so
the body is `untyped` too, and an unknown receiver `foo.map { ... }` "doesn't presume it's an array,"
so it doesn't check the body and falls back to `untyped`.

## 5-7. Summary

- The return type comes out by synthesis, but arguments need "working backward from usage" — the hard
  spot of inference.
- Road A: gather capabilities (capability/duck) into a structural interface. Only certain messages.
- Road B: type variables + constraints + unification (the gist of type reconstruction / HM). A
  remaining variable becomes generic.
- **Don't do all of it:** only the obvious range. If doubtful, `untyped` (for determinism, false
  positives, the RBS boundary). A deliberate choice differing in design from TypeProf (whole-program
  abstract interpretation; argument inference from the call site).
- Adding inference, diagnostics still appear only at a checking position — the don't-frighten
  structure is invariant.

## Exercises

1. **Trace unification:** trace `solve([[X, Integer], [Y, X]])` in `examples/unification.rb` by hand
   and write the final `subst` (what `X` and `Y` each resolve to).
2. **Gather capabilities with Road A:** from `def f(x); x.name + "!"; end`, write out the capabilities
   (the structural interface) demanded of `x`. With a phrase for the constraint on `x.name`'s return.
3. **Why not do all of it:** state how the diagnostic for the call `g(foo.bar)` differs between
   "solving the type with global HM" and "toppling the argument to `untyped`" for `def g(x); x; end`
   (from the false-positive angle).

**Next chapter (Part 6):** we extend to the complete **FactStore** that stores the facts stacked by
narrowing and carries them around while *invalidating* them on reassignment and side effects. With
six buckets, stability, and join, we implement flow-sensitive narrowing.

---

> **This chapter's design sketch** → [`examples/unification.rb`](examples/unification.rb) (self-checks with `ruby unification.rb`)
