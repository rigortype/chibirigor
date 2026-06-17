---
title: Part 4 — Union: when a type doesn't settle on one
description: "Introduce the type `Union` for when a type doesn't settle on one, and gather the branch types of an if / ternary into a single type."
sidebar:
  order: 5
---

# The Little chibirigor Part 4 — Union: when a type doesn't settle on one

This chapter's goal: **introduce the type `Union` for when a type doesn't settle on one.** In
Ruby, returning a different type per branch of an `if` or a ternary is everyday. When that
happens, instead of forcing the type to one, we hold it together as "either one" — that's
Union.

> [!NOTE]
> The Union we build (an **untagged union** like `Integer | String`) is in fact the starting
> point the reference books *deliberately avoided*. Both 『しくみ』 and TAPL hold *tagged*
> variants — values labeled with a tag to tell them apart — which is a different thing from an
> untagged union. But for us, dealing with Ruby, an untagged union is essential. (For the
> difference, see appendix [a5-4](../appendix/a5-other-languages.md).)

---

## 4-1. When a type doesn't settle on one — Union

Consider this Ruby:

```ruby
x = rand < 0.5 ? 1 : "a"
```

Is `x`'s type `Integer`? `String`? — **it can be either.** When this happens, instead of
forcing the type to one, we make it a type that says "either `Integer` or `String`." That's the
**Union**:

```ruby
module Type
  Union = Data.define(:members) do
    def to_s = members.map(&:to_s).join(" | ")   # e.g. "Integer | String"
  end

  module_function

  # A small tool that combines types: flatten nesting, drop duplicates
  def union(types)
    flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
    flat.size == 1 ? flat.first : Union[flat]
  end
end
```

The little `union` tool does just two things. **Flatten nesting** (when a `Union` shows up
inside a `Union`, level it out), and **drop duplicates** (when the same type appears twice, make
it one). If the combined result has a single member, it returns that type itself rather than
bothering to wrap it in a `Union` (`Integer | Integer` is just `Integer`).

The type of an `if` (a ternary is the same `IfNode` in Prism) becomes **the combination of the
then-branch's and else-branch's types**:

```ruby
when Prism::NilNode
  Type::Const[nil]            # the type of the nil literal. Sits in a Union member like any other
when Prism::IfNode
  then_type = type_of(node.statements.body.last, scope, diagnostics)
  else_type =
    if node.subsequent        # is there an else (or elsif)?
      type_of(node.subsequent.statements.body.last, scope, diagnostics)
    else
      Type::Const[nil]        # no else → nil when false, matching real Ruby
    end
  Type.union([then_type, else_type])
```

> [!NOTE]
> Here `node.subsequent` is a `Prism::ElseNode` for an `else` clause (an `IfNode` for `elsif`).
> We find the type from its `.statements.body.last` (the *last expression* of that clause) — note
> that we **don't pass `node.subsequent` straight to `type_of`** (if you do, it falls to
> `untyped` as an unknown node).

We treat `nil` as an ordinary type, `Const[nil]`, too, and an `if` with no `else` mixes "`nil`
when false" straight into the Union. So both `c ? 1 : nil` and `if cond then 1 end` are
plainly `1 | nil`.

Check with `annotate` / `type_of` and a Union duly comes out:

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => 1 | "a" (both branches union as Const)
```

![Figure 4-1 — the type of if / ternary: union then and else](../figures/svg/little-4-1.svg)
> ▼ Figure 4-1 — the type of `if` / ternary: union the then and else branches (the reverse of Figure 5-1)

- **① Type theory:** when a value can be more than one type = a union type (the area 『しくみ』
  deliberately avoided).
- **② In Ruby:** returning different types per branch is everyday. You write `x = cond ? 1 : "a"`
  normally.
- **③ In Rigor:** don't fix it to one; hold it as a Union. Not deciding = no trouble later.

> [!NOTE]
> `nil` (`NilClass`) sits in a Union's members like anything else. A `User | nil` — the
> "value if found, nil if not" shape so common in Ruby — is just a Union too. *How to peel off*
> this `nil`-containing Union is the lead of the next chapter, Part 5.

> [!NOTE]
> When you read something *out of* a Union (e.g. `(Integer | String).to_s`), the basic move is to
> **consider each member one at a time and combine.** This "run over all members and take the
> weakest conclusion" thinking returns, exactly, in Part 7's `accepts` (`:yes` / `:no` /
> `:maybe`). Keep it in a corner of your mind.

> [!NOTE]
> A branch "narrowed until zero candidates remain" has a type-theory name — the **bottom type**
> (`never`). The body of this book doesn't build the bottom type *as a type*; that story is
> gathered in appendix [a1](../appendix/a1-special-types.md).

---

## 4-1x. A note: method sends to a Union receiver (distribute and fold)

> [!NOTE]
> This is a note set aside from the main line. This chapter focuses on *building* a Union and
> didn't step into sending a method to one. Here we layer on what happens to a Union receiver if
> we add *a spoonful* to Part 2's dispatch table. The `union` of 4-1 and the typing of `IfNode`
> stay as is.

With `x = cond ? 1 : 2`, `x` is `1 | 2`. So what's the type of `x + 1`? This chapter's minimal
version (and Part 2's naive dispatch table) rounds the receiver's type to a single class name
with `class_of` and looks up the table. A `Union` doesn't round to a class name (`class_of` is
`nil`), so **the table can't be looked up and it quietly falls to `untyped`** — the fail-soft
exit. It doesn't frighten, but it throws away the precision of that hard-won `1 | 2`.

The real `exe/chibirigor` steps in here. **A Union receiver looks up the table per member, and
folds the resulting return types with `Type.union`** (`dispatch_union` in
`lib/chibirigor/dispatch.rb`):

```ruby
# Distributive dispatch for a Union receiver. At run time it can be any member,
# so dispatch per member and fold the results with union.
def dispatch_union(receiver_type, name, arg_types, node, diagnostics)
  buffers = []
  results = receiver_type.members.map do |member|
    buffers << (buffer = [])
    dispatch(member, name, arg_types, node, buffer)   # look up the table for one member at a time
  end
  diagnostics.concat(merge_member_diagnostics(buffers))
  budgeted_union(results)                              # fold the results (overlaps collapse to one)
