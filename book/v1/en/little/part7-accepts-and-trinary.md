---
title: Part 7 — Acceptance checks and "maybe"
description: "Build `accepts`, which judges \"does this type fit when passed here?\" with three values (`:yes` / `:no` / `:maybe`)."
sidebar:
  order: 8
---

# The Little chibirigor Part 7 — Acceptance checks and "maybe"

This chapter's goal: **build `accepts`, which judges "does this type fit, passed here?"** But the
answer isn't a two-way `yes` / `no` — it's a three-way **`yes` / `no` / `maybe`**. This "maybe"
is the single biggest reason Rigor can stay gentle.

> In 『しくみ』 this is ch. 7 "Subtyping" (TAPL ch. 15 "Subtyping"). That book answered "fits /
> doesn't fit" as a two-way `true`/`false`. We add one thing to it.

---

## 7-1. A new question: "does it fit?"

So far `type_of` only found a type *upward* from an expression (`1` → `Const[1]`). But to find
errors, a different question is needed:

> This method wants an `Integer`. What you passed is a `String`. — **Does it fit?**

This is an **acceptance check.** In 『しくみ』 ch. 7 it's built as `subtype(a, b)` (is a a subtype
of b), and the answer was a two-way `true`/`false`.

But making this two-way in Ruby gets you stuck right away. `type_of` returns `Dynamic` (untyped)
for an expression it can't tell (we decided so in Part 1). So:

> An `untyped` arrived where an `Integer` is wanted. — Does it fit?

Answer `true` and you might miss a real bug. Answer `false` and you might **flag code
that works fine.** Both are lies. The true answer is **"don't know (maybe)."**

So `accepts`'s answer is three-way: `:yes` / `:no` / `:maybe`.

---

## 7-2. First, where it's black and white — definitely fits, definitely doesn't

What is "fits"? Don't overthink it; take it as **"does the value passed go into the *box* of the
type being wanted."** Into the `Integer` box go both `1` and `2`. `String`'s `"x"` doesn't. But
comparing `Const[1]` and `Nominal[:Integer]` directly won't match — the type of a value itself
and the type of a class are different things. So before comparing, we *round the value's type to
a class* to line them up (`widen`; Part 1's "rounding" works here too):

```ruby
# "the value itself" like Const[1]: round to a class before comparing
def widen(t) = t.is_a?(Type::Const) ? Type::Nominal[t.value.class.name.to_sym] : t

def accepts(expected, actual)
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Type::Nominal[:Integer], Type::Const[1])      # => :yes  (1 fits in the Integer box)
accepts(Type::Nominal[:Integer], Type::Const["x"])    # => :no   ("x" doesn't fit)
```

This "does it fit in the box" is 『しくみ』 ch. 7's **subtyping** itself (no need to memorize the
term; just *a small box fits in a big box*). 『しくみ』 defined box sizes carefully in steps; we
start from the most naive place — "is the class the same."

> **Column: "subtype" is not only inheritance**
>
> "Subtype" isn't only inheritance (it fits in the box because it inherited the class). Part 6's
> "a hash with *more* keys is a subtype of a hash with *fewer* keys" (`{name:, age:}` is a subtype
> of `{name:}`) has no inheritance relation at all — it's decided by *shape* alone: **as long as
> the structure (the keys held) is there, it's a subtype.** This is called **structural
> subtyping.** **Rigor / chibirigor handle subtyping decided by inheritance and subtyping decided
> by structure together, in one judgment: "does it fit in the box."** `accepts` is its doorway.
> (For the correspondence with Java's inheritance and Go's / TypeScript's structural types, see
> appendix [a5-2](../appendix/a5-other-languages.md). The formal treatment is in Seasoned Part 2.)

- **① Type theory:** does a value *go into* the expected type = subtyping (『しくみ』 ch. 7).
- **② In Ruby:** `1` is `Integer`, `"x"` is `String`. You can judge roughly by class.
- **③ In Rigor:** a fine type like `Const[1]` is *rounded to a class* before judging (Part 1's
  "rounding" works here too).

---

## 7-3. When "maybe" comes out — untyped mixes in

