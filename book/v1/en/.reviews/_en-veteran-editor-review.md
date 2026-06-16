# Veteran US technical-book editor — read-feel review (English edition)

**Scope:** the whole English book under `book/v1/en/` (README, glossary, Little Part 0–9,
Seasoned Part 1–8, appendices a1–a5). Prose only. Code blocks, identifiers, RBS, CLI text
left untouched. The two settled framings (Little/Seasoned + three voices) and the
"non-rejecting / never frighten / gradual" spine are preserved in every suggestion.

---

## Verdict

**This is a genuinely good translation.** It reads, the large majority of the time, like an
English technical book that was *written* in English, not converted into it — warm, precise,
well-signposted, with the casual register intact. There is no chapter I would call "obviously
translated." The reproducibility/fidelity already-passed status is visible: nothing is muddy.

So the report below is a *polish pass*, not a rescue. There is exactly **one HIGH mistranslation**
(the "mouth" calque the brief flagged — confirmed, 4×) and **one recurring tic** that is the single
biggest read-feel liability across the book (the checker "getting angry," 9×). After those, it's a
short tail of MEDIUM/LOW idiom-smell. If you only fix two things, fix "mouth" and "get angry."

Chapters that read **clean as-is** (no actionable read-feel issue worth a rewrite): Little Part 2,
Little Part 3 (minus one "manner"), Seasoned Part 3, Seasoned Part 4, Seasoned Part 6, appendix a4.

---

## HIGH / PATTERN items (lead with the worst)

### [HIGH · PATTERN] The checker "gets angry" — calque of 怒る, 9× across the Little volume
**Files:** `little/part0` §0.3 · `little/part1` §1-2 · `little/part3` §3-2 · `little/part4` §4-1x ·
`little/part7` §7-1, §7-3a, §7-4, §7-6 (also the part4 summary note).

In well-edited English technical prose a *tool* does not "get angry." It reads either childish or,
to a native ear, translated — and it's the book's most frequent translation-smell by far. The
warmth the original is reaching for survives fine with a verb that fits a checker: **complain,
flag, object, balk, cry foul, report an error.** (Keep "never frighten working code" — that's the
established motto and it works; the problem is specifically *anger*.) Vary the replacement so it
doesn't become a new tic.

- `little/part7` §7-1 · before: "Answer `false` and you might **get angry at code that works fine.**"
  · after: "Answer `false` and you might **flag code that works fine.**"
- `little/part7` §7-4 · before: "As you can see, **we get angry only on `:no`.**"
  · after: "As you can see, **we complain only on `:no`.**"
- `little/part7` §7-4 · before: "code whose type is unknown but that nonetheless works **is never gotten angry at.**"
  · after: "code whose type is unknown but that nonetheless works **is never flagged.**"
- `little/part7` §7-3a · before: "We can only **get angry** when everyone is `:yes`."
  · after: "We can only **complain** when every member fails." *(also fixes a logic-readability slip:
  the sentence is about complaining when members fail, not when "everyone is `:yes`")*
- `little/part0` §0.3 · before: "once 'don't know' (`untyped`) is involved, **it never gets angry.**"
  · after: "once 'don't know' (`untyped`) is involved, **it never complains.**"
- `little/part1` §1-2 · before: "So **'getting angry because code has no type'** simply can't happen"
  · after: "So **'objecting just because code has no type'** simply can't happen"
- `little/part3` §3-2 · before: "quietly degrades to `Dynamic`. **It doesn't get angry.**"
  · after: "quietly degrades to `Dynamic`. **It raises no complaint.**"
- `little/part4` §4-1x · before: "So we **get angry only when all members fail**, and stay quiet about a partial failure"
  · after: "So we **complain only when all members fail**, and stay quiet about a partial failure"

**Why:** highest-frequency calque in the book; replacing it is the single biggest read-feel win.
**Severity:** HIGH (recurring; reads translated).

---

### [HIGH · PATTERN] "mouth" — calque of 口, 4× in Seasoned Part 1 (the brief's flagged item, confirmed)
**File:** `seasoned/part1-bidirectional-typing.md` — chapter intro (line 14), §1-7-a heading
(line 210), §1-7-a intro (line 216), §1-7-a body (line 250).

Japanese 口 ("opening / outlet / where something issues from") does not carry over. "the mouth
through which `⇐` emits a diagnostic" / "opens a mouth for diagnostics" / "the minimal mouth that
makes `⇐` work" all read as a mistranslation. The right English image is a **point / site / place**
where checking *emits* or *first raises* a diagnostic — or, keeping a touch of voice, where checking
**first speaks up**.

- intro (l.14) · before: "at the end we peek, in just one place, at **the mouth through which checking `⇐` emits a diagnostic.**"
  · after: "at the end we peek, in just one place, at **the first point where checking `⇐` emits a diagnostic.**"
- §1-7-a heading (l.210) · before: "A note — **the first mouth where `⇐` emits a diagnostic** (`check(rbs:)` mode)"
  · after: "A note — **the first place where `⇐` emits a diagnostic** (`check(rbs:)` mode)"
- §1-7-a intro (l.216) · before: "a small window for seeing where checking `⇐` **opens a mouth for diagnostics.**"
  · after: "a small window for seeing where checking `⇐` **first raises a diagnostic.**"
- §1-7-a body (l.250) · before: "This is **the minimal mouth that makes `⇐` work** outside 'arguments.'"
  · after: "This is **the minimal point at which `⇐` does work** outside 'arguments.'"