end
```

A Union on the argument side is the same idea. The constant-folding stage of 2-7 expands
arguments into the **product of members** and folds per combination (`const_combinations`) — for
`1 + (1 | 2)`, it computes both `1+1` and `1+2` and gets `2 | 3`. Run for real, both receiver
distribution and argument distribution come out like this (`exe/chibirigor annotate`):

```text
x = cond ? 1 : 2 ; x + 1        # 2: 2 | 3      (distribute receiver (1|2) and fold)
a = 1 ; a + (cond ? 1 : 2)      # 2: 2 | 3      (expand argument (1|2) into the product and fold)
x = cond ? 1 : "a" ; x + 1      # 2: 2 | String (fold the Integer side; the String side goes to the table's return type)
```

This behavior is continuous with the zero-false-positive principle. What to do when **the
distribution splits** — for `x = cond ? 1 : "a"`, the `x + 1` passes for `1 + 1` and is a type
error for `"a" + 1`. But at run time, if `x` fell to the `Integer` side, it works. So we **complain
only when all members fail, and stay quiet about a partial failure** (`:maybe`). Only an
expression that fails for *either* of `(1 | 2)`, like `x + "a"`, becomes a single diagnostic.
**If there's an unknown member**, we're more conservative still: for `x = cond ? 1 : nil`, the
`x + 1` collapses the whole Union to `untyped` the moment `nil.+` isn't in the table (lose track of
even one type, and we don't assert precision for the whole).

The real behavior's spec-cum-samples is **`test/test_union_dispatch.rb`** (covering receiver
distribution, the argument product, complaining only when all members fail, `untyped` on an
unknown member, and rounding to a class on the member-count budget). Read it as the *sequel* to
4-1's `annotate` output (`rand < 0.5 ? 1 : "a"` → `1 | "a"`): send a method to that `x` and
distribution happens. The reason `(1 | 2) + 1` shows up as `2 | 3` when you run
`exe/chibirigor` yourself (the chapter's minimal version would say `untyped`) is that this distribution
lives on the Dispatch side.

> [!NOTE]
> **In real Rigor**, a `Union` receiver is specified as "dispatch each member individually; if
> all members resolve, union the return types; if even one fails to resolve, make the whole
> `nil` (resolution failure)" (`rigor/docs/internal-spec/inference-engine.md`: "`Union` receivers
> MUST dispatch each member individually"). chibirigor's "if there's an unknown member, the whole
> is untyped" is a miniature of this.

---

## 4-2. This chapter's summary

What we added is one type carrier, `Union`, the combining tool `union`, and the typing of
`IfNode` (which `union`s the then-branch's and else-branch's types). The little `union` tool is
just the two of "flatten nesting, drop duplicates." With this, we can plainly express, at the
type level, Ruby where "a type doesn't settle on one."

Running it:

```ruby
x_int_str = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").last[:type]
x_int_nil = Chibirigor.annotate("x = c ? 1 : nil\nx\n").last[:type]
puts "c ? 1 : \"a\"  ->  #{x_int_str}"
puts "c ? 1 : nil   ->  #{x_int_nil}"
```

```text
c ? 1 : "a"  ->  1 | "a"
c ? 1 : nil   ->  1 | nil
```

`c ? 1 : "a"` has then-branch `1` and else-branch `"a"`. Rather than forcing it to one or the
other, we combine into a Union, `1 | "a"`. With `nil` on the else side it's likewise `1 | nil`.

This chapter's three perspectives:

| | Content |
|---|---|
| ① Type theory | A value can be more than one type = a union type (the area 『しくみ』 *deliberately avoided*; TAPL has no direct chapter either) |
| ② Ruby / RBS | Returning different types per branch is everyday. You write both `x = cond ? 1 : "a"` and `User | nil` normally |
| ③ Rigor's implementation problem | Don't force it to one; hold it as a Union. Not deciding = no trouble later |

## Exercises

1. Checking the type of `rand < 0.5 ? 1 : 2` with `annotate` gives `1 | 2` (both branches stay
   `Const`). So what does `rand < 0.5 ? 1 : 1` give? Explain via how the `union` tool folds *the
   same member*.
2. Checking the type of the else-less `if cond\n  1\nend` with `annotate` gives `1 | nil`
   (matching how real Ruby returns `nil` when an else-less `if` is false). Explain how `union`
   combines the two, in terms of the member order.
3. What does `Union[[Integer, Union[[String, Integer]]]]` return through `union`? Answer by
   applying the three of "flatten nesting," "drop duplicates," and "don't wrap a single member"
   in order.

---

**Next chapter (Part 5):** Union was an operation that *grows* a type. Next chapter we build the
reverse — **narrowing**, which *shrinks* a Union. In the else clause of `if x.nil?`, we tighten
the type to "the `x` here is no longer `nil`" — making that obvious move traceable in types too.
There we'll cover: false is just `false` / `nil`; the `narrow` implementation; the dead branch of
`is_a?`; the two laws of narrowing; and resetting on reassignment.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part4/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part4/lib)
