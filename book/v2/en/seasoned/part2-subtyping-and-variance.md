---
title: "Part 2 — Subtyping and variance"
description: "Rebuild the Little volume's three-valued acceptance check in the theory of subtyping `<:`, and treat covariant returns and contravariant arguments formally."
sidebar:
  order: 12
---

# The Seasoned chibirigor Part 2 — Subtyping and variance

> References (optional): TAPL ch. 15 "Subtyping," ch. 16 "Metatheory of subtyping" / 『しくみ』 ch. 7.
> This chapter rebuilds the *insides* of the Little volume's `accepts` (the three-valued acceptance
> check) in the theory of subtyping `<:`.

In Little Part 7, we built `accepts(expected, actual)` with the naive judgment "do the classes
match." In Seasoned Part 1, we revealed that this `accepts` is bidirectional typing's **checking
`⇐`**, and its true form is **subsumption** — "synthesize, then match by `<:`."

This chapter builds that `<:` (the subtyping relation) head-on. In particular, where dealing with a
function **twists the direction** — *variance* — is this chapter's climax.

---

## 2-1. A subtype is "safely assignable"

Let's define `S <: T` (S is a subtype of T) from meaning first:

> **`S <: T` means "it's safe to pass an S value everywhere a T is expected."**

Seen another way, it's **inclusion of value sets.** If "the set of values that pass as S" fits
*entirely inside* "the set of values that pass as T," then `S <: T`. The Little volume's "does it
fit in the box" was a gentle restatement of this.

There are two minimal rules: **reflexivity** (a subtype of itself) and **transitivity**:

```
  ─────────  (S-Refl)        S <: U   U <: T
  T <: T                     ─────────────────  (S-Trans)
                                  S <: T
```

And the two ends of the lattice — setting aside the Little volume's `untyped` (seen later), the
minimal `Bot` (⊥, a subtype of every type = unreachable) and the maximal `Top` (⊤, every type a
subtype of it), with `Bot <: T <: Top` holding.

```text
                 Top (⊤)           ← every type is a subtype of this (the biggest box)
              ╱    │    ╲
     {name:}   Integer   String     ← concrete types. {name:, age:} <: {name:} (extends downward by width subtyping)
              ╲    │    ╱
                 Bot (⊥)           ← a subtype of every type (unreachable; the smallest box)
```

![Figure 2-1 — the subtyping lattice](../figures/svg/seasoned-2-1.svg)
> ▼ Figure 2-1 — the subtyping lattice. Higher = "bigger type (more values pass)," lower = "smaller
> type." `S <: T` is "S below (or equal to) T."

> [!IMPORTANT]
> **The two ends are dual.** `Top` (⊤) is "**anything fits, but you can do nothing until you narrow
> it**" — the Little volume Part 1's `unknown` is this (note that `any` / `Dynamic[Top]` is not the
> top type but a "type that turns checking off"). `Bot` (⊥) is its exact opposite, "**not a single
> value, but usable for anything**" (`Bot <: T` holds for every `T`). Other languages' `never`
> (TypeScript), `!` (Rust), `Nothing` (Scala/Kotlin) are `Bot`, the type of a *non-returning*
> expression (`raise`, infinite loop) or a *branch narrowed dry* (the dead branch of Little Part 5).
> Where subsumption is the operation of "loosening upward (toward `Top`)," `Bot`, from the lattice's
> floor, "can impersonate any type" — the directions being symmetric is the lattice's beauty.
> (The deeper dive into the relation of `Top` / `Bot` and `untyped` / `void` / `never` is in
> appendix [a1](../appendix/a1-special-types.md) "Catalog of special types.")

---

## 2-2. Width / depth subtyping of objects

With the Little volume's `HashShape` (the structure of a hash), subtyping first becomes
*interesting*. Can you pass `{name: String, age: Integer}` where `{name: String}` is expected?

> You can. The extra `age` can be ignored, and the needed `name` is there.

This often runs against intuition — **the side with *more* keys is the subtype** (the smaller box).
Because the values that pass as `{name:, age:}` are a *subset* of the values that pass as `{name:}`.
There are two rules:

- **Width subtyping:** `{l_i: T_i (i∈1..n+k)} <: {l_i: T_i (i∈1..n)}` (`k ≥ 0`; at `k=0` it matches
  reflexivity) — the side with more keys is the subtype.
