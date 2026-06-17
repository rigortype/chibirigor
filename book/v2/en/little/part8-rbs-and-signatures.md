---
title: Part 8 — RBS and type signatures
description: "Swap the hand-written `METHODS` table for an RBS-derived one, then read `def`, synthesize a return type from the body, and emit an RBS-style signature."
sidebar:
  order: 9
---

# The Little chibirigor Part 8 — RBS and type signatures

This chapter's goal: **put types "outside the code," then take them back "from inside the
code."** In the first half, we swap the hand-written `METHODS` table for one loaded from a
*separate file* (RBS) — not a single diagnostic changes when we swap, and that's the proof of
correctness.

In the second half, taking that RBS notation as a model, we **read the `def` of an
annotation-free method, synthesize a return type from its body**, and emit an RBS-style
signature. Here it's clearest that `chibirigor` is **built on a foundation of inference** — even
for an annotation-free method, the return type is fixed by the last expression of the body.

> [!NOTE]
> 『しくみ』 ch. 9 "Generics" (TAPL ch. 23 "Universal types / System F") and its *type
> substitution* is a distant relative, but what we really meet here is the Ruby/RBS-specific
> worldview — "**types are written not in the code, but in a separate file (.rbs).**"
>
> The second half goes the reverse way, toward TAPL ch. 22 "Type reconstruction" — raising
> types from annotation-free code.

---

## 8-1. Types are written in a "separate file" — RBS

Until now we wrote a method's type directly into Ruby code (the `METHODS` table). But Ruby's own
way is different. You **don't write** type annotations in Ruby code. Instead, types are written in
a *separate file* (`.rbs`) called **RBS** — the aim being that you can **add just type
information later, without changing a single character of the working Ruby code**:

```rbs
class Integer
  def +: (Integer) -> Integer
  def to_s: () -> String
end
```

Since this may be the first notation in which you've "written" a type, let's read it out.
`def +: (Integer) -> Integer` means "`Integer#+` **takes one `Integer` and returns an
`Integer`**." The right of `:` is the type, `(...)` the argument types, the right of `->` the
**return type**. `def to_s: () -> String` means "takes no arguments, returns `String`." Read it
as no more than the head of a Ruby `def` with the argument and return types penciled in, and
you're fine.

One Ruby gotcha: in Ruby `->` makes a lambda (`square = ->(x) { x * x }`), but in RBS it's a
*different arrow* meaning "take these arguments → return this type." Same symbol, different job —
make peace with it.

This is the Ruby/RBS worldview. "The code knows nothing about types. Types are given from
outside." Rigor reads this RBS as **truth**, and adds more precision on top of it.

- **① Type theory:** look up and use a declared type (a distant relative of 『しくみ』 ch. 9's type
  substitution).
- **② In Ruby:** there are no type annotations in the code. Types are written separately in
  `.rbs`.
- **③ In Rigor:** read RBS as the source of truth. The hand-written table was a *mini version* of
  that RBS.

---

## 8-2. Reading a very small RBS

Ideally we'd use the real `rbs` gem, but here, chibirigor-style, we **read a minimum ourselves**
(no added dependency / we see everything that happens). The shapes we handle are just two:
`class` and `def name: (args) -> ret`:

```ruby
module Rbs
  CLASS_LINE = /\A\s*class\s+(\S+)\s*\z/
  DEF_LINE   = /\A\s*def\s+(\S+):\s*\((.*)\)\s*->\s*(\S+)\s*\z/

  def load(source)
    table = {}
    current = nil
    source.each_line do |line|
      if (m = CLASS_LINE.match(line))
        current = m[1].to_sym
      elsif current && (m = DEF_LINE.match(line))
        params = m[2].split(",").map(&:strip).reject(&:empty?).map { |t| Type::Nominal[t.to_sym] }
        table[[current, m[1].to_sym]] = { params: params.freeze, returns: Type::Nominal[m[3].to_sym] }
      end
    end
    table.freeze
  end
end
```

The one line `def +: (Integer) -> Integer` becomes
`[:Integer, :+] => { params: [Integer], returns: Integer }`, that's all. Real RBS is far richer,
but the bones are the same — "turn declarations into a table."

---

## 8-3. Swap the hand-written table for an RBS-derived one

We swap `Dispatch`'s `METHODS` from a hand-written literal to an RBS load:

```ruby
module Dispatch
  # Was a hand-written literal. Now generated from RBS text.
  METHODS = Rbs.load(Rbs::CORE)
end
```

In `Rbs::CORE` we write, in RBS text, the methods of the core types that dispatch needs (the same
content as Part 2's hand-written table, plus a "complete version" that includes `*` and `upcase`
used in later chapters):

