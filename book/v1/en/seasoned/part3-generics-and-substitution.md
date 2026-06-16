---
title: Part 3 — Generics and type substitution
description: Rebuild the type-variable substitution traced in the Little volume properly, and treat α-equivalence, variable capture, and erasure formally.
sidebar:
  order: 13
---

# The Seasoned chibirigor Part 3 — Generics and type substitution

> References (optional): TAPL ch. 22 "Type reconstruction," ch. 23 "Universal types (System F)" /
> 『しくみ』 ch. 9. This chapter rebuilds properly the type-variable substitution we *traced* with
> RBS in Little Part 8.

In the last chapter (Part 2), with **subtyping** like `Cat <: Animal`, we saw the relation "one type
can be used in place of another." This chapter goes one step further and **punches holes** in the
type itself — making the contents swappable later. In Little Part 8 we read types like `Array[Elem]`
from RBS. But the substitution "put `String` into `Elem` to make `Array[String]`" we only touched at
the simplest spots. This chapter builds that **substitution** head-on. The climax is the two pitfalls
where substitution quietly breaks — **shadowing** and **variable capture.**

---

## 3-1. Type abstraction and type application

Generics are a mechanism that **abstracts** a type (opens a hole) and **applies** it where used
(fills the hole).

```ruby
def select(cond, a, b) = cond ? a : b
# type: <T>(bool, T, T) -> T      ← <T> is type abstraction (a type hole)
# select<Integer>(...)            ← type application (fill the hole with Integer)
```

Putting `<T>` is **type abstraction**, and inserting a concrete type like `select<Integer>` is
**type application.** These are the very terms of System F in TAPL ch. 23. The contents of type
application are — strip `<T>`, and replace *every* `T` in the body with the concrete type, i.e.
**substitution.**

---

## 3-2. The naive `subst`, and its pitfalls

Writing `subst(ty, X, repl)` (substitute `repl` for type variable `X` in type `ty`) naively:

```
subst(Nominal[C, args], X, repl) = Nominal[C, args.map { subst(_, X, repl) }]
subst(TypeVar[name],    X, repl) = name == X ? repl : TypeVar[name]
subst(TypeAbs[params, body], X, repl) = TypeAbs[params, subst(body, X, repl)]   # ← here's the trap
```

Diving **unconditionally** under the last `TypeAbs` (an inner `<...>`) is the mistake. Two pitfalls:

**(1) Shadowing:** when an inner `<T>` *hides* an outer `T`, the inner `T` is a different thing and
must not be substituted. First, let's separate out that **type application is two stages** (an
easily-crossed spot):

