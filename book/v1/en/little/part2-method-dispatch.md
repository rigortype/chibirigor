---
title: Part 2 — Method sends and dispatch
description: Implement typing of method calls with a hand-written "dispatch table," and diagnose argument-type mismatches.
sidebar:
  order: 3
---

# The Little chibirigor Part 2 — Method sends and dispatch

This chapter's goal: **hand the typing of method calls over to a hand-written "dispatch
table."** Since everything in Ruby is a method send, this becomes the foundation.

> This corresponds to 『しくみ』 ch. 3 "Function types" (TAPL ch. 9 "The simply-typed lambda
> calculus"). That book held a function's type as data, `{ params, retType }`. We hold almost
> the same information, but *per method, in a table*.

---

## 2-0. A small tidy-up first — gathering types under `Type::`

As methods grow, so do the type carriers, so let's gather the `Const` / `Nominal` / `Dynamic`
we put directly under `Chibirigor` in Part 1 into a `Chibirigor::Type` module (from here we
write `Type::Const` and so on; the `diagnostic` helper we made in Part 1 stays as is):

```ruby
module Chibirigor
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = "untyped" }
  end
end
```

The groundwork is in place. On to the main topic.

---

## 2-1. Everything in Ruby is a method send

When we gave the `+` in `1 + 2` special treatment in Part 1, we wrote: "`+` is a method send
(`1.+(2)`)." This isn't limited to `+`.

```ruby
1 + 2          # 1.+(2)
"ab".length    # "ab".length()
"a" * 3        # "a".*(3)
```

**Every one of them is just sending a message to a receiver.** Here, fix the word
**receiver** in mind. Many languages call a function *on its own*, like `length("ab")`; Ruby
doesn't — it always calls a method **on someone (the receiver)**, like `"ab".length`. `1 + 2`
is `1.+(2)` underneath, too: "send the message `+`, with argument `2`, to the receiver `1`."
Even the argument-less `"ab".length` has `"ab"` as its receiver. Ruby has almost no *bare
function with no receiver*; write `foo` and that's an implicit `self.foo` to `self` (this
property pays off again in Part 3).

This bears directly on typing. The same `+`, with an `Integer` receiver, means "integer +
integer → integer"; with a `String`, "string + string → string" — **the meaning changes with
the receiver.** So "what does this method return" isn't fixed by the method name alone; it's
fixed only **together with the receiver's type.** That most of "find the type of an
expression" comes down, in the end, to knowing "**what does this method of this receiver
return**" is exactly why. We throw away Part 1's `+`-only code and generalize here.

---

## 2-2. A hand-written dispatch table

When you "send a message to a receiver" like `1.+(2)`, Ruby selects, at run time, "which
implementation of `+` applies to that receiver." This **mechanism that picks the actual method
from the message sent** is called **dispatch** — the second half of the chapter title. What we
do is the *type version* of it: where Ruby picks the method body *at run time*, we pick "what
does that method return (its return type)" *at type-check time*. Instead of actually running
`1.+(2)`, we **pick "what does `(Integer, +)` return" from a table.** So we prepare a
hand-written **dispatch table**.

We hold "which class's, which method, takes what arguments, returns what" in a plain table:

```ruby
module Dispatch
  I = Type::Nominal[:Integer]
  S = Type::Nominal[:String]

  # [receiver class, method name] => { params: [arg types...], returns: return type }
  METHODS = {
    %i[Integer +]      => { params: [I], returns: I },
    %i[Integer to_s]   => { params: [],  returns: S },
    %i[String  +]      => { params: [S], returns: S },
    %i[String  length] => { params: [],  returns: I },
    # ...
  }.freeze
end
```

To look up the table, we need a tool that rounds a type to a "class name" (`Const[1]` and
`Nominal[:Integer]` both to `:Integer`):

```ruby
def class_of(type)
  case type
  when Type::Const   then type.value.class.name.to_sym
  when Type::Nominal then type.name
  end # Dynamic etc. → nil (can't look it up)
end
```

- **① Type theory:** a function's (method's) type is "argument type → return type" (『しくみ』
  ch. 3, `{params, retType}`).