```ruby
module Rbs
  CORE = <<~RBS
    class Integer
      def +: (Integer) -> Integer
      def -: (Integer) -> Integer
      def *: (Integer) -> Integer
      def to_s: () -> String
    end
    class String
      def +: (String) -> String
      def *: (Integer) -> String
      def length: () -> Integer
      def upcase: () -> String
    end
  RBS
end
```

Since the content is the same as the hand-written table, **not a single diagnostic changes** when
we swap. That all of Part 1–7's tests stay green is the proof (= a safe refactor that swaps only
the foundation without changing behavior).

```console
$ ruby test/test_part1.rb  # … green
$ ruby test/test_part7.rb  # … green (only the table's origin changed)
```

- **① Type theory:** consolidate the origin of types into a declaration (RBS).
- **② In Ruby:** `.rbs` is the single source of types.
- **③ In Rigor:** hand-written table → RBS-derived. Behavior is unchanged (we swap only the
  internal implementation without changing the externally-observed behavior).

With that, we have the foundation for reading types "from outside the code" (RBS). Next is the
reverse — read the code of an **annotation-free** method, and *synthesize from our side* its
RBS-notation signature.

---

## 8-4. A return type can be synthesized from the body

Ruby methods have no type annotations. But **the return type can often be told from the body**:

```ruby
def greet
  "hi".upcase   # returns String
end
```

`"hi".upcase`'s type is (from the RBS table of the previous sections) `String`. A method's return
type is the type of the body's last expression itself. So it can be synthesized. We add `def` to
`type_of`:

```ruby
when Prism::DefNode then type_of_def(node, scope, diagnostics)

def type_of_def(node, scope, diagnostics)
  method_return_type(node, scope, diagnostics)  # type-check the body (diagnostics gather too)
  Type::Const[node.name]                        # the def expression's value is the method-name symbol
end

def method_return_type(node, scope, diagnostics)
  # params are untyped (the main volume doesn't infer arguments = sequel)
  body_scope = method_param_names(node).reduce(scope) { |s, n| s.with_local(n, Type::Dynamic.new) }
  type_of_body(node.body, body_scope, diagnostics)
end
```

There are two small tools used here. `method_param_names` just extracts the required parameter
names. `type_of_body` is a helper that "evaluates a statement sequence top-down and returns **the
last statement's type**," reusing Part 3's `eval_statement` (evaluate one statement and return
`[type, scope]`) — the body of an `if` branch and the body of a `def` are both "statement
sequences," so the same tool handles them:

```ruby
def method_param_names(node)
  node.parameters&.requireds&.map(&:name) || []
end

# evaluate the statement sequence and return the last statement's type (threads scope inside branches too)
def type_of_body(statements_node, scope, diagnostics)
  return Type::Const[nil] if statements_node.nil?   # an empty body is nil

  last = Type::Const[nil]
  statements_node.body.each { |stmt| last, scope = eval_statement(stmt, scope, diagnostics) }
  last
end
```

With that, **the body of a `def` gets type-checked too** (`check` picks up the error inside
`def bad; 1 + "x"; end`). Since arguments are `untyped`, `def ok(x); x + 1; end` produces no false
positive (`untyped + Integer` is `:maybe` → stay quiet).

> [!NOTE]
> `type_of_body` made an empty body the `nil` type, but RBS has one looser return type still,
> **`void`** ("a value is returned, but don't rely on it"). Where this matters is the
> **contract** side.
>
> Declaring a return type `-> nil` means promising "I return nil," and later changing the
> implementation to return another value breaks the contract. `void`, on the other hand, makes
> the caller promise "don't depend on the return value," so **changing the return value later
> isn't a breaking change (BC break).** Herein lies the practical benefit of choosing `void` for
> a method called for its side effect.
>
> chibirigor is on the *synthesizing* side of return types (it *builds* types, not *verifies*
> annotations), so `void` doesn't appear — we always emit a concrete type, "the type of the last
> expression." A summary of the three special types `void` / `never` / `untyped` is in **Part 9**;
> their place on the lattice (an alias of ⊤) is in appendix
> [a1-2](../appendix/a1-special-types.md).

---

## 8-5. Showing it RBS-style

`annotate` returns a signature string only when the statement is a `def`, and otherwise the
inferred type as before. We just branch on the statement kind:

```ruby
def annotate(source)
  program = Prism.parse(source).value
  scope = Scope.new
  ignored = []
  program.statements.body.map do |stmt|
    if stmt.is_a?(Prism::DefNode)
      { line: stmt.location.start_line, type: method_signature(stmt, scope, ignored) }
    else
      type, scope = eval_statement(stmt, scope, ignored)
      { line: stmt.location.start_line, type: type }
    end
  end
end

def method_signature(node, scope, diagnostics)
  params = method_param_names(node).map { "untyped" }.join(", ")
  "def #{node.name}: (#{params}) -> #{method_return_type(node, scope, diagnostics)}"
end
```

