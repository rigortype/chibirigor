---
title: "Part 6 — Hash and array types"
description: "Give structural types (`HashShape` / `Tuple`) to hash and array literals, and design them to allow a partial match."
sidebar:
  order: 7
---

# The Little chibirigor Part 6 — Hash and array types

In the last chapter (Part 5) we narrowed Unions by case, slimming the type per branch of an
`if`. Now we turn to the *contents* of a value. **Give structural types to hash and array
literals (`HashShape` / `Tuple`).** Then we find the type of reading a value out of them. Ruby
code is awash in "symbol-keyed hashes," so handling this well makes things practical in one
leap.

> This corresponds to 『しくみ』 ch. 5 "Object types" (TAPL ch. 11 §11.8 "Records" / §11.7
> "Tuples"). That book represented the same thing as a type `{ tag: "Object", props }`. We do
> almost the same in Ruby — and make one last judgment call: to make `HashShape` **open** (allow
> extra keys).

---

## 6-1. Raising a type from a literal — HashShape and Tuple

What's the type of `{ foo: 1, bar: "a" }`? "`Hash`" is too coarse. We want to remember **which
key holds which type**. That's `HashShape`:

```ruby
module Type
  HashShape = Data.define(:fields) do   # fields: { foo: Const[1], bar: Const["a"] }
    def to_s = "{" + fields.map { |k, v| "#{k}: #{v}" }.join(", ") + "}"
  end

  Tuple = Data.define(:elements) do     # remember an array by per-position type
    def to_s = "[" + elements.map(&:to_s).join(", ") + "]"
  end
end
```

We just add two cases to `type_of`. In Prism a hash is a `HashNode` (each pair an `AssocNode`, a
symbol key a `SymbolNode`), an array an `ArrayNode`:

```ruby
when Prism::HashNode
  fields = node.elements.to_h { |a| [a.key.unescaped.to_sym, type_of(a.value, scope, diag)] }
  Type::HashShape[fields]
when Prism::ArrayNode
  Type::Tuple[node.elements.map { |el| type_of(el, scope, diag) }]
```

```ruby
type_of(parse(%q[{ foo: 1, bar: "a" }]))   # => {foo: 1, bar: "a"}
type_of(parse(%q[[1, "x"]]))               # => [1, "x"]
```

- **① Type theory:** a type that gathers several values by label = a record type (『しくみ』
  ch. 5).
- **② In Ruby:** symbol-keyed hashes are everywhere. Arrays are used tuple-like too
  (`[name, age]`).
- **③ In Rigor:** don't round to `Hash`; remember the type per key and per position (an extension
  of Part 1's "remember finely").

---

## 6-2. Reading out — `h[:foo]` and `a[0]`

Since the type holds "which key is which type," reading out is straightforward. `h[:foo]` is a
`[]` method send in Prism (`h.[](:foo)`). If the argument is a **literal symbol/integer**, we can
look it up from the type:

```ruby
def read_index(receiver, arg_node)
  if receiver.is_a?(Type::HashShape) && arg_node.is_a?(Prism::SymbolNode)
    # unknown key is nil (because real Ruby returns nil. Don't error)
    return receiver.fields.fetch(arg_node.unescaped.to_sym, Type::Const[nil])
  end
  if receiver.is_a?(Type::Tuple) && arg_node.is_a?(Prism::IntegerNode)
    return receiver.elements.fetch(arg_node.value, Type::Const[nil])
  end
  nil   # can't special-case it → fall through to ordinary dispatch
end
```

```ruby
# when h : {foo: 1, bar: "a"}
h[:foo]   # => 1     (Const[1])
h[:zzz]   # => nil   (★ don't error)
a[0]      # => 1
a[9]      # => nil
```

The point is that `h[:zzz]` **doesn't error.** The reason is simple — **real Ruby returns `nil`
for `{foo: 1}[:zzz]`.** Reading a nonexistent key is not a "bug"; "nil is returned" is the
*correct* behavior. The type follows suit and returns `nil`. It doesn't decide otherwise.

---

## 6-3. Open or closed — allowing extra keys

This is the climax of Part 6. Consider this Ruby:

```ruby
def greet(user)        # suppose user is expected to be { name: ... }
  "Hello, #{user[:name]}"
end

greet({ name: "Alice", admin: true })   # ★ admin is in there too, besides name
```

What `greet` wants is just `name`. But the hash passed in also has `admin`. Should this be **a
fit, or not a fit?**

If it's type *equality*, "the properties must match *exactly*" — but with **subtyping** it's a
different story: "you can pass `{name:, admin:}` where `{name:}` is wanted" is sound, and this is
called **width subtyping** (『しくみ』, too, adopts this width subtyping in ch. 7, allowing extra
properties).