- **② In Ruby:** `+` and `length` are all method sends. Type info is needed per method.
- **③ In Rigor:** there's no function-type carrier; we *look up* `(class, method) → type` *from
  a table* (in real Rigor that table is RBS). For now this book stays with "naive table
  lookup" (the real thing resolves it far more elaborately; the full picture is signposted in
  the 2-6 summary).

---

## 2-3. Hand it to the table

The flow of dispatch is this — look up the table by receiver and method name; if found, check
the arguments and return the return type; if not found, quietly return `untyped`:

```text
  1 + "x"
    │ receiver type = Integer, method = :+, arg types = [String]
    ▼
  METHODS[[:Integer, :+]] ─ found ─→ check args with accepts ─ mismatch ─→ diagnostic
    │                                                       └ match ─→ returns Integer
    └─ not found (unknown method) ─→ untyped (don't frighten it)
```

So there are three exits — **not in the table** → quiet `untyped` (don't frighten it), **in
the table and args match** → the return type, **in the table and args don't match** → a
diagnostic. Every path starts from the same place: "could we look up the table by the
receiver's type and the method name."

![Figure 2-1 — dispatching a method send](../figures/svg/little-2-1.svg)
> ▼ Figure 2-1 — dispatching a method send

The method-call part of `type_of` becomes **just: find the types of the receiver and each
argument, and hand them to the table**:

```ruby
def type_of_call(node, diagnostics)
  receiver = node.receiver ? type_of(node.receiver, diagnostics) : Type::Dynamic.new
  arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, diagnostics) }
  Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
end
```

In Part 1 we looked only at `+`'s argument; now we **run *every* argument through `type_of`.**
Thanks to that, we even find errors *buried deeper down*, like `puts(1 + true)` (even if we
don't know `puts` itself, we notice while typing the argument `1 + true`).

---

## 2-4. Looking at argument count and type

Here's the body of `dispatch`. When the table is found, we check the argument **count** and
**type**:

```ruby
def dispatch(receiver_type, name, arg_types, node, diagnostics)
  signature = METHODS[[class_of(receiver_type), name]]
  return Type::Dynamic.new unless signature # unknown method → don't frighten it (2-5)

  if arg_types.size != signature[:params].size
    diagnostics << Chibirigor.diagnostic(
      node, "wrong number of arguments for #{name} (#{signature[:params].size} expected, #{arg_types.size} given)"
    )
    return signature[:returns]
  end

  signature[:params].zip(arg_types).each do |param, arg|
    next if matches?(param, arg)

    diagnostics << Chibirigor.diagnostic(node, "expected #{param} but got #{arg}")
  end

  signature[:returns]
end
```

Whether the argument types fit, we judge naively for now by "do the classes match":

```ruby
def matches?(param, arg)
  return true if param.is_a?(Type::Dynamic) || arg.is_a?(Type::Dynamic) # unknown passes
  class_of(param) == class_of(arg)
end
```

```ruby
check('"a" + 1')        # ["expected String but got 1"]
check('"ab".length(1)') # ["wrong number of arguments for length (0 expected, 1 given)"]
```

> This `matches?` is a **hand-written stand-in judgment.** In Part 7 we replace it with the
> real `accepts`, which returns the three values `:yes` / `:no` / `:maybe` (paying off "the
> ad-hoc bits in Part 1/2 were a hand-written `accepts`"). For now "class match" is enough.

---

## 2-5. Don't frighten an unknown method

When the `[class, method]` isn't in the table — or the receiver is `Dynamic` (the type was
lost) — we **return `Dynamic` without emitting a diagnostic** (the first `return` in
`dispatch`).

```ruby
check("foo.bar(1, 2)")   # []   ← we know neither foo nor bar. Pass quietly
```

This is an attitude toward Ruby's reality. Ruby has **open classes** (you can add methods to
existing classes), it has `method_missing`, and methods are countless. **Writing them all into
a table is impossible.** So we never treat "not in the table = suspicious." The unknown stays
unknown, and we move on with `untyped`.

- **① Type theory:** how to type an unknown call.
- **② In Ruby:** open classes, countless methods, `method_missing`. The table is necessarily
  incomplete.
- **③ In Rigor:** the unknown degrades to `Dynamic`. Real Rigor brings the table closer to "the
  real thing" with **RBS + inheritance-chain resolution** instead of a hand-written table (a
  spoonful in Part 8; the real resolution in the sequel).

