---
title: Part 3 — Local variables and an immutable Scope
description: "Remember a type with `x = 1` and make `x` readable. Learn the \"never overwrite\" design through an immutable Scope."
sidebar:
  order: 4
---

# The Little chibirigor Part 3 — Local variables and an immutable Scope

This chapter's goal: **remember a type with `x = 1`, and make `x` readable later.** To do that
we introduce a "variable name → type" mapping — the **type environment (Scope)** — and thread
it from statement to statement.

> This corresponds to 『しくみ』 ch. 3–4 (TAPL ch. 9 + ch. 11 §11.5 "let-bindings"). That book
> called the type environment `tyEnv` (a variable-name → type mapping) and carried it around by
> copying it, `{ ...tyEnv, x: type }`. Our `Scope` is the same thing.

---

## 3-1. Where variables are remembered — Scope

When we write `x = 1`, we want to remember "`x` is `Integer` (`1`, precisely)," and return that
type when we read `x` later. We need a place to keep it. That's the **Scope** (type
environment) — just a "variable name → type" mapping:

```ruby
class Scope
  def initialize(locals = {})
    @locals = locals.freeze
  end

  def local(name)        # the type for that name (nil if unbound)
    @locals[name]
  end

  def with_local(name, type)   # return a "new" Scope with one binding added
    Scope.new(@locals.merge(name => type))
  end
end
```

This is the same thing you do in your head when reading code. See the line `x = 1` and you
remember "`x` is a number"; a few lines down, when `x` appears, you recall "that number from
before" — Scope is that *mental note* turned into data a program can hold. A type checker, like
a person, needs a note it can consult when it sees a variable: "what was this again?"

The point is that it's **immutable.** `with_local` *doesn't change* the original Scope; it
returns a new Scope with a binding added. The same manner in which 『しくみ』 copied
`{ ...tyEnv, x: type }` without destroying `tyEnv` — we just gave it object form.

Why go to the trouble of making it *immutable*? Rewriting an ordinary `Hash` with
`@locals[name] = type` would seem to work too. The reason runs slightly ahead — in Part 4–5 we
want to hold *separate* notes per `if` branch: "**inside this branch only**, `x` is `Integer`."
The note we mean here is one **the type checker holds internally**, a different matter from
Ruby's runtime variable scope. If we kept rewriting one internal `Hash` each time, a note added
inside a branch would linger into the *checking* outside the branch. With an immutable design
that returns a new Scope, we can build "a note for this branch only" and hand it on **without
fouling the original.** The benefit is hard to see now, but this "add without changing the
original" property pays off in the narrowing of later chapters.

- **① Type theory:** the mapping that remembers variables' types = the type environment, tyenv
  (『しくみ』 ch. 3–4).
- **② In Ruby:** local variables are everyday. `x = ...; ...x...`.
- **③ In Rigor:** Scope is immutable. Adding a binding returns a new Scope (real Rigor uses the
  same immutable design).

---

## 3-2. Reading a variable

It's just one line added to `type_of`. Reading a local variable looks the type up from the
Scope:

```ruby
when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
```