Rigor, too, makes `HashShape` **a fit.** What differs is the *motive and scope*. It's not handling
statically-written records for soundness; the counterpart is **Ruby's option hashes**, and the
aim is **not producing false positives**:

- In Ruby, "build a big option hash, and each method picks just the keys it needs" is **the
  standard move.**
- If you got angry every time there was an extra key, **properly working code would go bright
  red.**

So Rigor's HashShape, seen from the expecting side, is **"as long as it has *at least* these
keys, it's fine"** (open). Extras don't matter. **Only when a needed key is *missing* is it a
problem.** This is how "never frighten working code" shows up in a structural type.

![Figure 6-1 — an open HashShape: allow extras, blame only what's missing](../figures/svg/little-6-1.svg)
> ▼ Figure 6-1 — an open `HashShape`: allow extras, blame only what's missing

- **① Type theory:** **width subtyping** of records — the side with *more* keys is the subtype
  (『しくみ』 ch. 7 uses the same width subtyping). Looks backward? In a phrase: "you can **pass**
  `{name:, admin:}` where `{name:}` is wanted (`name` is duly there); you can't pass the
  reverse" — grasp just *the side with more keys passes more demands, so it's the subtype* and you
  can move on here ("subtype" is taken head-on in the next chapter, Part 7, as 'does it fit in the
  box').
- **② In Ruby:** an extra key in an options hash is everyday. Forcing an exact match doesn't fit
  reality.
- **③ In Rigor:** the expectation is open ("at least"). Allow extras, blame only what's missing =
  avoid false positives.

> **Column: `HashShape` is not Rigor's invention**
>
> "A structural hash type that remembers keys and value types" is not Rigor's invention. Several
> type checkers hit the same problem (Hack's `shape`, PHPStan/Psalm's `array{...}`), and all chose
> "allow extras (open)" — because a naive join mixes the value types into `String | Integer` and
> loses the per-key information. chibirigor's `HashShape` is the minimal implementation of that
> lineage. (For each tool's syntax and history, see appendix
> [a5-3](../appendix/a5-other-languages.md).)

> Actually deciding "are the expected keys present" is Part 7's `accepts`'s job (the three-valued
> judgment of whether types fit). Here we only decided **the *policy* of "allow extras = open."**
> The implementation of the judgment we write in Part 7, when we run HashShape through `accepts`.

---

## 6-4. This chapter's summary

What we added is the type carriers `HashShape` / `Tuple`, two cases in `type_of`, and the read
`read_index`. There's almost no new judgment logic (the read is just `fetch`'s second argument);
the difficulty is concentrated in the concept — "the open policy."

Running it:

```ruby
Chibirigor.annotate("h = {foo: 1, bar: \"a\"}\nh[:foo]\nh[:bar]\nh[:zzz]\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
```

```text
1: {foo: 1, bar: "a"}
2: 1
3: "a"
4: nil
```

`h` is a `HashShape` that remembers each key's type. `h[:foo]` and `h[:bar]` return the
remembered types respectively, and the unknown key `h[:zzz]` returns `nil` without blame (the
*open* policy of allowing "at least").

This chapter's three voices:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 5 / TAPL ch. 11 §11.8) | Gather values by label = a record type. The side with more keys is the subtype |
| ② Ruby / RBS | Symbol-keyed options hashes are heavily used. An exact match doesn't fit reality |
| ③ Rigor's implementation problem | The expectation is open (at least). Allow extras, blame only what's missing = apply width subtyping to a *dynamic hash* and avoid false positives |

**Handed to the sequel:**

- full support for keyword arguments (`def f(name:, **opts)`). The main volume stops at handling
  them as hash values.
- **the type difference between `map` and `filter_map`:** in Rigor, `tuple.map { |x| f(x) }`
  *keeps* the per-position types (applying `f`'s return type to each). `filter_map`, by contrast,
  has a result size that varies with the predicate, so it can't keep per-position information and
  is **forcibly widened** to `Array[T]`. A natural consequence of type theory: "only an operation
  that doesn't change positions can keep a Tuple's precision."
- the *depth* of record subtyping (comparing recursively down to value types), read-only, and
  other RBS-record details.
- types raised from `Struct` / `Data.define` (real Rigor's `DataClass` / `DataInstance`).

## Exercises

1. What type does a nested hash `{ a: { b: 1 } }` become? Confirm with `annotate`.
2. Confirm that `a = [1, "x"]\na[99]` becomes `nil`, and explain why it doesn't error.
3. How is a string key `{ "a" => 1 }` handled now (only symbol keys are supported)? What caution
   is needed if you widen the support?

---

**Next chapter (Part 7):** at last we build `accepts`, which judges whether types *fit* each
other. With the three values `:yes` / `:no` / `:maybe`, the "open" policy decided here comes into
real effect too.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part6/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part6/lib)