Now for the main point. Once `Dynamic` (untyped) is involved, we don't call it black or white.
**We return `:maybe`.**

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)   # ★ added
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Type::Nominal[:Integer], Type::Dynamic.new)   # => :maybe  (the unknown stays unknown)
```

Just one line, but it's one of the main pillars making Rigor a "type checker gentle to dynamic
languages." `untyped` is the mark of "the type was lost." About something lost, not insisting it
"fits" or "doesn't fit" — that is what honesty is.

- **① Type theory:** once unknown (untyped) mixes in, the verdict *can't fall to one side*.
- **② In Ruby:** code with no annotations is normal. Nobody knows the return of `foo.bar`.
- **③ In Rigor:** where unknown, `:maybe`. This connects directly to "don't frighten" in the next
  section.

---

### 7-3a. When a Union comes as an argument — take the weakest answer

A Union (a type like "either `Integer` or `String`," built in Part 4–5) can be passed too. **When
the passing side (actual) is a Union, we don't know which member's value actually arrives**, so
it's safe only when all members pass — so we just **run every member through `accepts` and take
the weakest answer.** Even one `:no` is `:no`; with no `:no` but a `:maybe`, it's `:maybe`; all
`:yes` is `:yes`:

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)

  if actual.is_a?(Type::Union)   # :yes only when all members pass (the weakest answer)
    results = actual.members.map { |m| accepts(expected, m) }
    return :no    if results.include?(:no)
    return :maybe if results.include?(:maybe)
    return :yes
  end

  if expected.is_a?(Type::Union)  # :yes if any one matches (the strongest answer)
    results = expected.members.map { |m| accepts(m, actual) }
    return :yes   if results.include?(:yes)
    return :maybe if results.include?(:maybe)
    return :no
  end

  widen(expected) == widen(actual) ? :yes : :no
end
```

When `expected` is a Union (`accepts(Integer | String, Integer)`, etc.), we take **the strongest
answer** — if any one matches, it's `:yes`. The intuition: "expecting either `Integer` or
`String`," and what's passed is `Integer`, is no problem.

With this, even when **a Union comes as an argument**, as in `1 + (c ? 1 : 2)`, if the contents
are all `Integer` it's `:yes` = stay quiet (no false positive). If a `String` mixes in, as in
`Integer | String`, we report `:no`.

> (A **parenthesized expression** like `(c ? 1 : 2)` becomes a separate node, `ParenthesesNode`,
> in Prism. Add a line to `type_of` to evaluate the body and return it:
> `when Prism::ParenthesesNode then type_of_body(node.body, scope, diagnostics)`. Forget this and
> a parenthesized expression falls to `untyped`.)

- **① Type theory:** `:yes` only when every member of the Union passes (take the weakest
  conclusion = union-subtyping).
- **② In Ruby:** `c ? 1 : 2` is `Integer | Integer`, so no problem. `c ? 1 : "a"` becomes a Union,
  and passing it to an expression expecting `Integer` is `:no`.
- **③ In Rigor:** even checking against a Union, "if unknown, `:maybe`." We can only complain when
  everyone is `:yes`.

