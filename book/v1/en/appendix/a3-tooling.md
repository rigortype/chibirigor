---
title: Appendix a3 — Tooling (real Rigor's CLI and the dispatch cascade)
description: A reference appendix gathering in one place the real-Rigor tool behaviors the main volume sent off with a one-line pointer (`check --explain`, the two-layer type display / erasure, `trace`, the 5-stage dispatch cascade).
sidebar:
  order: 23
---

# Appendix a3 — Tooling (real Rigor's CLI and the dispatch cascade)

This is a **reference appendix.** Since this book's (chibirigor's) main purpose is building a
"working minimal version" by hand, at various points in the main volume we sent the real **Rigor**'s
tool behaviors off with a *one-line pointer.* This is where they pointed. We lay out the difference
between chibirigor's minimal version and real Rigor with the bridge "**in this book it's naive /
the real thing is like this.**"

> **Pointers back from the main volume**
>
> - From Little **Part 2** (method sends and dispatch): the dispatch we kept to a naive
>   "single-stage table lookup," the real thing looks up with a **5-stage cascade** → this
>   appendix §a3-3.
> - From Little **Part 9** (the philosophy of gradual): the mechanism by which `rigor check
>   --explain` lists `Dynamic[Top]`'s *fail-soft* sites as a "map" → this appendix §a3-1.
> - From the column in Little **Part 1** (literals and arithmetic): the mechanism by which real
>   Rigor holds an internal precise type and an RBS-boundary conservative type in two layers
>   (erasure) → this appendix §a3-2.

The factual descriptions of Rigor written here are made consistent with the main volume's
manuscript (the order and names of the 5 stages are as in the manuscript). They are not
descriptions that change this book's code behavior.

---

## a3-1. `rigor check --explain` — produce a map of fail-soft

A normal `rigor check` reports as diagnostics **only problems it could prove**, and stays quiet
about sites it *fail-softed* to `Dynamic[Top]` (the internal notation of the main text's minimal
`Dynamic` = `untyped`; see appendix a1-1) (the real thing behind Little Part 2's "stay quiet if you
don't know" and Part 9's "deliberately miss"). This is an attitude for not producing false
positives, but turned over, it's also "**quietly missing.**"

Add `--explain` and **every fail-soft site appears as an `:info` diagnostic** — a map of "I lost
the type here" is output.

```console
$ rigor check --explain app/models/user.rb
app/models/user.rb:42:7: info: fell soft to Dynamic[Top] here (RBS not found for `external_call`)
app/models/user.rb:51:3: info: fell soft to Dynamic[Top] here (param `opts` is untyped)
```

Its uses are these:

- When the question "am I overlooking this bug?" arises, trace back **where the type vanished** with
  `--explain`'s output.
- From the fail-soft site you reach, you can discover **missing RBS, an unconfigured plugin, gaps in
  type annotations.**

### Why a "map" can be drawn — recovering the `Dynamic[Top]` marker

This listing works because the **`Dynamic` marker of `Dynamic[Top]`** touched on in Little Part 1
**doesn't vanish** at the fail-soft site. By holding `untyped` not as "just a hole" but as "a `Top`
marked with `Dynamic`," "where it stayed quiet" **remains as structure.** That's exactly why it can
be raised into a list later with `--explain`.