> A note on notation: in the examples ahead, we write a function (lambda) as `=> body` (the same as
> TypeScript's arrow function). §3-1's `->` is *a function's type*, `=>` is the arrow of *the
> function itself* — neither has anything to do with Ruby Hash's `=>` (rocket). Read
> `(arg1: T, …) => true` as "a lambda taking arguments and returning `true`."

```ruby
# The type application (<T>BODY)<Integer> is two stages:
#   Stage 1 [apply]   strip the outer <T>, decide "substitute T:=Integer into BODY."
#   Stage 2 [subst]   run subst(BODY, T, Integer).
#
# When BODY = (arg1: T, arg2: <T>(x: T) => bool) => true, stage 2 is:
#   - arg1's T          → substitute to Integer (the outer T)
#   - arg2's inner <T> … → stop here. the inner <T> is a *different* T (shadowing), so don't touch inside
# Result: (arg1: Integer, arg2: <T>(x: T) => bool) => true
```

Separating Stage 1 (apply, strip the outer) and Stage 2 (`subst` into the body) makes the rule and
the example agree. Stage 2's stop condition is this —
→ **if `TypeAbs.params` contains the substitution target `X`, return without substituting inside that
abstraction.**

**(2) Variable capture:** when the substituting type `repl` contains a *free variable* that *happens
to have the same name* as an inner bound variable, two different things get **captured** into one.

```ruby
# foo = <T>(arg1: T, arg2: <U>(x: T, y: U) => bool) => true
# bar = <U>() => foo<U>     ← apply T := U
# naive: (arg1: U, arg2: <U>(x: U, y: U) => bool)  ← bar's U and arg2's U get crossed!
```

`bar`-derived `U` and `arg2`'s bound `U` collapse into the same `U`. This is the capture bug.

---

## 3-3. Avoid capture with fresh variables

The fix for capture is, **before substituting, rename the inner bound variables to *brand-new
names*** (α-conversion):

```
freshTypeAbs(params, body):
  rename each param to a unique new name param@n (update body via subst too)
  → make collision-proof names, then do the outer substitution
```

Using a *character a programmer can't write* like `@` (or `#`) plus a counter, we make non-colliding
names. In running Ruby, the `subst` with shadowing and capture-avoidance built in looks like this
(`TypeAbs` is the type abstraction `<...>`):

```ruby
# substitute repl for type variable x in type ty.
def subst(ty, x, repl)
  case ty
  in Symbol then ty
  in Var then ty.name == x ? repl : ty
  in Arrow then Arrow.new(ty.params.map { subst(it, x, repl) }, subst(ty.ret, x, repl))
  in TypeAbs
    return ty if ty.params.include?(x) # shadowing → don't substitute inside that abstraction

    body = ty.body
    new_params = ty.params.map do |p| # α-rename the bound variables to fresh first…
      np = fresh_name(p)
      body = subst(body, p, Var.new(np))
      np
    end
    TypeAbs.new(new_params, subst(body, x, repl)) # …then the outer substitution (no capture occurs)
  end
end
```

The `TypeAbs` clause is the heart of this chapter — (①) if `params.include?(x)` it's shadowing, so
return without substituting; (②) otherwise rename the bound variables with `fresh_name`
(`:"#{p}@#{n}"`), then (③) substitute the outer. In the standalone design sketch
[`examples/subst.rb`](examples/subst.rb), the three points go **green**:

```text
PASS: shadowing leaves the inner T untouched
PASS: non-shadowing substitutes T and freshens U
PASS: capture is avoided (inner U becomes U@1, distinct from the substituted U)
```

In concrete terms, the verified behavior: `subst(<T>(T)->Bool, T:=Num)` is `<T>(T) -> Bool`
*unchanged* (① shadowing); `subst(<U>(T,U)->Bool, T:=Num)` is `<U@1>(Num, U@1) -> Bool` (② freshen
U); and the capture example `subst(foo_body, T:=U)` is `(U, <U@1>(U, U@1) -> Bool) -> Bool` — the
leading `U` (`bar`-derived) and the inner `U@1` are kept distinct (③ capture-avoidance).

The last case is capture-avoidance itself — `bar`-derived `U` and the different thing made by
renaming `arg2`'s inner `<U>` to `U@1` are kept without crossing.

> **Reference note.** 『しくみ』 ch. 9 first shows a *wrong* `subst` (`poly_bug.ts`), draws out
> shadowing and capture with concrete examples, and fixes it with `freshTypeAbs` — this chapter's
> structure follows it closely. TAPL ch. 23 gives the theory of System F's substitution and
> α-conversion.

---

## 3-4. Equivalence under type variables — α-equivalence

`<A>(x: A) => A` and `<B>(x: B) => B` are **the same type** (only the bound-variable names differ).
This is called **α-equivalence.** The equivalence test is solved by carrying around a **name
correspondence table**:

```
typeEq(TypeAbs[p1, b1], TypeAbs[p2, b2], map):
  pair p1[i] with p2[i] and add to map, compare b1 and b2
typeEq(TypeVar[n1], TypeVar[n2], map):
  map[n1] == n2     # translate n1 via the map, then compare
```

This technique of "holding the bound-variable name correspondence in a map" appears in *the same
form* in the next chapter Part 4's equivalence test for recursive types — α-equivalence and
recursive-type α-equivalence are one at the root.

---

## 3-5. erasure — erase the types and return to RBS

Type application **does nothing at run time.** `select<Integer>` is just `select` at run time. TAPL
23.7's *erasure theorem* guarantees "erasing type annotations and type applications doesn't change
the execution result" (a different thing from Java generics' "type erasure" — that's an
implementation technique for dropping types from generated code, this is a *semantic* theorem).

For Rigor, erasure has one more meaning — the operation of **conservatively returning the rich
internal type to RBS.** `HashShape` returns to an RBS record or `Hash[K,V]`, a literal union to its
base class, `Dynamic[T]` to `untyped`. **It may get wider, but never narrower** (a sound
approximation). This is the substance of "Rigor is a superset of RBS, writable back to RBS at any
time."