- **Depth subtyping:** if each key's value type is a subtype, the record is too
  (`S_i <: T_i ⇒ {l_i: S_i} <: {l_i: T_i}`). **Covariant** in the value types.

When in Little Part 6 we said "Rigor leans open (allows extra keys)," that meant **allowing this
width subtyping on the argument side.** `accepts`-over-`HashShape` just runs these two rules
recursively.

> [!TIP]
> **Reference note.** TAPL 15.2 "The subtype relation," 『しくみ』 ch. 7 implements this as a
> `subtype` function and shows the dramatic construction "subtyping enters just by swapping `typeEq`
> for `subtype`."

---

## 2-3. Function variance — returns covariant, arguments contravariant

This is the chapter's core. How is the subtyping of a function type `(A) -> B` decided? When does
`(A) -> B <: (A') -> B'` hold?

> [!NOTE]
> For Java writers: this "returns covariant, arguments contravariant" is the same story as
> generics' `? extends T` (covariant, the reading side) and `? super T` (contravariant, the writing
> side). You can *read* from `List<? extends T>` but not *write*, and *write* to `List<? super T>`
> but not *read* — that asymmetry's true form is this.

**The return (B) is covariant.** `() -> {name:, age:}` is a subtype of `() -> {name:}`. The caller
wanting the latter only reads `name` from the return value. The former always has it. So *the return
keeps the subtyping direction as is* (`B <: B'`).

**The argument (A) is contravariant.** Here the direction goes *upside down.* Can you pass
`({name:, age:}) -> Integer` where `({name:}) -> Integer` is expected? **You can't.** The expecting
side only passes `{name:}`, but the passed function tries to read `age` in its body and breaks.
Conversely, passing `({name:}) -> Integer` where `({name:, age:})` is expected is **safe** (a
function that makes fewer demands).

That is, for the argument the subtyping direction is **reversed**:

```
  A' <: A      B <: B'
  ─────────────────────  (S-Arrow)
   (A) -> B  <:  (A') -> B'
```

```text
   conditions for (A) -> B  <:  (A') -> B'

     return (covariant · → same direction):   B  <: B'
     argument (contravariant · ← reversed) :   A' <: A      ← A and A' swap
```

![Figure 2-2 — S-Arrow (covariant, contravariant)](../figures/svg/seasoned-2-2.svg)
> ▼ Figure 2-2 — S-Arrow. The return is covariant (subtyping direction as is), the argument
> contravariant (direction reversed).

That `A` and `A'` are **swapped** is the mark of contravariance. The return stays (covariant). Seen
from the subtype-function side, it settles to remember "**you may return less (narrower), you may
accept more (wider)**" (Little Part 7's robustness principle = strict on returns, lenient on
arguments = this variance itself).

> [!TIP]
> **Reference note.** 『しくみ』 ch. 7 expresses contravariance by *swapping* ty1/ty2 in the
> implementation as `subtype(ty2.params[i], ty1.params[i])`, showing "variance changing direction in
> the implementation" before your eyes. TAPL 15.2's rule S-Arrow is its grounds.

Putting §2-1–2-3 into one `subtype`, written in running Ruby, looks like this (types: base =
`Symbol`, record = `Obj`, function = `Arrow`, `:Top`):

```ruby
# s <: t ?  = "is it safe to pass s where t is wanted"
def subtype(s, t)
  return true if t == TOP

  case [s, t]
  in [Symbol, Symbol] then s == t # base types: reflexivity only (no non-trivial subtyping)
  in [Obj, Obj] # width + depth: each key of t is in s, values covariant
    t.fields.all? { |k, tv| s.fields.key?(k) && subtype(s.fields[k], tv) }
  in [Arrow, Arrow] # arguments contravariant, return covariant
    s.params.size == t.params.size &&
      s.params.zip(t.params).all? { |sp, tp| subtype(tp, sp) } && # ★ tp/sp swapped = contravariance
      subtype(s.ret, t.ret)
  else false
  end
end
```

The `subtype(tp, sp)` (with `sp` and `tp` swapped) on the `Arrow` line is argument contravariance
itself. In the standalone design sketch [`examples/subtype.rb`](examples/subtype.rb),
`ruby subtype.rb` goes **green** like this (`arg contravariant reverse is false` is the proof of
contravariance):