```text
1: String
1: Integer
expected Integer but got "x"
```

---

## 2-6. This chapter's summary

What we added is the `Dispatch` module (the `METHODS` table, `class_of`, `matches?`,
`dispatch`). The `type_of` side, if anything, got *shorter* (the `+`-only code is gone; it just
hands off to the table).

This chapter's three voices:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 3 / TAPL ch. 9) | A method's type is "argument type → return type" |
| ② Ruby / RBS | Everything is a method send. With open classes you can't write it all in a table |
| ③ Rigor's implementation problem | Look up `(class, method) → type` in a table; the unknown degrades to `Dynamic`. Argument judgment is hand-written (promoted to `accepts` in Part 7) |

Stopping at a naive single-stage table lookup is for the sake of gentleness. From here on it's
a matter of *depth* — "how to fatten that table, and how to look it up correctly" — and for now
we only set down the signposts.

**Handed to the sequel / later Parts:**

- the hand-written table → the real lookup from **RBS** (Part 8).
- method resolution that walks the inheritance chain and module mixins, `method_missing`, and
  full open-class support (sequel).
- three-valuing the argument judgment (`accepts`) and robustness (Part 7).
- the full picture of the real five-stage dispatch cascade (constant folding → shape → RBS →
  in-source → fallback) is in appendix a3.

---

## 2-7. A note: constant folding (fold it if you can)

> This is a **note** set aside from the main line. In Part 1 we only "rounded" `1 + 2` to
> `Integer`; here we layer on a story about taking one extra step *just before* that rounding,
> folding where it can be folded. Leaving the teaching code (the table lookup of 2-2–2-4) as
> is, we see what happens if we add *a spoonful* to the resolution of `+`.

In Part 1 we **rounded** `1 + 2` to `Integer`. But `1` and `2` are both *known values* — so we
ought to be able to **actually add them and fold to `Const[3]`.** Holding "the value itself"
one stage longer raises `annotate`'s precision (a miniature of real Rigor's `Constant<3>`
literal precision).

What we do is just "if both operands are `Const` of known value, compute it — but *if it grows
too big*, round." Adding *a spoonful* to the resolution of `+` looks like this (try folding once
before rounding):

```ruby
# If both are Const of known value, compute and fold. Over budget (size) → leave it to rounding.
if recv.is_a?(Type::Const) && arg.is_a?(Type::Const)
  result = recv.value + arg.value
  return Type::Const[result] if result.abs <= 1_000_000   # within budget → fold
end
return Type::Nominal[:Integer]                              # can't fold → round
```

Now `annotate` changes like this:

```text
1 + 2          # => 3          (folded)
1 + 2 + 3      # => 6          (recursion keeps folding: 1+2→3, 3+3→6)
"a" * 3        # => "aaa"      (strings fold too)
100000 * 100   # => Integer    (over 1,000,000 = over budget → round)
1 + x          # => Integer    (x's value unknown = can't fold → round)
```

Two points:

- **widening:** so as not to hold an unboundedly large `Const`, round once past a threshold.
  How real Rigor systematizes this "when to stop folding" is dug into in the Seasoned volume.
- **zero false positives:** folding *only adds precision*. `Const[3]` passes anywhere `Integer`
  does, so no new diagnostic is ever added (an expression that can't fold, like `1 + "x"`,
  rounds as before and keeps its original behavior).

And here's the payoff back into the main line — in the actual `chibirigor`, this folding lives
not as a special case of `+` but on the method **table** side (this chapter's `Dispatch`). So it
works for any operation the table looks up, and if a variable *carries a known `Const`*, as in
`x = 1; 1 + x`, that folds to `2` too. The reason `1 + 2` shows up as `3` when you run
`exe/chibirigor` yourself is that the folding lives on the Dispatch side.

---

## Exercises

1. Add `Integer#*` to the `METHODS` table, and confirm `check("2 * 3")` becomes an empty array.
2. `check` `1.to_s(2)` (too many arguments) and read the message that comes out. Which branch of
   `dispatch` is the arity check?
3. Make three examples of a table-less method call that "passes quietly," and explain why we
   don't frighten them.

---

**Next chapter (Part 3):** local variables and statements. With `x = 1` we remember a type, and
make `x` readable. Here the "type environment = Scope" enters.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part2/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part2/lib)
