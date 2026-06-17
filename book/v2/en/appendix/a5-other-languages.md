---
title: "Appendix a5 — Bridges from other languages"
description: "Null safety · nominal/structural subtyping · the lineage of structural hash types · untagged unions — a reference set linking the main volume's concepts to Java/Kotlin/TypeScript/Go/Hack/PHPStan."
sidebar:
  order: 25
---

# Appendix a5 — Bridges from other languages

This appendix is a reference set for **linking the concepts of the main volume to knowledge of
other languages.** The Little volume proceeds **presupposing only Ruby**, but for readers with
experience in Java, Kotlin, TypeScript, Go, PHP (Hack/PHPStan), and the like, we can build a
bridge that lands as "ah, the same story as that." Conversely, even without knowing these
languages, **the main volume's argument is complete** — this is a "nice to have if you know it"
bonus.

It assumes readers who came here via a "for details see appendix a5" pointer from Little Part 4
(Union), Part 5 (narrowing), Part 6 (HashShape), and Part 7 (subtyping). **Look up only the items
you need, and return to the main volume.**

---

## a5-1. Null safety — catching the "NPE" in the types (related to Part 5)

If you write Java, Part 5's `User | nil` should look familiar. `find_user` returns "if found
`User`, if not `nil`" — the same setup as Java's "`User` or `null`," with Ruby's `nil`
corresponding to Java's `null`. And call `x.name` while `x` is still `nil`, and in Ruby it's a
`NoMethodError`, in Java a `NullPointerException` (**the NPE**) — the same accident.

Narrowing is the machine that **catches that accident ahead of time, at the type level.** In the
else-branch of `if x.nil?`, it tightens the type to "the `x` here is no longer `nil`" — the same
as the habit, in Java, of writing `if (x != null) { … }` and then touching a field, except **the
type checker traces it for you automatically.** You carry around "a Union containing `nil`" and
**strip** `nil` where you pass the guard. Call `.name` somewhere the stripping isn't complete
(`nil` still remains in the type), and that's "where the NPE comes out."

For someone who took `NullPointerException` as "something that happens to crash at run time," this
is where the view shifts — **null was a bug expressible in types, and preventable in types.** This
is the core of the idea called **null safety**, and Kotlin's `User?` and TypeScript's `User | null`
are the same notion. Rigor / chibirigor hold `nil` as just an ordinary Union member and strip it
with narrowing — standing at the doorway of null safety without adding any special syntax.

---

## a5-2. Nominal and structural subtyping (related to Part 7)

If you write Java, hearing "subtype" you'll picture **inheritance** — `class Dog extends Animal`
lets you assign `Animal a = new Dog();`. That is indeed one kind of subtyping, restatable as "the
`Dog` box fits in the `Animal` box" (small box → big box). `implements` (interface implementation)
is subtyping too.

But **subtyping isn't only inheritance and implementation.** Recall Part 6 saying "a hash with
*more* keys is a subtype of a hash with *fewer* keys" — `{name:, age:}` was a subtype of
`{name:}`. Here there's neither `extends` nor `implements` — **no inheritance relation at all.**
It's
decided by *shape* alone: **as long as the structure (the keys held) is there, it's a subtype** —
this is called **structural subtyping.** Java is basically a **nominal subtyping** world ("decided
by name, by inheritance declaration"), so this is where intuition easily slips. It might click to
say Go's interfaces and TypeScript's object types are structural.

To sum up, subtyping has two lineages, "decided by inheritance (nominal)" and "decided by
structure (structural)," and **Rigor / chibirigor handle both together in one judgment: 'does it
fit in the box.'** Part 7's `accepts` is its doorway (the formal treatment of when to use nominal
vs. structural is in Seasoned Part 2).

---

## a5-3. The lineage of structural hash types — Hack → PHPStan → Rigor (related to Part 6)

"A structural hash type that remembers keys and value types" (the main volume's `HashShape`) isn't
Rigor's invention; it's the product of a history in which type checkers hit the same problem
whenever they handled a dynamic hash.

- **Hack (Facebook):** a language that added static types to PHP. It introduced the type
  `shape('name' => string, 'age' => int)` and adopted the design "spell out the keys, but allow
  extras (open)." A design conscious of coexisting with options hashes from the start.
- **PHPStan / Psalm:** PHP's checkers hit the same problem and introduced the same type with the
  notation `array{name: string, age: int}`. The vocabulary follows Hack, and some can state
  open/closed explicitly.
- **Rigor:** raises types from Ruby's RBS `{ name: String, age: Integer }`, and likewise adopts
  open. It receives with "at least."

In all three tools, a naive join (a wide type like `Hash[Symbol, String | Integer]`) loses the
per-key information, so a type that remembers keys individually was needed. chibirigor's
`HashShape` is the minimal implementation of this lineage.

---

## a5-4. Untagged unions vs. "tagged variants" (related to Part 4)

The Union built in Part 4 (an **untagged union** like `Integer | String`) is, in the world of the
reference books, the *rarer* starting point. 『しくみ』 *deliberately avoided* general union types
as "too large an impact on the type system" (touching only lightly on *tagged* variants in a ch. 5
exercise), and what TAPL holds is also a **tagged variant** (a type that attaches a tag to a value
to tell which type it is) — a different thing from the untagged union we build.

A tagged variant carries "a label that distinguishes, at run time, which type the contents are,"
but Ruby's values carry no such label — whether `x` is `Integer` or `String` you can only *check on
the spot* with `is_a?`. That's exactly why, dealing with Ruby, we essentially need an **untagged
union** that doesn't rely on a label, and the narrowing (Part 5) that tightens it by case.

---

## a5-5. The difference in direction from exhaustiveness checks — Java/C#'s missing arm (related to Part 5)

The "unreachable arm" reporting touched on in Part 5 is a design *the reverse* of Java's and C#'s
exhaustiveness checks. Java's and C#'s `switch` / pattern matching enforce **exhaustiveness**, and
if the `case`s don't cover every pattern, the compiler stops you with a "*missing arm*." Rigor (and
chibirigor) don't ask about "missing"; instead they report an "**unreachable arm**." When you write
`if x.is_a?(String)` even though `x : Integer`, that branch never executes — they find it and tell
you "this is a superfluous branch." chibirigor can actually emit this diagnostic too, with
`check --unreachable` (**opt-in**; by default it keeps "stay quiet about working code" and stays
silent. A runnable worked example is in appendix [a1-3x](a1-special-types.md)).

| | Java / C# | Rigor / chibirigor |
|---|---|---|
| What it reports | missing arm (an arm not covered) | unreachable arm (an arm never taken) |
| Stance toward working code | stops you until you write it | stays quiet about what works |
| Who loses out | a developer who knows "that pattern won't come" | a bug that "thinks it won't come but actually does" |

This is an expression of Rigor's value of prioritizing fewer false positives (don't frighten
working code) over soundness (cover every pattern).