**Why:** literal of 口; reads as a mistranslation exactly as flagged. **Severity:** HIGH.

---

## MEDIUM items

### [MEDIUM] "turns its own self-check green" / "goes green" — fine as jargon, but the README phrasing is opaque on first contact
**File:** `seasoned/README.md` (line 16) · `seasoned/part2`…`part6` (recurring in design-sketch lines).

"where `ruby <file>` turns its own self-check green" assumes the reader already maps green = passing
self-test. In the per-chapter sketch lines it's established by then and fine; in the README it lands
cold. One clause fixes it.
- before: "the runnable sketches live in `examples/`, where `ruby <file>` **turns its own self-check green**"
- after: "the runnable sketches live in `examples/`, where `ruby <file>` **runs its own self-check and prints all-pass**"

**Why:** buried idiom at first appearance; trivial to de-fog without losing the later shorthand.
**Severity:** MEDIUM.

### [MEDIUM] "a running start on reading real Rigor's source" — slightly off collocation
**File:** `README.md` §"What you get from this book" (line 32).
- before: "and **a running start on reading real Rigor's source.**"
- after: "and **a head start on reading real Rigor's source.**"

**Why:** "a running start *on* X" is not idiomatic; "a head start on" is the native phrase here.
**Severity:** MEDIUM (low-stakes but it's in the opening pitch).

### [MEDIUM · PATTERN] "manner" used for 流儀/作法 — reads stiff/translated, 4–5×
**Files:** `little/part3` §3-1 · `little/part7` §7-5 + footnote · `seasoned/part1` §1-4 ·
`seasoned/examples/README.md`.

"manner" for "way / practice / convention / habit" is a faux-ami here; English "manner" leans toward
*demeanor*. Each instance reads a hair formal-and-translated.
- `little/part7` §7-5 · before: "it's **a manner Rigor deliberately keeps.**"
  · after: "it's **a discipline Rigor deliberately keeps.**" (or "a convention")
- `seasoned/part1` §1-4 · before: "In the Little volume it appeared as **a manner**; in the Seasoned volume, as theory."
  · after: "In the Little volume it appeared as **a practice**; in the Seasoned volume, as theory."
- `little/part3` §3-1 · before: "**The same manner in which** 『しくみ』 copied `{ ...tyEnv, x: type }`…"
  · after: "**Just as** 『しくみ』 copied `{ ...tyEnv, x: type }`…"

**Why:** small recurring calque; swapping to convention/practice/discipline reads native and is
exactly synonymous. **Severity:** MEDIUM (recurring).

### [MEDIUM] §4-1x dense run-on at the climax ("get angry" plus stacked dashes)
**File:** `little/part4` §4-1x (the "This behavior is continuous…" paragraph, ~lines 154–162).

Beyond the "get angry" fix above, this is the one paragraph where the sentence length and dash
pile-up genuinely tax the reader — three ideas (partial failure → `:maybe`, unknown member → whole
`untyped`, all-fail → one diagnostic) braided into two long sentences. Recommend breaking into three
short sentences, one idea each. Not a single before→after line; flagged as the one place to *split*.

**Why:** run-on + buried logic at a payoff moment. **Severity:** MEDIUM.

---

## LOW items (nitpicks)

### [LOW · PATTERN] "in a phrase" — repetitive tic, ~10×
**Files:** appendix a1 (3×), a4 (4×), `little/part6`, `seasoned/part7` (2×).
Idiomatic and individually fine, but it recurs enough to notice. Vary a few to "in short" / "put
simply" / "in a word." **Severity:** LOW.

### [LOW · PATTERN] "a spoonful" / "by a spoonful" — whimsical calque, 5×
**Files:** `little/part1` §1-4b, `little/part2` (note ×2 + §2-6), `little/part4` §4-1x.
"add *a spoonful*" works as warm voice in the dispatch notes and I'd keep it there. The one that
reads slightly off is `little/part1` §1-4b: "We extend `diagnostic` **by a spoonful**" — "extend X by
a spoonful" doesn't quite parse. Suggest: "We extend `diagnostic` **just a little**." **Severity:** LOW.

### [LOW] "settle in" (intransitive, = sink in / click) — mild calque of 腑に落ちる/馴染む
**Files:** `little/part0` §0.3 ("that settles in chapter by chapter"), `little/part9` §9-5
("settle in more easily"), `seasoned/part8` §8-2-a ("should settle in.").
Reads slightly soft. "that becomes clear chapter by chapter" / "sink in more easily" / "should land"
are crisper. **Severity:** LOW.

### [LOW] §3-1 "without fouling the original" — register dip
**File:** `little/part3` §3-1: "hand it on **without fouling the original.**"
"fouling" is a notch too colorful/colloquial for the surrounding precision. "without disturbing the
original" / "without corrupting the original." **Severity:** LOW.

### [LOW] Glossary line 31 "where each tool's syntax and history" wording is fine; no change. (Confirming the glossary reads clean.)

---

## Chapters confirmed clean (no rewrite warranted)
Little Part 2, Little Part 3 (bar one "manner"), Seasoned Part 2, Seasoned Part 3, Seasoned Part 4,
Seasoned Part 5, Seasoned Part 6, Seasoned Part 8, appendix a2, a4, a5. These read as native,
well-edited English technical prose.