![Figure 7-1 — accepts's three-valued verdict: untyped is :maybe, a Union the weakest answer](../figures/svg/little-7-1.svg)
> ▼ Figure 7-1 — `accepts`'s three-valued verdict: untyped is `:maybe`, a Union takes the weakest answer

---

## 7-4. "Maybe" is not punished — the chapter's climax

We use `accepts` in `check`. The place an error is reported is **only "where the wanted type is
fixed."** Recall the hand-written dispatch table from Part 2. `Integer#+` was written as "wants
one `Integer`." **That is "where the wanted type is fixed."**

```ruby
# Part 2's table (recap). param holds "the wanted type"
METHODS = {
  [:Integer, :+] => { params: [Type::Nominal[:Integer]], returns: Type::Nominal[:Integer] },
  # ...
}

def check_call(recv_type, name, arg_types, diagnostics)
  sig = METHODS[[class_of(recv_type), name]]
  return Type::Dynamic.new unless sig          # unknown method → don't frighten it (Part 2's policy)

  sig[:params].zip(arg_types).each do |want, got|
    case accepts(want, got)
    when :no
      diagnostics << "expected #{want} but got #{got}"
    when :maybe, :yes
      # do nothing ← this is everything
    end
  end
  sig[:returns]
end
```

As you can see, **we complain only on `:no`.** `:yes` of course, but **on `:maybe` too, we stay
quiet.**

```ruby
check("1 + 2")        # no complaint (:yes)
check('1 + "x"')      # ["expected Integer but got \"x\""] (:no)
check("1 + foo.bar")  # no complaint! foo.bar is untyped → :maybe → stay quiet
```

```text
Integer | Integer: OK (no errors)
expected Integer but got 1 | "a"
```

Here we set down the single most important sentence in this book. No jargon needed:

> **An error only appears where the wanted type is fixed. And once untyped mixes in it's
> `:maybe`, and `:maybe` is not punished. So — code whose type is unknown but that nonetheless
> works is never flagged.**

『しくみ』 ch. 7 had a column with a very similar concern: "a sound type system *rejecting good
programs* is a false positive." 『しくみ』 worked in the direction of *reducing* false positives.
Rigor goes one step further, **placing "unknown = maybe = stay quiet" at the center of the
mechanism,** making false positives hard to produce in the first place.

And in fact — that ad-hoc `integerish?` check we wrote for `+` in Part 1–2 was **the hand-written
version of this `accepts`.** We've now replaced it with the proper mechanism.

---

## 7-5. Strict on returns, lenient on arguments (a small column)

One last small observation. Right now we,

- judged **arguments** *leniently* with `accepts` (letting `:maybe` through).
- meanwhile find the **return type** *exactly* with `type_of` (for `Integer#+`, asserting
  `Integer`).

This asymmetry — **"strict and precise about what you return, lenient and liberal about what you
accept"** — isn't an accident; it's a discipline Rigor deliberately keeps. Make acceptance strict and
the caller is forced to write needless type conversions and gets cramped. Make the return lenient
and whoever uses that value loses out. So we invert them.[^postel]

[^postel]: This practice of "strict in returns, liberal in arguments" has a name too, but you needn't
    memorize it. Why this asymmetry is correct — arriving at the same rule from separate starting
    points in type theory and in object-oriented substitutability — we confirm head-on in Seasoned
    Part 2.

---

## 7-6. This chapter's summary

What we added is the function `accepts` (returning `:yes` / `:no` / `:maybe`), and the mechanism
where `check` uses it to **complain only on `:no`.** The code is just `accepts`'s body of 4 lines
+ a few extra lines for Union + swapping out `check`. No new type carrier was added.

This chapter's three voices:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 7 / TAPL ch. 15) | Does a value go into the expected type = subtyping |
| ② Ruby / RBS | Duck typing, code with no annotations, untyped mixes in normally |
| ③ Rigor's implementation problem | Two-way means false positives or misses → **bring "unknown" out with a third value (`:maybe`), and don't punish it** |

**Handed to the sequel** (digging in here would break the gentleness):

- the **real subtyping** that defines box sizes properly in steps (『しくみ』 ch. 7's width/depth).
- **variance** (when passing a function, the argument direction alone reverses — "contravariance").
  The climax of 『しくみ』 ch. 7.
- the two directions of `type_of` (find) and `accepts` (check), together called **bidirectional
  typing**.

## Exercises

1. Confirm that `accepts(Nominal[:Integer], Union[[Const[1], Const["x"]]])` becomes `:no` (the
   weakest answer).
2. Confirm with the implementation above the behavior when *expected* is a Union
   (`accepts(Integer | String, Integer)`). Explain why it returns `:yes`, along with why it isn't
   `:no`.
3. Make one example where `:maybe` comes out but `check` stays quiet, and confirm "`:maybe` is
   not punished."

---

**Next chapter (Part 8):** we swap the hand-written `METHODS` table for one pulled from real
**RBS**. We touch, for the first time, the Ruby/RBS worldview of "write the types in a separate
file."

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part7/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part7/lib)