```text
PASS: width: {name,age} <: {name}
PASS: width: {name} </: {name,age}
PASS: depth: {p:{a,b}} <: {p:{a}}
PASS: arg contravariant: ({name})->Num <: ({name,age})->Num
PASS: arg contravariant reverse is false
PASS: ret covariant: ()->{name,age} <: ()->{name}
PASS: Num <: Top
PASS: Num </: Bool
```

### 2-3a. The robustness principle and LSP arrive at "the same rule"

Little Part 7 §7-5's **robustness principle** ("strict on what you return, lenient on what you
accept") and S-Arrow's "returns covariant, arguments contravariant" derive the same rule from two
entirely different starting points.

**Route A — from substitutability (TAPL ch. 15 / LSP)**

> "Replacing an S value where a T is expected is safe" = the set of arguments the caller can pass
> must be *wider* (A' <: A), and the set of returned values *narrower* (B <: B').

This is S-Arrow's rule itself. Barbara Liskov's "substitution principle (LSP)" follows the same
derivation, known as the OO-design rule "an override may widen arguments, may narrow the return."

**Route B — from a practical complaint (Rigor's robustness principle)**

> "I don't want to be forced to write every type conversion just to confirm `.to_s` exists."
> "I want to protect the precision for whoever uses a method's return value."

The rule Rigor worked backward from these two *practical complaints* is "arguments lenient, returns
strict," which matches S-Arrow. It reached the same form from real-world experience without
passing through a mathematical proof of substitutability.

```text
    S-Arrow (type theory)     LSP (OO design)          robustness principle (Rigor)
          ↓                         ↓                          ↓
    args contravariant,       widen args,              args lenient,
    returns covariant         narrow returns           returns strict
          └─────────────────── the same rule ──────────────────────┘
```

This convergence is no accident. The safety demand "won't break when replaced" arrives at the
asymmetry of variance, wherever the starting point.

> [!NOTE]
> **Reference.** Little Part 7 §7-5 introduced the robustness principle as a practical matter. Having
> derived S-Arrow here, the theoretical answer to that "why is it so" is now in place.

---

## 2-4. Variance of data structures — covariant on read, contravariant on write

In a mutable container, variance is decided by *the operation's direction.* Is `Array[S]` a subtype
of `Array[T]`?

- **Read only**, covariant: a value taken from `Array[Cat]` can be used as `Animal` (`Cat <:
  Animal`).
- **Write**, contravariant: you can put `Dog` into `Array[Animal]`, but not `Dog` into `Array[Cat]`.
- **Both read and write**, **invariant**: `Array[S] <: Array[T]` only when `S = T`.

The Little volume handled arrays as `Tuple` (per-position, read-centric), so covariance sufficed.
Handle mutable arrays seriously in the Seasoned volume and this invariance comes into effect.

> [!NOTE]
> **Implementation note.** "Covariant on read, contravariant on write, invariant on both" is a
> *design direction*, and Rigor's current implementation (`lib/rigor/inference/acceptance.rb`)
> processes a Nominal's type arguments **uniformly covariantly.** Declaration-site variance
> (reflecting an RBS variance declaration like `Array[+Elem]` in inference) is an unimplemented item
> from Slice 5 on (designed). The element covariance of `Tuple` / `HashShape` is as in the current
> implementation.

---

## 2-5. Three-valued `<:` and `untyped` — gradual's guarantee goes to Seasoned Part 7

The Little volume's `accepts` was three-valued `:yes/:no/:maybe`. Once `untyped` (`Dynamic`) is
involved, `<:` becomes a special relation that passes both ways — **gradual consistency** `~` — and
the three values were the contrivance for folding it. Where `Top` / `Bot` were "*types* at the two
ends of the lattice," `untyped` differs decisively in that it "turns off the `<:` judgment itself"
(→ appendix [a1](../appendix/a1-special-types.md)).

This `~`'s symmetry and non-transitivity, and the design guarantee (the gradual guarantee) by which
gradual typing takes on being **deliberately unsound**, are consolidated together with the soundness
discussion into **Seasoned Part 7.** This chapter purifies to variance and stays at a one-line
pointer here.

> [!NOTE]
> **Sorbet's `T.assert_type!` also stands on gradual-consistency ground**
>
> Sorbet (Stripe's Ruby type checker) has an API `T.assert_type!(expr, T::Integer)`. It's mainly a
> **static assertion**, fixing the type statically and tightening subsequent inference. If static
> analysis can't determine that the types fit (when `untyped` is involved, etc.), it also type-checks
> at run time. In terms of Rigor's three-valued `accepts`: pass statically if definitely fits
> (`:yes`-equivalent), reject statically if definitely doesn't (`:no`-equivalent), defer to run time
> if uncertain (`:maybe`-equivalent) — continuous with the thinking of gradual consistency (note:
> `:yes/:maybe/:no` is Rigor's vocabulary, not Sorbet's official term). `T.let`/`T.cast` are
> separate, more runtime-leaning APIs.
>
> Rigor has no "runtime confirmation" (a static-checker specialist), so the combination of
> complementing a Sorbet `:maybe`-equivalent spot with `T.assert_type!` can be said to have good
> affinity. (`accepts` in full is Seasoned Part 7.)

---

## 2-6. Algorithmic subtyping — from rules to a decision procedure

Theory's `<:` is "a collection of rules," but `accepts` is a *program.* The bridge between them is
**algorithmic subtyping** (TAPL ch. 16).

The declarative rules have reflexivity and transitivity, and "which rule to use when" isn't uniquely
determined (transitivity can be inserted anywhere). So we rework the rules so that **for each type
shape there's exactly one applicable rule** (*absorbing* reflexivity and transitivity into the
structural rules), and prove it's *equivalent* to the declarative system. That the Little volume's
`accepts` was "a single function that `case`-branches on the shape of `expected`/`actual`" is because
we wrote this **algorithmic** side from the start.

---

## 2-7. Inside Rigor

- **The `<:` relation:** Rigor has subtyping `<:` (value-set inclusion, reflexive and transitive,
  `Bot <: T <: Top`), and `accepts` puts three-valued / gradual consistency on top of it.
- **Variance:** the current implementation processes a Nominal's type arguments uniformly
  covariantly (declaration-site variance is designed, unimplemented). `Tuple` / `HashShape` run the
  acceptance check covariantly per element and per key. The function (proc) type itself is erased to
  the nominal `Proc` (it has no first-class function subtyping). Covariant returns / contravariant
  arguments (S-Arrow) are implemented as the **method override-compatibility check** (ADR-35).
- **join/meet:** where the Little volume escaped to `Union` at an `if` branch — the common upper
  bound (join) / lower bound (meet) — Rigor computes daily as normalization (`Combinator.union`).
  Where 『しくみ』 ch. 7 settled for "remove `if` because you'd need join/meet," Rigor implements it
  head-on.

---

## 2-8. Summary

- The insides of `accepts` are subtyping `<:`. Its meaning is "safely assignable = inclusion of
  value sets."
- Records are width/depth subtyping (more keys = subtype / values covariant).
- Functions are **covariant returns, contravariant arguments** — variance twists the direction (the
  true form of the robustness principle).
- Mutable containers are covariant on read, contravariant on write, invariant on both.
- Add `untyped` and `<:` becomes gradual consistency `~` (symmetric, non-transitive), folded into
  three-valued `accepts` (the guarantee in full is Seasoned Part 7).

## Exercises

1. **Derive contravariance from S-Arrow:** show that `({name:}) -> Integer <: ({name:, age:}) ->
   Integer` holds, by applying it to S-Arrow's premises (`A' <: A` and `B <: B'`). With a phrase for
   why the reverse (`({name:, age:}) -> ... <: ({name:}) -> ...`) doesn't hold.
2. **One width and one depth:** make one pair each of a width-subtyping example and a depth-subtyping
   example, each with concrete `HashShape`s (you may add them to `subtype.rb`'s self-check and
   confirm it goes green).
3. **A non-transitive example of gradual consistency:** show that `Integer ~ untyped` and `untyped ~
   String` yet not `Integer ~ String`, and state what would break if `~` were transitive.

**Next chapter (Part 3):** having tidied "the type box" with subtyping, now we punch *holes* in the
box. Type abstraction `<T>` and type application = **generics** (System F). We avoid the shadowing
and variable capture that naive type substitution `subst` falls into, with α-conversion (TAPL
ch. 23). To the true form of what we traced with RBS type variables in Little Part 8.

---

> **This chapter's design sketch** → [`examples/subtype.rb`](examples/subtype.rb) (self-checks with `ruby subtype.rb`)