---

## 3-6. Inside Rigor

- **Substitution:** `RbsTypeTranslator.translate(..., type_vars:)` is the proper `subst`. With
  `type_vars[:Elem]=String`, `Array[Elem]` → `Array[String]`. Being RBS-derived, it's less exposed
  to general nested type abstraction than 『しくみ』 ch. 9, so **the capture exposure is small** (but
  the idea is the same α-conversion).
- **Bounded quantification (`X extends T`):** TAPL ch. 26. In Rigor, conformance to a structural
  contract (an interface/capability role) carries part of that role.
- **erasure:** `Type#erase_to_rbs`. A conservative conversion to RBS on export.

---

## 3-6x. A note: reading the element type entered lib (generics 5a)

> Lib-ifying generics proceeds in three stages — **5a = reading the element type / 5b = pushing down
> to a block parameter / 5c = return polymorphism.** This section is **5a**; the following 5b/5c are
> treated in Seasoned Part 5 "5-6x."

So far we've seen "how to substitute `Elem` in `Array[Elem]`" with concept and sketch (`subst.rb`).
Its **doorway** — *reading out* `Elem` from a known array — has been promoted into the chibirigor
body (`element_read` in `lib/chibirigor/type_of.rb`). It works with `annotate`/`check` with no
special flag:

```console
$ printf '[1, 2, 3].first\n{ a: 1, b: 2 }.values\n[].first\n' | ruby exe/chibirigor annotate /dev/stdin
1: Integer
2: Integer
3: untyped
```

`[1,2,3].first` is `Integer`, `{a:1,b:2}.values` is `Integer` — it can **read the element type
`Elem`** (literal precision `Const` is rounded to a class here. Not `1` but `Integer` = the
abstraction "the element type"). A non-literal index `a[i]` returns the element type too (because the
position is unknown). A literal index `a[0]`, on the other hand, keeps Little Part 5's **per-position
precision** (Tuple read) — choosing *read by position* vs. *read by element type*.

It keeps zero false positives too: **an empty array `[].first` or an unknown receiver `foo.first` is
`untyped`** (untyped if it can't be filled). The element type read flows into checking too —
`a = [1,2]; a.first + true` emits one "can't add true to an Integer."

What lib gained here is generics' **read (5a).** The following **push-down (5b)** — flowing the
element type into a block parameter (making `map { |x| ... }`'s `x` be `Elem`) — entered lib too;
the details, a worked example, and "why direct substitution suffices instead of unification" go to
Seasoned Part 5 "5-6x." The **substitution `subst`** that led this chapter, and the **full
unification** ([`examples/unification.rb`](examples/unification.rb)) that solves the general case
where the element is an *unknown type variable*, are left as design sketches.

## 3-7. Summary

- Generics = type abstraction (`<T>`) + type application (fill the hole) = substitution (System F,
  TAPL ch. 23).
- The naive `subst` breaks on **shadowing** (an inner same-name is a different thing) and **variable
  capture** (a free-variable collision).
- The fix: shadowing is "if it contains it, return without substituting"; capture is **α-convert with
  fresh variables, then substitute.**
- Equivalence under type variables is **α-equivalence** (a name correspondence table) — the same
  technique as recursive types.
- **erasure:** type application vanishes at run time / Rigor can return internal types
  conservatively to RBS.

## Exercises

1. **Trace the capture:** trace `subst.rb`'s capture example `subst(foo_body, T:=U)` by hand
   **without using fresh variables**, and show at which stage `bar`'s `U` and `arg2`'s `U` collapse
   into the same name.
2. **α-equivalence:** show that `<A>(x: A) -> A` and `<B>(x: B) -> B` are equal, step by step using
   the name correspondence table `map` (what to add to `map`).
3. **The two stages of shadowing:** evaluate
   `(<T>(arg1: T, arg2: <T>(x:T)=>bool)=>true)<Integer>` along §3-2's "Stage 1 [apply] → Stage 2
   [subst]," and write where in the result becomes `Integer` and where stays `T`.

**Next chapter (Part 4):** we treat **recursive types**, where *the type itself* appears in the type,
with μ (fixed point) and coinduction. The same α-equivalence technique as this chapter reappears in
the equivalence test.

---

> **This chapter's design sketch** → [`examples/subst.rb`](examples/subst.rb) (self-checks with `ruby subst.rb`)
