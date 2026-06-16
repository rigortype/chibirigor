---
title: Part 5 — Narrowing: splitting by case
description: "Narrow the types that Union grew, per branch of an if / case. Build the `narrow` implementation and the two laws of narrowing."
sidebar:
  order: 6
---

# The Little chibirigor Part 5 — Narrowing: splitting by case

This chapter's goal: **in the last chapter we got the Union, where a type "doesn't settle on
one." Now we build the machine (narrowing) that *tightens* a variable's type, per branch of an
`if` / `case`.** What Ruby code does as a matter of course — "we nil-checked, so from here it's
not nil" — we make traceable in types too.

By introducing the Union in the last chapter (Part 4), a single variable came to hold several
types, like `User | nil`. If the type grew, the next turn is to **shrink** it (we add per-branch
narrowing to the `IfNode` typing we wrote in the last chapter). Inside the branches of an `if` /
`case`, a person reads "in this branch the type is this" unconsciously. Tracing that in types is
this chapter's subject.

---

## 5-1. Tightening a type by case — narrowing

Look at this Ruby:

```ruby
x = find_user   # type is User | nil (nil if not found)
if x.nil?
  puts "not found"
else
  puts x.name   # here x is definitely not nil → User
end
```

A person reads "inside the `else`, `x` is not `nil`" as a matter of course. Tracing that in
types is **narrowing.** Per branch of a conditional, we *tighten* a variable's type.

- In the **then-branch** of `if x.nil?`, `x` is `nil`.
- In the **else-branch**, `x` is the rest with `nil` removed (`User | nil` → `User`).

```text
              x : User | nil
                    │
            if x.nil?
          ┌─────────┴─────────┐
       then branch          else branch
     x : nil          x : User (nil removed)
          └─────────┬─────────┘
              union both branches
```

![Figure 5-1 — narrowing if x.nil?](../figures/svg/little-5-1.svg)
> ▼ Figure 5-1 — narrowing `if x.nil?`

We type each body in a **separate Scope** with `x`'s type swapped per branch, and at the end
union the two branches' results.

> **Column: this is about catching "crash on nil" in the types**
>
> `User | nil` is "a Union containing `nil`." Narrowing is the machine that, in the else-branch
> of `if x.nil?`, **strips** `nil` from the type as "the `x` here is no longer `nil`" — and if you
> call `.name` somewhere the stripping isn't complete (`nil` still remains in the type), that's a
> "place that crashes on nil." Here one's view shifts: a crash-on-nil bug isn't "something that
> happens to crash at run time" but a bug **expressible in types, and preventable in types.** This
> shift of view is what's called **null safety.** Without adding special syntax — just holding
> `nil` as an ordinary Union member and stripping it with a guard — you stand at its doorway. (For
> the correspondence with Java's `NullPointerException`, Kotlin's `User?`, and TypeScript's
> `User | null`, see appendix [a5-1](../appendix/a5-other-languages.md).)

---

## 5-2. Ruby's "false" is just two things — implementing the narrowing

Before the implementation, one important Ruby fact. **The only things treated as "false" in Ruby
are `false` and `nil`.** `0` and `""` are both true. So `if x` means "`x` is neither `false` nor
`nil`."

Narrowing is just "look at the condition, and build a **new scope** with the variable's type
swapped per branch." For the scope we use the immutable `Scope` from Part 3 as is (look the type
up with `scope.local(name)`, and `scope.with_local(name, type)` returns a new `Scope` with one
binding added):

```ruby
def remove_nil(t)
  return t unless t.is_a?(Type::Union)
  # nil arrives as Const[nil] for the nil literal, or Nominal[:NilClass] in the truthy branch of x.nil?. Strip both.
  Type.union(t.members.reject { |m| m == Type::Const[nil] || m == Type::Nominal[:NilClass] })
end

def narrow(scope, cond, truthy:)
  # Handle just the x.nil? shape for now (other conditions add the same way later)
  if cond.is_a?(Prism::CallNode) && cond.name == :nil? &&
     cond.receiver.is_a?(Prism::LocalVariableReadNode)
    name = cond.receiver.name
    narrowed = truthy ? Type::Nominal[:NilClass] : remove_nil(scope.local(name))
    return scope.with_local(name, narrowed)   # add a binding to the immutable Scope and return it
  end
  scope   # ★ a condition we can't narrow → return the scope as is (assert nothing)
end
```