| | This book (chibirigor) | The real thing (Rigor) |
|---|---|---|
| What `untyped` really is | `Type::Dynamic` (just a mark) | `Dynamic[Top]` (`Top` with a `Dynamic` marker) |
| The fail-soft site | just quietly returns `Dynamic` | keeps the site in structure, lists it with `--explain` |
| Making the silence visible | no mechanism (out of the minimal version's scope) | `check --explain` produces a map with `:info` diagnostics |

chibirigor's "stay quiet if you don't know" prevents false positives, but `--explain` is **a tool
that makes that silence itself visible.**

### a3-1x. A note: chibirigor has a tiny `--explain` too

The table above marks this book as "making silence visible = no mechanism," but we added a **tiny
version** here too. Add `check --explain` and it also emits, as `:info` diagnostics, the sites where
inference toppled to `untyped` (the dispatch of an unknown method):

```console
$ printf 'x = mystery_call\ny = x + 2\n' > demo.rb
$ ruby exe/chibirigor check --explain demo.rb
demo.rb:1:5: info: fell to untyped here (can't look up the type of `mystery_call`)
  x = mystery_call
      ^^^^^^^^^^^^
demo.rb:2:5: info: fell to untyped here (can't look up the type of `+`)
  y = x + 2
      ^^^^^
```

What to notice is **line 2.** `mystery_call`'s type is unknown so `x` becomes `untyped`, and the
`+` against that `x` can't look up a type either and topples to `untyped` — the **silence
propagating** shows up on the map. Without `--explain` it just stays quiet with `No type errors`
(since it produces no false positives). `:info` doesn't soil the exit code (`exit 0`), so you can
peek at "where the type vanished" without stopping CI.

The difference from the real thing is that real Rigor keeps `Dynamic[Top]`'s `Dynamic` marker in
structure and lists *every* fail-soft (missing RBS, an unannotated argument, an unconfigured
plugin…), whereas chibirigor picks up **just one kind, the unknown dispatch** (the implementation
just stacks one line of provenance in the `signature`-is-nil branch of `lib/chibirigor/dispatch.rb`).
The idea of **mapping the silence** is the same.

---

## a3-2. Rigor's type display — the internal precise type and the RBS-boundary conservative type

This book's `annotate` shows the inferred type, *one* of it, as is. Real Rigor too shows types with
`annotate` (a tool of the same name as this book's) and editor hover, but the real thing has a
**two-layer setup** this book lacks:

1. The **precise type** inside Rigor (e.g. `Constant<"FOO">`).
2. The conservative type after **dropping to a coarse type** at the RBS boundary (e.g. `String`).

chibirigor's `annotate` shows only the internal type. The real thing has a double structure of
"**knowing precisely inside, throwing it away at the boundary**," so "inference seems to know more,
yet why is the signature (RBS) so coarse" happens. The answer is this two-layer setup.

> The boundary operation of dropping a precise internal type to a coarse type expressible in RBS,
> Rigor calls **erasure** (a different thing from Java generics' runtime "type erasure" — that's
> runtime, this is rounding static precision at the boundary). The mechanism is treated in the
> Seasoned volume.

| | This book (chibirigor) | The real thing (Rigor) |
|---|---|---|
| How types are shown | `annotate` lists internal types per line | shows with `annotate` / hover, rounds at the boundary with erasure |
| The type shown | one internal type | two layers: the internal precise type / the boundary-rounded conservative type |
| What the discrepancy is | (doesn't happen, since there's no boundary) | the divergence between internal precise type and RBS-boundary type = erasure |

> The chibirigor side narrows its tools to the two of `check` and `annotate`. When you want to see
> the inferred type, `annotate` (per line) suffices. The two-layer setup of "precise inside, coarse
> at the boundary" and erasure are scenery you meet when you advance to real Rigor.

> **When you want to see only "some expression's" type pinpoint in real Rigor**, besides `annotate`
> (the whole file) there's the move of writing `dump_type(expr)` in the source and `check`-ing
> (a bare `dump_type(x)` needs `include Rigor::Testing`, but fully qualified as `Rigor.dump_type(x)`
> the include isn't needed). At run time `dump_type` just returns the value as is — when you `check`,
> it **prints the inferred type at that position as an `:info` diagnostic** (the same idea as
> PHPStan's `dumpType()`, riding the same `:info` mechanism as §a3-1's `--explain`. Not an error, so
> sprinkling them doesn't turn diagnostics red). The type editor hover shows is the same inference,
> but that's backed by a **low-level API for tools** — not a command a human strikes directly.

And **chibirigor also has `dump_type` as a basic feature.** Tools stay narrowed to the two of
`check` and `annotate` — this is *not a command* but an **expression** `check` recognizes. No
`include` needed; just write `dump_type(expr)` and `check`:

```console
$ printf 'x = c ? 1 : "a"\ndump_type(x)\n' > demo.rb
$ ruby exe/chibirigor check demo.rb
demo.rb:2:1: info: dump_type: 1 | "a"
  dump_type(x)
  ^^^^^^^^^^^^
```

Being `:info`, **the diagnostic doesn't turn red** (the exit code is 0), and it doesn't get in the
way emitted alongside real type errors. The mechanism is the `:info` diagnostic built in Little
Part 9 as is — when `type_of` finds `dump_type(expr)`, it records the argument's inferred type in an
`:info` diagnostic (which `check` brings out) and returns the value as is (so `dump_type(x)`'s type
stays `x`'s type). The implementation is `lib/chibirigor/type_of.rb`; the behavior coverage is
`test/test_dump_type.rb`.

---

## a3-3. The 5-stage dispatch cascade — the real table lookup

Little Part 2 kept the typing of a method send to a **naive single-stage table lookup** (look up the
`METHODS` table by `(class, method)`; if found, the return type, if not, `untyped`). Real Rigor
makes this "table lookup" a **5-stage cascade.** **It applies from the top stage in order, and if a
stage can't apply, falls through to the next.** What each stage resolves, and what it passes to if it
misses:

| Stage | Name | What it applies | If it misses |
|---|---|---|---|
| ① | **constant folding** | if both sides are known constants like `1 + 2`, **actually compute** on the spot and emit the result type (`3`) | to ② |
| ② | **shape dispatch** | operations that **directly touch the type's structure**, like a `HashShape` key read, solved directly from the structure | to ③ |
| ③ | **RBS** | look up with the **RBS types** that core, stdlib, and plugins provide (the real thing of this book's hand-written `METHODS` table) | to ④ |
| ④ | **in-source** (body inference) | a method not in RBS, **infer the body** and synthesize a return type (the real thing of Little Part 8's return-type synthesis) | to ⑤ |
| ⑤ | **fallback** | if no stage hits, degrade to **`Dynamic[Top]`** (don't frighten) | — (stops here) |

### Reading the flow

A single call is asked, from the top in order, "can this stage solve it?" and stops at the stage
that solves it. A stage that can't just quietly passes to the next — **producing no false positives,
and always catching at the end with `Dynamic[Top]`**, so it doesn't stop even on an unknown call
(the real thing of Little Part 2's "don't frighten an unknown method").

```text
  receiver.method(args)
    │
    ▼
  ① constant folding ── hit ─→ the result type (e.g. 3)
    │ miss
    ▼
  ② shape dispatch ──── hit ─→ a type solved directly from the structure
    │ miss
    ▼
  ③ RBS ─────────────── hit ─→ an RBS-derived return type
    │ miss
    ▼
  ④ in-source ───────── hit ─→ a return type synthesized by body inference
    │ miss
    ▼
  ⑤ fallback ──────────────→ Dynamic[Top] (untyped)
```

### An example where priority matters — why ③ beats ④

The cascade's **order itself carries meaning.** For instance, a type declared with an
`RBS::Extended` directive **beats** the inference of a method body because ③ RBS **hits before**
④ in-source. The design judgment "prefer the declaration over the body" shows up as the ordering of
the stages.

| | This book (chibirigor) | The real thing (Rigor) |
|---|---|---|
| Number of dispatch stages | 1 stage (just look up `METHODS`) | 5 stages (① constant folding → ② shape → ③ RBS → ④ in-source → ⑤ fallback) |
| Table contents | a hand-written `METHODS` Hash | ③ is RBS (from core, stdlib, plugins) |
| Body inference | separately in `annotate` (Little Part 8) | built into dispatch as ④ in-source |
| Handling the unknown | returns `Dynamic` | `Dynamic[Top]` in ⑤ fallback |
| Declaration vs. inference priority | (no distinction) | expressed by stage order (③ before ④) |

> This book's Part 2 kept dispatch to one stage because listing ③ RBS (not learned until Little
> Part 8) and ④ in-source while still unlearned would leave the story floating. The full 5-stage
> picture is factored out here so a reader who has finished Part 8 can survey it at a glance.

---

## a3-3b. `rigor trace` — see the steps of inference frame by frame

The tools so far (`--explain`, the two-layer type display, the cascade) showed inference's **answer**
or **map.** Real Rigor has one more, a tool that shows the **steps of inference themselves** —
`rigor trace`. It re-runs the same inference `check` runs against a file, and replays the recorded
inference events as a **frame-by-frame terminal animation.** One frame = one scene of inference,
showing the moment a local enters scope (`bind`), the moment a branch's types dissolve into one
union (`union`), and the moment a method send resolves (or fail-softs to `Dynamic[top]`)
(`dispatch`), highlighting the range being evaluated.

```sh
rigor trace lib/example.rb              # step by keypress
rigor trace --delay=0.5 lib/example.rb  # auto-play
rigor trace --format=json lib/example.rb # raw event stream
```

`--verbose` emits everything down to per-expression enter/result; by default it narrows to just the
three "teaching points" above. The JSON event stream is stable, so it can be material for textbook
figures or lecture slides.

### a3-3bx. A note: chibirigor has a tiny `trace` too

This is, rare among the appendices, a section where **this book's side has a tool nearly identical to
the real thing** (the implementation is `lib/chibirigor/tracer.rb`). The parts you built by copying
each Part in this book — binding into scope, `Type.union`'s folding, dispatch's table lookup — flow
before your eyes **in running order.** It's a learning tool for **confirming with your eyes** the
inference a reader assembled in their head ("the evaluation order should be this," "it should become
a union here").

Let's run it on a 3-line example — containing an assignment, a ternary, and a method call, one each:

```console
$ printf 'x = 5\ny = x > 0 ? 1 : -1\nz = y + 2\n' > demo.rb
$ ruby exe/chibirigor trace demo.rb
```

In the terminal you advance frame by frame with Enter (`q` to quit). Of all 17 frames, pulling out
just the key ones:

```text
chibirigor trace ─ step 2/17
────────────────────────────────────────────────────────────────
  1  x = 5
  2  y = x > 0 ? 1 : -1
  3  z = y + 2
────────────────────────────────────────────────────────────────
type env   : x: 5
evaluating : (top level)
► bind: x ← 5 (added to type env)
…
chibirigor trace ─ step 5/17
…
type env   : x: 5
evaluating : if (incl. ternary) › call to >
► dispatch: 5.>(0) → untyped (not in table → fail-soft to untyped)
…
chibirigor trace ─ step 7/17
…
evaluating : if (incl. ternary)
► union: 1 , -1 → 1 | -1
…
chibirigor trace ─ step 9/17
…
type env   : x: 5   y: 1 | -1
evaluating : (top level)
► bind: y ← 1 | -1 (added to type env)
…
chibirigor trace ─ step 12/17
…
evaluating : call to +
► dispatch: 1.+(2) → 3 (constant folding)
chibirigor trace ─ step 13/17
…
► dispatch: -1.+(2) → 1 (constant folding)
chibirigor trace ─ step 14/17
…
► union: 3 , 1 → 3 | 1
chibirigor trace ─ step 15/17
…
► dispatch: 1 | -1.+(2) → 3 | 1 (distribute Union to members)
…
chibirigor trace ─ step 17/17
…
type env   : x: 5   y: 1 | -1   z: 3 | 1
evaluating : (top level)
► bind: z ← 3 | 1 (added to type env)

── playback done (17 steps total) ──
```

In these 17 frames, the main volume's parts line up in running order. `x`'s binding (step 2) → the
ternary's condition `x > 0`, being a `>` **not in the table**, fail-softs to `untyped` (step 5) →
the two arms `1` and `-1` dissolve into a **union** and bind to `y` (steps 7, 9) → `y + 2`, since
`y` is `1 | -1`, **distributes to members** and **constant-folds** each member (steps 12, 13),
combining the results again with a union (steps 14, 15) and binding `z` to `3 | 1` (step 17). How
the mechanisms built separately in each Part **interlock** on a single expression is surveyable at a
glance. Add `--verbose` and the thinned-out per-expression enter/result come out in full too. To
output JSON it's `--json`; auto-play is `--delay 0.5`.

The mechanism is the same idea as the real thing, and **it doesn't touch the core at all.** It just
inserts hooks into `type_of` / `eval_statement` / `Type.union` / `Dispatch.dispatch` with
`Module#prepend`, and when the recorder is `nil` (= not tracing) the hook immediately `super`s. So
`check`'s and `annotate`'s behavior is unchanged, **soiling not one line of the code copied in the
main volume** (see the head comment of `tracer.rb`).

| | This book (chibirigor) | The real thing (Rigor) |
|---|---|---|
| What it shows | frame-by-frame inference events (`bind` / `union` / `dispatch`) | the same (replays inference's derivation) |
| Interference with the core | `Module#prepend` hooks, immediate super if the recorder is nil | a record→replay probe riding the same inference as `check` |
| Output formats | terminal animation / `--json` / `--verbose` / `--delay` | terminal animation / `--format=json` / `--verbose` / `--delay` / `--line` |

There are leafy differences — the real thing can narrow to a single line with `--line=N`, chibirigor
has no line filter — but the idea of **replaying and showing the steps of inference** and the three
event kinds are the same. Where this book's other tools were "Rigor's real thing → tiny version,"
`trace` is the rare one where **this book's side lines up nearly identical to the real thing.**

---

## a3-4. Summary — a quick reference of the "naive / real" correspondence

The four bridges of this appendix on one sheet:

| Mechanism | Treatment in this book | The real thing's behavior | Pointer back |
|---|---|---|---|
| `rigor check --explain` | tiny version present (maps unknown dispatch with `:info` · §a3-1x) | maps fail-soft sites with `:info` using the `Dynamic[Top]` marker as a clue | Little Part 9 |
| two-layer type display (erasure) | `annotate` shows one internal type (no boundary, so no discrepancy · §a3-2) | rounds internal precise type ↔ RBS-boundary type with erasure behind `annotate` / hover | Little Part 1 |
| `rigor trace file` | tiny version nearly identical to the real thing (frame-by-frame `bind` / `union` / `dispatch` · §a3-3bx) | replays the steps of inference as a terminal animation (`--verbose` / `--line` / `--format=json`) | — |
| 5-stage dispatch cascade | 1-stage table lookup (`METHODS`; no tiny version) | ① constant folding → ② shape → ③ RBS → ④ in-source → ⑤ fallback | Little Part 2 |

Each can be read as a correspondence in which the skeleton hand-built in this book (the `Dynamic`
marker, `annotate`, the `METHODS` table) runs, in real Rigor, **as that same skeleton scaled up.**
