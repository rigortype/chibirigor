---
title: Part 1 — Literals and arithmetic
description: Build the smallest machine that gives values a type, and run check and annotate on real Ruby. Introducing Const and type_of.
sidebar:
  order: 2
---

# The Little chibirigor Part 1 — Literals and arithmetic

This chapter's goal: build the smallest possible machine that gives values a type, and run
`check` and `annotate` on real Ruby.

What's new is just one type, `Const`, and one function, `type_of`.

> [!NOTE]
> We refer to *The Mechanics of Type Systems* (『しくみ』) ch. 2 a few times this chapter. That
> book does the same thing in a TypeScript mini-language; we'll set the same idea side by side
> and watch how it changes in Ruby.

---

## 1-1. Representing a type as "data"

**What is a type?** Don't overthink it; here, take it to be "a small label stuck on a value."
`1` gets the label "integer," `"hi"` gets the label "string."

Type checking is looking at whether these labels are at odds with one another.

In 『しくみ』 ch. 2, too, a type was represented as **plain data** like `{ tag: "Number" }`. We
do the same — we represent types as Ruby objects.

Here, flip one switch in your head: **a type is not a "class" like `Integer` itself; it's
*data* that represents a type.** We also want to build "fine types you can't write as a class,"
like `Const[1]`, so we represent them with purpose-built data.

This chapter uses just these three:

```ruby
module Chibirigor
  # A type for "this exact value." E.g. Const[1], Const["hi"]
  Const = Data.define(:value) do
    def to_s = value.inspect
  end

  # A type for a named class. E.g. Nominal[:Integer] (used by the "rounding" in 1-2)
  Nominal = Data.define(:name) do
    def to_s = name.to_s
  end

  # A type for "unknown / no way to check" (it earns its keep later)
  Dynamic = Data.define do
    def to_s = "untyped"
  end
end
```

Here's one round of the **three perspectives** (this book's recurring frame):

- **① Type theory:** a type is a label on a value. Internally it's just data (『しくみ』 ch. 2).
- **② In Ruby:** `1`'s class is `Integer`, `"hi"`'s class is `String`. Ruby and RBS alike say
  no more than "`1` is an `Integer`."
- **③ In Rigor:** Rigor goes one step further and makes **the value `1` itself** a type
  (`Const[1]`). Not "`Integer`" but "`1`." Why bother being so fine-grained? — it pays off
  later, in `case` branches and constant arithmetic. For now, just "Rigor remembers types
  finely."

When a type is unknown — when there's no way to check — we **fall back to `Dynamic`
(`untyped`)** — the mark for "unknown / no way to check — keep quiet."

> [!NOTE]
> **"Don't know" comes in two opposite schools.**
>
> A language's attitude toward "the type is unknown" splits into two opposite schools.
>
> TypeScript's `unknown` is the cautious school: "if you don't know, *narrow it first* before
> you use it." Ruby's `untyped` (and TypeScript's `any`) is the permissive school — "if you
> don't know, *let it through quietly*" — and chibirigor's `Dynamic` is on this side.
>
> Same "don't know," but the schools divide on whether you take soundness or *not stopping
> working code*. This book is permissive precisely in order to "never frighten working code."
>
> Why `untyped` is so easily confused with the "holds anything" top type (they're actually
> different), plus a name-correspondence table across languages, is gathered in appendix
> [a1-1](../appendix/a1-special-types.md).

---

## 1-2. `type_of` — finding a type from an expression

The heart of a type checker is **one function: "take an expression, return a type."** 『しくみ』
called it `typecheck`. We call it `type_of`.

We parse the code with Prism. Following `Prism.parse("1").value`, `1` is an `IntegerNode`,
`"hi"` is a `StringNode` — a node per kind. We just split on the kind:

```ruby
module Chibirigor
  module_function

  def type_of(node, diagnostics)
    case node
    when Prism::IntegerNode then Const[node.value]
    when Prism::FloatNode   then Const[node.value]
    when Prism::StringNode  then Const[node.unescaped]
    when Prism::TrueNode    then Const[true]
    when Prism::FalseNode   then Const[false]
    when Prism::CallNode    then type_of_call(node, diagnostics)
    else
      Dynamic.new   # an unknown node: "don't frighten it" — silently return untyped
    end
  end
end
```