```console
$ printf 'def greet\n  "hi".upcase\nend\n' | ruby exe/chibirigor annotate /dev/stdin
```

```text
1: def greet: () -> String
```

`check` and `annotate` use **the same inference engine** (`type_of` / `method_return_type`).
Inference is the foundation, and both checking and display use its output — this is the form of
"a type checker built on a foundation of inference" we spoke of in Part 0.

---

## 8-6. Where `untyped` appears = the weak point of inference

Since arguments are `untyped`, when that flows through to the return, `untyped` shows its face
(it propagates, as in `n is untyped → n * 2 is untyped too`):

```text
1: def double: (untyped) -> untyped
1: def mystery: (untyped) -> untyped
```

This **way `untyped` appears is itself "where inference lost the type."** Where to fix so the
types go through is clear at a glance. This is the seed of Rigor's `sig-gen` (its RBS-generating
feature) — the `untyped` in a generated RBS points to "where a human should add a type."

> [!NOTE]
> Leaving `def double(n)`'s argument `n` as `untyped` here is a **design judgment.** Ruby's
> bundled TypeProf would find **where `double` is called**, as in `double(3)`, work `n` backward
> to `Integer`, and fill in `(Integer) -> Integer`.
>
> chibirigor (and Rigor) deliberately don't — instead of tracing every caller, they look at each
> method locally and default an unknown argument to `untyped`. It scales better, and produces no
> false positives.
>
> **The "real inference" that guesses arguments from how they're used is taken on head-on in
> Seasoned Part 5.**

- **① Type theory:** synthesize a return type from the body (a type stands even without
  annotations).
- **② Ruby / RBS:** methods have no annotations, but the return can often be told from the body.
- **③ Rigor's implementation problem:** show the synthesized type RBS-style, and make inference's
  holes visible with `untyped`.

---

## 8-7. This chapter's summary

What we added (first half) is `Rbs.load` (a very small RBS reader) and `Rbs::CORE`. Only the
*origin* of `Dispatch::METHODS` changed; behavior didn't. What we added (second half) is
`type_of`'s `DefNode` support (body check + return-type synthesis) and `annotate`'s
`method_signature`. On the foundation of reading types "from outside," we placed a contrivance for
raising types "from inside."

This chapter's three perspectives:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 9 / TAPL ch. 22, 23) | Look up and use a declared type (a distant relative of type substitution) / synthesize a return type from the body (a type stands with zero annotations) |
| ② Ruby / RBS | Types aren't written in the code but in a separate `.rbs` file / methods have no annotations but the return is told from the body |
| ③ Rigor's implementation problem | Make RBS the source of truth (swap only the foundation without changing behavior) / show with an RBS-style sig and make inference's weak point visible with `untyped` (the seed of sig-gen) |

What we assembled in the main volume is just the skeleton of "types in a separate file, the return
from the body." The rest is homework re-treated in the Seasoned volume *with proper names* — at
the close of the main volume, let's survey where it's headed.

**Handed to the sequel / later Parts:**

- **argument inference** (guess `x`'s type from how it's used in the body). The main volume stops
  at argument = `untyped` — the heart of this type inference is taken on head-on in **Seasoned
  Part 5.**
- full RBS loading with the real `rbs` gem (union, optional, blocks, generics), substitution of
  type variables (`Array[Elem]` → `Array[String]`), and method resolution along the inheritance
  chain.
- the confluence of return types across multiple `return`s, and writing out generated RBS
  (erasure). Dug into in Seasoned Part 3.

## Exercises

1. Add `String#downcase: () -> String` to `Rbs::CORE`, and confirm `"A".downcase` passes.
2. Name one RBS syntax the homemade mini RBS reader **can't handle** (e.g. union types
   `Integer | String`, optional `?`, blocks). What would `DEF_LINE`'s regex need to handle it?
3. Confirm that Part 1–7's tests stay green after swapping the table to RBS-derived, and explain,
   in your own words, what "swap the foundation without changing behavior" means.
4. Confirm the signature of `def f\n  1 + 2\nend` with `annotate`.
5. Why is the return type of `def g(x)\n  x.upcase\nend` `untyped`? What's needed to produce
   `String` (hint: argument type inference = Seasoned Part 5's story)?
6. `check` a `def bad\n  1 + "x"\nend` with an error in its body, and confirm the diagnostic's line
   number points to the body's line.

---

**Next chapter (Part 9, finale):** we close all this with the philosophy of `gradual` typing. We
finish off `untyped`'s propagation, sum up the "three special types" `untyped` / `void` / `never`,
and tell in full "chibirigor never frightens working code by deliberately missing things." We
connect to gradual typing — which 『しくみ』 named as one of its frontiers in closing — and shut the
main volume.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part8/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part8/lib)