The typing of `if` finds the then-branch in a "scope narrowed to truthy," the else-branch in a
"scope narrowed to falsy," and combines them at the end:

```ruby
when Prism::IfNode
  then_scope = narrow(scope, node.predicate, truthy: true)
  else_scope = narrow(scope, node.predicate, truthy: false)
  then_type = type_of(node.statements.body.last, then_scope, diagnostics)
  else_type =
    if node.subsequent   # there's an else clause (a ternary is the same IfNode)
      type_of(node.subsequent.statements.body.last, else_scope, diagnostics)
    else
      Type::Const[nil]   # no else → nil when false
    end
  Type.union([then_type, else_type])
```

(When there's **no else**, as in `if cond; ...; end`, `node.subsequent` is `nil`. In that case we
make the falsy branch's type `nil` — matching how real Ruby returns `nil` when an else-less `if`
is false.)

Run it and it narrows properly:

```ruby
# when x : Integer | nil
# then branch → x is NilClass
# else branch → x is Integer
```

```text
nil? narrowing: OK (no errors)
expected String but got 1
```

`is_a?` works the same way (the then-branch of `if x.is_a?(String)` narrows `x` to `String`). As
the shapes grow, you just add a branch to `narrow`. **The falsy branch returns the scope as is**
(the side where the `is_a?` condition didn't hold carries on the original Scope unchanged).

There's one pitfall with `is_a?`, though. When `x` was originally `Integer`, narrowing the body
of `if x.is_a?(String)` to "`x` is `String`" makes that branch — which *can't happen* (an Integer
never becomes a String) — treat `x + 1` as String addition and produce a **false positive.** That
violates "never frighten working code." So **narrow only when that class is possible** — narrow
when `x` contains String, as in `Integer | String`; don't when it's `Integer` alone (that branch
is a dead branch, so leave it untouched). We don't narrow `Dynamic` either (something whose type
is unknown, we let through unknown).

```ruby
check("x = 1\nif x.is_a?(String)\n x + 1\nend\n")              # OK (dead branch; no false positive)
check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n")  # String-addition error (correct)
```

> **Implementation note — the `possible?` guard.** To have `narrow` decide "is that class
> possible," a small helper is needed. `Dynamic` is false (we can't say it's impossible), a Union
> searches its members, and anything else is decided by whether the class matches:
>
> ```ruby
> def possible?(current, klass)
>   return false if current.is_a?(Type::Dynamic)
>   members = current.is_a?(Type::Union) ? current.members : [current]
>   members.any? { |m| Dispatch.class_of(m) == klass }
> end
> # in narrow_type's is_a? clause: narrow only when klass && truthy && possible?(current, klass)
> ```
>
> Without this guard, applying `is_a?(String)` to a bare `Integer` would narrow the dead branch
> to `String`, and `x + 1` would be falsely flagged as "String addition."

### Reporting an unreachable arm

Leaving the dead branch of `is_a?` "untouched (not narrowed)" is a passive way to avoid a false
positive, but Rigor goes one step further and can **point out that branch as "a superfluous branch
that is never taken."** By default, though, it stays silent; it surfaces only when you explicitly
ask with `check --unreachable` — an opt-in (so as not to frighten working code). The opposite of
Java's and C#'s exhaustiveness checks, which "stop you until you write the missing arm," this
**stays quiet about what works, and points out an unreachable branch only when asked** — another
expression of the "don't frighten" value, on the same axis as the judgment, in the `possible?`
guard above, to "not narrow when it's impossible."

> The "type" of an unreachable branch (the bottom type `Bot` / `never`) is treated in appendix
> [a1](../appendix/a1-special-types.md); the difference in direction from Java's/C#'s
> exhaustiveness checks in appendix [a5-5](../appendix/a5-other-languages.md).

---

## 5-3. The two laws of narrowing (this is what makes it Rigor)

Narrowing has two laws Rigor keeps. Both are for "don't frighten."