Almost the same shape as 『しくみ』's `typecheck` with its `switch (t.tag)`. The difference is
the last line.

『しくみ』 never meets unknown syntax — its subject is a tidy mini-language.

But we're dealing with *real Ruby*. Unknown things will certainly turn up. When they do, we
don't error — we return `Dynamic` (untyped).

This is Rigor's stance, right at the entrance.

> [!IMPORTANT]
> **A principle to remember:** `type_of` *never fails.* If it can't tell the type, it just
> returns `Dynamic`. So "complaining because code has no type" can't happen in the
> first place.

### What happens with `1 + 2`?

Here's one fact peculiar to Ruby. **The `+` in `1 + 2` is also a "method send."** In Prism it
becomes a `CallNode` sending the message `+` to `1` (the same as `1.+(2)`).

For now it's arithmetic only, written very naively:

```ruby
module Chibirigor
  module_function

  def type_of_call(node, diagnostics)
    recv = type_of(node.receiver, diagnostics)
    args = node.arguments&.arguments || []

    if node.name == :+ && integerish?(recv)
      arg = type_of(args.first, diagnostics)
      unless integerish?(arg)
        diagnostics << diagnostic(node, "can't add #{arg} to an integer")
        return Dynamic.new
      end
      # ★ The point: we don't compute Const[3] — we "round" to Integer
      return Nominal[:Integer]
    end

    Dynamic.new   # any other method we don't know yet → don't frighten it
  end

  def integerish?(t)
    (t.is_a?(Const) && t.value.is_a?(Integer)) || t == Nominal[:Integer]
  end

  # A diagnostic is a small hash carrying "which line, what's wrong"
  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
```

`Nominal[:Integer]` is the label for "the integer class" we defined in 1-1.

Here the **third perspective — ③, where Rigor ran into trouble — surfaces naturally:**

- `type_of(1)` is `Const[1]`, `type_of(2)` is `Const[2]`.
- So should `type_of(1 + 2)` be `Const[3]`?
  - It's tempting, but that would mean *actually computing the addition*.
  - With `x + 2` we no longer know the value.
- So we round the result to `Integer`.
  - Remembering "the value itself" is handy, but somewhere you have to let go and fall back to
    a coarse type.
  - For now it's enough to remember "the result of addition is `Integer`."
  - How real Rigor systematizes *when to round* is treated in the Seasoned volume.

## 1-3. `check` — finding contradictions (but not stopping)

Now that we have `type_of`, we build **the reporting of contradictions** — `check`. All it does
is "run each top-level statement through `type_of` and gather the complaints (diagnostics) found
along the way":

```ruby
module Chibirigor
  module_function

  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    program.statements.body.each { |stmt| type_of(stmt, diagnostics) }
    diagnostics
  end
end
```

`check`'s return value is **an array of `{line:, message:}`** (which line, what's wrong). Let's
run it:

```ruby
Chibirigor.check("1 + 2")       # => []                                            (no complaint)
Chibirigor.check('1 + "x"')     # => [{ line: 1, message: "can't add \"x\" to an integer" }]
Chibirigor.check("foo.bar")     # => []   ← an unknown method passes quietly
```

When 『しくみ』's `typecheck` finds a contradiction, it **`throw`s and stops there.** We're
different: **we pile complaints into an array and read on to the end.** We don't stop at the
first error, and where we don't know (`foo.bar`) we *let it slide quietly.* This too is part of
"never frighten working code."

- **① Type theory:** type checking = detecting contradictions among labels (『しくみ』 ch. 2).
- **② In Ruby:** `1 + "x"` is a `TypeError` if you run it. `foo.bar` might work, depending on
  `foo`.
- **③ In Rigor:** report only what definitely contradicts, and stay quiet where you don't know.
  *Don't stop, don't frighten.*

---

## 1-4. `annotate` — showing the types we found

Once we're here, a bonus comes almost for free. Since `type_of` is *building* types, just
**outputting them** gives us `annotate`, which "shows the inferred types."