We make `type_of` take a `scope` (passed along through the recursion that finds the receiver's
and arguments' types, too). If unbound, it's `Dynamic` (don't frighten it).

> A small Ruby subtlety: **a bare name with no assignment (`y`) is a method call (`self.y`), not
> a variable.** So "you used `y` before assigning it" is not a reference to an unbound variable
> but a call to an *unknown method*, which flows into Part 2's dispatch and quietly degrades to
> `Dynamic`. It doesn't get angry.

---

## 3-3. Threading statements

The `1 + 2` and `foo.bar` so far, when evaluated, **only produced a type.** But `x = 1` is
different — on top of producing a type (`1`), it leaves an effect behind: "**`x` is usable from
here on.**" Something that not only produces a value but also **grows the scope** like this we
call a "statement." So as not to drop that "takes effect later" part, we write a function that
evaluates one statement and returns **`[the statement's type, the updated scope]`**:

```ruby
def eval_statement(node, scope, diagnostics)
  case node
  when Prism::LocalVariableWriteNode
    type = type_of(node.value, scope, diagnostics)   # find the type of the RHS…
    [type, scope.with_local(node.name, type)]        # …and return a new scope binding that name
  else
    [type_of(node, scope, diagnostics), scope]       # non-assignments don't change the scope
  end
end
```

`check` and `annotate` evaluate the sequence of statements top to bottom, **threading the
scope** as they go (handing the scope updated by the previous statement to the next — the same
motion as reading code top-down while remembering "what's defined so far"):

```ruby
scope = Scope.new
program.statements.body.each do |stmt|
  _type, scope = eval_statement(stmt, scope, diagnostics)   # update scope and move on
end
```

Now "use, below, a variable defined above" is traceable in types too:

```ruby
check("x = 1\nx + 2")        # OK
check("x = \"a\"\nx + 1")    # ["expected String but got 1"]
```

- **① Type theory:** evaluate statements in order, advancing while growing the environment
  (the sequencing of 『しくみ』 ch. 4).
- **② In Ruby:** top to bottom, a defined variable is visible later.
- **③ In Rigor:** return `[type, scope]` and thread it. Because the scope is immutable, "where
  what is visible" is crisp.

---

## 3-4. Reassignment changes the type

In Ruby you can put a different type back into the same variable. `with_local` simply overwrites
the binding, so this traces naturally too:

```ruby
# annotate("x = 1\nx\nx = \"a\"\nx\n")
1: 1       # x is 1
2: 1       # read x → 1
3: "a"     # reassign x to "a"
4: "a"     # read x → "a" (the type changed)
```

```ruby
check("x = 1\nx = \"a\"\nx + 1")   # after reassignment x is String → ["expected String but got 1"]
```

Here 『しくみ』 made an issue of "should redefinition in the same block be an error," but,
deciding it wasn't the main line of learning, dropped shadowing handling. So do we.
Reassignment is treated straightforwardly as swapping the type.

---

## 3-5. This chapter's summary

What we added is `Scope` (the immutable type environment) and `eval_statement` (threading
statements). `check` / `annotate` now carry a scope around.

Running it:

```ruby
Chibirigor.annotate("x = 1\nx\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
puts Chibirigor.check("x = \"a\"\nx + 1").map { |d| d[:message] }.first
```

```text
1: 1
2: 1
expected String but got 1
```

The type `1` of `x = 1` is carried straight to `x` on the next line (`1: 1` → `2: 1`), and after
`"a"` is reassigned, `x + 1` passes `1` (Integer) to `String#+`, so it's a type error.

This chapter's three voices:

| | Content |
|---|---|
| ① Type theory (『しくみ』 ch. 3–4 / TAPL ch. 9, 11) | The type environment tyenv that remembers variables' types; sequencing that threads statements |
| ② Ruby / RBS | Reassignment changes the type; a bare name with no assignment is a method call |
| ③ Rigor's implementation problem | An immutable Scope makes "where what is visible" clear; reassignment is a type swap |

**Handed to the sequel:**

- fact invalidation when a block **captures** an outer local (a subtlety of real Rigor's
  FactStore).
- compound assignment `x += 1` (`LocalVariableOperatorWriteNode`), multiple assignment.
- non-local bindings: instance variables, constants, global variables, and so on.

## Exercises

1. Confirm that `x = 1\ny = x\ny + 2` passes, and trace how the type was carried.
2. Observe with `annotate` how `x`'s type changes before and after the reassignment
   `x = 1\nx = "a"`.
3. How is the compound assignment `x += 1` (`LocalVariableOperatorWriteNode` in Prism) handled
   by the current code? Work out what you'd add to `eval_statement` to support it.

---

**Next chapters (Part 4, Part 5):** in Part 4 we build the type for when `if` makes the type
branch (`Union`), and in Part 5 the "narrowing" that tightens a type by a condition like
`x.nil?`. Scope comes into its own here. This idea of an immutable scope that "swaps the binding
on reassignment" works the same way in the *narrowing* of later chapters.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part3/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part3/lib)