**Law 1: a condition you can't narrow, pass quietly as is.** The last line of `narrow` —
`scope` (return it as is) — is that. For a condition we can't read, like
`if complicated_check(x)`, we **assert nothing.** We never say "can't narrow, so it's
suspicious."

**Law 2: narrowing only "adds a fact." If you get it wrong, fall to the looser side.** Since it's
an operation that *tightens* a type, overdoing it erases "a value that's actually possible" and
becomes a source of false positives. So when in doubt, don't narrow. Also, a **reassignment** to a
variable resets all prior facts — because facts attach not to "the variable name" but to "facts
fixed at that scope position." The moment you write `x = something_else`, all narrowing memory
about `x` vanishes.

> **Ground-laying for Part 7: a Union "thinks about all members."**
> When you read something out of a Union (e.g. `(Integer | String).to_s`), the basic move is to
> consider each member one at a time and combine. `to_s` is on both Integer and String, so it's
> OK. If a method were on only one side, it gets that much more doubtful. This "**run over all
> members and take the weakest conclusion**" thinking returns, exactly, in Part 7's `accepts`
> (`:yes` / `:no` / `:maybe`). Pick it up here.

- **① Type theory:** type information grows by case (terrain of its own that 『しくみ』 doesn't
  cover).
- **② In Ruby:** only `false` / `nil` are false; guarding with `x.nil?` / `is_a?` is the standard
  move. Further — whether `x` is a *local variable* is decided by "is there an assignment earlier"
  (without one, it's treated as a call to `self.x`).[^bare]
- **③ In Rigor:** narrowing only *adds a fact*. It stays quiet about a condition it can't read. If
  it must err, err to the looser side.

[^bare]: Whether a bare `x` is a local variable or a method call is decided by Prism from context.
    Narrowing works only on local variables. We don't chase this further in the main volume.

---

## 5-4. This chapter's summary

What we added is the tools `remove_nil` / `narrow`, and the narrowing-equipped typing of
`IfNode`. `narrow` is effectively 7 lines. The scope is just Part 3's immutable `Scope` with a
binding added via `with_local`.

This chapter's three voices:

| | Content |
|---|---|
| ① Type theory | Type information grows by case (terrain of its own that 『しくみ』 doesn't cover; the dead branch = bottom type is in appendix a1) |
| ② Ruby / RBS | False is only `false` / `nil`; guarding with `x.nil?` / `is_a?` is the standard move |
| ③ Rigor's implementation problem | Narrowing only *adds a fact · stays quiet when unreadable · loosens when in doubt* = produces no false positives |

**Handed to the sequel:**

- the real **FactStore** (six kinds of "fact stores," when a fact is invalidated, the subtlety of
  discarding facts on reassignment or a block's closure capture). The main volume stops at the
  naive `Scope`. The "facts vanish on reassignment" touched here is generalized in Seasoned Part 6
  (the complete FactStore).
- narrowing for `case`/`when`, `case`/`in` (pattern matching), and detection of unreachable arms
  (real Rigor's ADR-47). The main volume stops at `if`'s `nil?` / `is_a?`.
- **the Union size budget:** the last chapter's `union` helper only drops duplicates, but in real
  Rigor, when a Union's member count exceeds a configured cap, it is **forcibly widened** to a
  Union of each member's nominal class (`Integer`, `String`, …). This is the same idea as constant
  folding's "round if it's too big" — another expression of the design principle that "types, too,
  *have a budget*."

## Exercises

1. Confirm that when `x : String | nil`, the then-branch of `if x` narrows `x` to `String` (use
   "Ruby's false is just the two of `false` / `nil`"). What type is `x` in the else-branch?
2. Confirm that when `x : Integer | String`, the then-branch of `if x.is_a?(Integer)` narrows `x`
   to `Integer`.
3. To make `unless` narrowable too, state your approach for how to change the `if` typing (hint:
   swap the truthy and falsy branches).

---

**Next chapter (Part 6):** we give types to hash and array literals (`HashShape` / `Tuple`). In
Ruby, awash in "symbol-keyed options hashes," we step into the story of how *requiring types by
exact match becomes a storm of false positives*.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part5/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part5/lib)