To match `check` returning an array of `{line:, message:}`, `annotate` returns an array of
`{line:, type:}` per statement (the line number and the inferred type):

```ruby
module Chibirigor
  module_function

  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      type = type_of(stmt, [])     # throw away complaints for now
      { line: stmt.location.start_line, type: type }
    end
  end
end
```

Let's display it (types print via `to_s` as `1` / `Integer` / `untyped`):

```ruby
Chibirigor.annotate(<<~RUBY).each { |a| puts "#{a[:line]}: #{a[:type]}" }
  42
  "hello"
  1 + 2
  foo.bar
RUBY
```

```text
1: 42
2: "hello"
3: Integer
4: untyped
```

- `42` is `Const[42]`, so it prints finely as `42`.
- `1 + 2` rounds to `Integer`.
- `foo.bar` is unknown, so it's `untyped`.
- **"Where `untyped` appears" = "where Rigor lost track of the type,"** and being able to *see*
  that is exactly what makes `annotate` worthwhile (the seed of real Rigor's `sig-gen` idea).

> [!NOTE]
> chibirigor's `annotate` is the minimal version that shows the inferred internal type as is.
>
> Real Rigor's `annotate` has a two-layer structure — "know precisely inside, round coarsely at
> the RBS boundary" — so even though inference knows more, the outward-facing signature comes
> out coarse. That machinery is in appendix [a3-2](../appendix/a3-tooling.md).

> [!NOTE]
> The code so far only *rounds* `1 + 2` to `Integer`. But `1` and `2` are both known values, so
> there's actually room to **actually add them and fold to `3`.**
>
> This "fold it if you can" constant folding is treated in Part 2's note (it's why
> `exe/chibirigor` shows `3` for `1 + 2` when you run it yourself).

---

## 1-4b. Making diagnostics easier to read — position and caret

If we give a diagnostic not just a **`line`** but also a **`column`** and **`length`**, it can
*point at* where the problem is. We extend `diagnostic` by a spoonful (Prism nodes carry
position info in `location`):

```ruby
def diagnostic(node, message)
  location = node.location
  { line: location.start_line, column: location.start_column, length: location.length, message: message }
end
```

Then the CLI (`exe/chibirigor`) can draw a **caret `^^^`** under the offending line:

```console
$ ruby exe/chibirigor check bad.rb
bad.rb:2:1: expected Integer but got "bad"
  1 + "bad"
  ^^^^^^^^^
```

Just that much, and "which line, which word" is clear at a glance. Real Rigor builds this out
further, converting it into SARIF and GitHub annotations (ADR-51).

---

## 1-5. This chapter's summary

What we built: the types `Const` / `Dynamic` / `Nominal`, and the functions `type_of` /
`check` / `annotate`. About 50 lines in all. A sense of scale: 『しくみ』 ch. 2's `typecheck`
(about 40 lines) plus a little "rounding," "don't stop," and "fall back to untyped."

This chapter's three perspectives:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 2 / TAPL ch. 8) | A type is a label on a value = plain data. A function that finds a type from an expression is the heart |
| ② Ruby / RBS | `1` is `Integer`; even `+` is a method send. RBS stops at "`1` is `Integer`" |
| ③ Rigor's implementation problem | Making the value itself a type (`Const[1]`) is fine-grained but raises *when to round to `Integer`* (dug into in the sequel) |

## Exercises

1. Check the results of `annotate("3.14")` and `annotate("true")`, and explain how `Const#to_s`
   is doing its work.
2. The current `type_of_call` only rounds `+`. Extend it so `1 - 2` also becomes `Integer`
   (hint: make the condition cover both `:+` and `:-`).
3. `check`-ing `1 + 2 + 3` produces no diagnostic. Explain why it passes with no contradiction,
   in terms of `type_of`'s recursion.

---

**Next chapter:** we take on, head-on, the "even `+` is a method send" we waved past in `1 + 2`.
Since everything in Ruby is a method send, we need **a table (dispatch) for "which method of
which class returns what."** And in a note we treat the continuation of "an unknown method falls
back to `Dynamic`," plus the **constant folding** (fold it if you can) previewed in §1-4.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part1/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part1/lib)
