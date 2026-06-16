# EN review — modern technical-book editor lens (2016–present)

**Reviewer seat:** modern (screen-first) technical-book editor. Optics: (a) scannability —
headings, lead sentences, tables, the ①②③ device; (b) voice temperature — warm,
casual-but-precise, flag stiff *or* cutesy; (c) translation-smell a 2020s dev bounces on —
clunky connectives, buried verbs, calqued discourse markers, disorienting paragraph openers;
(d) does the Little/Seasoned + three-voices framing still land in 2020s English.

Coverage: full Little volume (README, part0–part9) read closely; Seasoned README, Part 1
(opener), Part 5 (mid), Part 8 (finale) sampled; shared README, STYLE.md, glossary skim;
prior `_en-fp-researcher-review.md` and `_fidelity-review.md` cross-checked to avoid
duplication.

---

## SHORT VERDICT

**This is a strong transcreation that mostly reads as English-first, not translated.** The
register is exactly where it wants to be — warm, plain, second-person-plural, reference-book
calm without going breezy or twee. Scannability is genuinely good: every chapter front-loads a
bolded goal line, the ①②③ recap and summary tables let you skim a chapter in 90 seconds and
still get it, and the "Handed to the sequel" / "Next chapter" rails give real forward
momentum. The Little/Seasoned framing and the three voices **land** — see the framing judgment
at the bottom. The issues below are a handful of **recurring calques** (one childish, a few
merely odd) and a small number of **over-packed sentences** that bury the verb under em-dash
pile-ups. None are structural; all are sentence-level. Fix the three PATTERN items and the
book reads noticeably cleaner with near-zero risk to the warm register.

---

## HIGH

### H1 · PATTERN — "gets/got/gotten angry" calque of 怒る · voice temperature
**Files/sections:** part0 §0.3 (`never gets angry`), part1 §1-2 (`getting angry`), part3 §3-2
(`It doesn't get angry`), part4 §4-1x (`getting angry only when all members fail`), part5
(implicit), part7 §7-1/§7-3a/§7-4/§7-6 (`get angry`, `gotten angry at`, `we can only get angry
when…`, `never gotten angry at`). ~9 occurrences across the Little volume.
**Issue:** This is a direct calque of the Japanese 怒る ("the checker gets angry"). In English,
a *tool* "getting angry" reads either childish/cutesy or like raw translation-smell — and it
sits awkwardly *next to* the book's actual sanctioned motto, "never frighten working code,"
which is warm-but-adult and does the same job. The book already owns a better verb. Worst
offenders are the load-bearing thesis sentences, e.g. part7 §7-4: "code whose type is unknown
but that nonetheless works is **never gotten angry at**" — the single most important sentence
in the book, currently phrased as a clumsy passive calque.
**Fix:** Replace with the verbs the book *already* uses elsewhere for the same act: **flag /
complain / blame / report**. "the checker never *flags* it," "it doesn't *complain*," "we only
*flag* when all members fail," and for the thesis sentence: "code whose type is unknown but
that nonetheless works is **never flagged**." (`complain`/`complaint` is already in the book —
part1 §1-3 "gather the complaints," part7 — so this is using the book's own register, not
importing a new one.) Leave "never frighten working code" untouched; that's the keeper.
**Severity:** HIGH (recurring, hits thesis sentences, only clearly *off-register* tic in the book)

### H2 — Buried-verb / em-dash pile-up in load-bearing sentences · translation-smell
**Files/sections:** part0 §0.1 (the `TypeProf` blockquote: "so we **narrow to local inference
and buy scale and 'silence' (no false positives)** instead. This one line we won't cross — not
inferring arguments backward — the Little volume returns to concretely just once, in Part 8…");
part2 §2-1 ("That most of 'find the type of an expression' comes down, in the end, to knowing
'**what does this method of this receiver return**' is exactly why."); part4 §4-1x final
paragraph; part7 §7-2 (the `widen` lead-in sentence runs 40+ words before its main verb).
**Issue:** These are the sentences where the *subject is buried* and the main verb arrives late
behind two or three em-dash asides — the classic translation-smell of a dense JA sentence
carried over clause-order intact. A skimming reader's eye loses the spine. part2's "…is exactly
why" is a fronted-cleft that inverts English word order (the JA そういうわけ pattern); it
reads backwards.
**Fix:** Split each into two sentences and lead with subject+verb. part2 example → "So 'find
the type of an expression' mostly comes down to one question: **what does this method, on this
receiver, return?** That's why we generalize Part 1's `+`-only code here." part0 TypeProf →
break after "buy scale and silence." then "That one line — not inferring arguments backward —
we won't cross. The Little volume returns to it once, in Part 8…" Same move each time: one idea,
one sentence, verb early.
**Severity:** HIGH (recurs in the highest-bounce first-20-min chapters; pure clarity, zero
register cost)

---

## MEDIUM

### M1 · PATTERN — "topple to `untyped`" calque of 倒す/落とす · translation-smell
**Files/sections:** part4 §4-1x ("topples the whole Union to `untyped`"), part8 §8-6 (implied),
Seasoned P1 §1-1/§1-3 ("topple to `untyped`", "synthesis topples the unknown to `untyped`"),
Seasoned P5 (×4, "the arguments we toppled to untyped"), Seasoned P7 (×3), Seasoned P8. 15+
occurrences across both volumes.
**Issue:** "topple" for "fall back to / collapse to / degrade to" is a calque of the JA verb.
"Topple" in English means a *structure falling over* (a tower topples), not a value gracefully
defaulting — so each use lands a hair wrong, and at 15+ repetitions it becomes a noticeable
house-verb tic rather than invisible prose. The book *also* already uses the right verbs —
"fall back to `untyped`" (part0 §0.1), "collapse to `untyped`" (STYLE), "degrades to `Dynamic`"
(part2 §2-5) — so the vocabulary is inconsistent, not just calqued.
**Fix:** Standardize on the verbs already in the book: **fall back to / collapse to / degrade
to `untyped`**. Pick one as primary (STYLE's "fall back" reads most natural for the gentle
register) and sweep "topple" out. This is the book's *own* terminology drift, so it's a safe,
register-preserving find-and-replace.
**Severity:** MEDIUM (recurring, but borderline-readable as a house verb today; the bigger win
is consistency with the verbs the book already uses)

### M2 · PATTERN — "in your hands" / "in your hands with `exe/chibirigor`" calque of 手元で
**Files/sections:** part1 §1-4 ("the reason `1 + 2` shows up as `3` in your hands with
`exe/chibirigor`"), part2 §2-7 (same), part4 §4-1x (same). 3 occurrences, all in the
identical "why the real tool differs from the chapter's minimal version" sentence.
**Issue:** "in your hands" is a calque of 手元で ("locally / on your own machine"). It's not
*wrong*, but it's an odd, slightly archaic English idiom, and because all three uses sit in the
same boilerplate sentence-frame it reads as a template that wasn't fully naturalized.
**Fix:** "when you run it yourself" / "when you run `exe/chibirigor`" / "on your own machine."
e.g. "the reason `exe/chibirigor` shows `3` for `1 + 2` **when you run it** is that the folding
lives on the Dispatch side."
**Severity:** MEDIUM (low count, but it's a verbatim repeated frame, so one fix template covers
all three)

### M3 — part0 is the densest doorway in the book; the two front blockquotes risk a bounce · scannability
**File/section:** part0 §0 (the "Where Rigor came from" blockquote) + §0.1 (the TypeProf
blockquote, the reference-books blockquote).
**Issue:** part0 is doing a lot — and three long blockquotes stack up before the reader has
written a line of code. The "Where Rigor came from" origin story (a decade of PHP static
analysis, RubyKaigi 2026) is *charming* and earns its place, but the back-to-back blockquotes in
§0.1 (TypeProf, then reference-books, then "This Part 0's topics correspond to TAPL ch.1…") form
a wall of optional-aside grey-box right at the moment a new reader is deciding whether to commit.
The main thread ("infer first, then check") gets buried between two digressions.
**Fix:** Don't cut the content — it's good. But let the *main line* breathe: move the
reference-books blockquote and the TAPL-correspondence line to the *end* of §0.1 (after "Fix
that order in your head"), so the section runs thesis → example → thesis-restated, *then*
optional asides. The reader hits the payoff before the sidebars. (TypeProf blockquote can stay;
it directly motivates the "we don't infer backward" line.)
**Severity:** MEDIUM (first-20-min bounce risk; pure reordering, no rewriting)

### M4 — Connective-opener density ("So" ×20, "But" ×11) clusters in a few sections · translation-smell
**Files/sections:** Little volume overall (37 sentence-initial So/But/And). Densest in part1
§1-2/§1-3, part6 §6-2/§6-3, part7 §7-2.
**Issue:** Warm prose *earns* some sentence-initial "So" and "But" — and most uses here are
fine, even good for the conversational temperature. The problem is local clustering: part6 §6-2
"The point is that `h[:zzz]` **doesn't error.** The reason is simple — **real Ruby returns
`nil`…**" then part6 §6-3 opens consecutive paragraphs/sentences with "So… So… So." When three
"So"s land in one screenful, the connective stops carrying logical weight and reads as a verbal
tic (a common JA→EN artifact, だから/それで repeated).
**Fix:** Not a blanket purge — keep the register. Just de-cluster: where two "So" or "But"
sentences are within ~3 lines, recast one with the actual logical relation ("That's why…",
"Which means…", "The catch:", or just drop it and let juxtaposition do the work). Target the
clusters, not the isolated uses.
**Severity:** MEDIUM (PATTERN-adjacent but mostly fine; only the clusters bounce)

---

## LOW

### L1 — "the X of Y" inverted-noun-phrase calque, scattered · translation-smell
**Files/sections:** part2 §2-1 "the meaning changes with the receiver"; part5 §5-3 "terrain of
its own that 『しくみ』 doesn't cover"; part7 §7-4 "Here we set down the single most important
sentence in this book"; various "this chapter's climax / this chapter's summary" (fine as
headings, slightly stiff in body prose).
**Issue:** Occasional inverted/possessive noun phrases ("the X of Y," "Y's X") that mirror JA の
order and read a touch formal-stiff against the otherwise-warm voice. Individually invisible;
flagged only as a low-grade texture note, not an action item per instance.
**Fix:** No sweep needed. When revising a paragraph for another reason, prefer "the receiver
changes the meaning" over "the meaning changes with the receiver," etc. Opportunistic only.
**Severity:** LOW

### L2 — "the climax of Part N" / "this is the climax" repeated framing · voice temperature
**Files/sections:** part6 §6-3 ("This is the climax of Part 6"), part7 §7-4 ("the chapter's
climax"), Seasoned P2 (referenced), part1-bidirectional ("the climax of 『しくみ』 ch. 7").
**Issue:** "climax" as a section label recurs enough to lose punch and read a little
breathless/promotional — the modern-editor "broken promise" smell if a labeled "climax" then
reads as a routine section. Most instances *do* deliver, so this is mild.
**Fix:** Vary it — "the heart of Part 6," "where it all pays off," "the crux" — or just let the
content be the climax without announcing it. Keep one or two; thin the rest.
**Severity:** LOW

### L3 — Cross-reference: "mouth" metaphor (already filed by FP-researcher) · clarity
**File/section:** Seasoned P1 §1-7-a (intro blockquote, heading, body: "the first **mouth**
where `⇐` emits a diagnostic," "opens a mouth for diagnostics," "the minimal **mouth** that
makes `⇐` work"). ~4× in one section.
**Issue:** Confirming `_en-fp-researcher-review.md` clarity-2 from the modern-editor seat: 口
("point of emission") does not carry into English; "opens a mouth for diagnostics" reads as a
mistranslation, not a figure, and recurs enough in one section to be load-bearing. **Concur
fully** — render as "site" / "point" / "where `⇐` *first produces* a diagnostic."
**Severity:** LOW (already filed; logged here only so the editor pass picks it up in the same
sweep — it's a register/clarity issue, not just a soundness one)

---

## What genuinely works (don't touch)

- **The goal-line + ①②③-recap + summary-table scaffold is excellent for screen-skimming.** A
  reader can read the bold goal line, the three-voice recap, and the summary table and walk away
  with the chapter. This is exactly the modern-reader affordance the book needs, and it's
  consistent across all 10 Little chapters. Keep it rigid.
- **"never frighten working code"** as the spine motto is warm, memorable, and adult — it does
  the anthropomorphizing the "angry" calque tries to do, but lands. The repetition is a feature
  (the thesis), not a tic. Keep verbatim.
- **The "Handed to the sequel" rails** at each chapter end manage scope-anxiety beautifully —
  they tell the reader *what's deliberately left out and where it's picked up*, which is exactly
  the reassurance a "this is the small version" book needs. This is some of the best structural
  writing in the book.
- **part0's "Where Rigor came from" origin blockquote** is genuinely good narrative color (the
  decade of PHP static analysis, RubyKaigi 2026 as the turn) — it earns reader investment. Keep
  it; just don't let the *following* asides crowd the main line (see M3).
- **Seasoned register holds.** The shift from "build it, jargon deferred" to "read it, notation
  head-on" is executed cleanly — Seasoned P1 introduces `⊢`/`⇒`/`⇐` and the Sub rule without
  ever going cold, and the "what we wrote / what it really is" tables make the formalization feel
  like a *reveal* rather than a lecture. The temperature stays warm even as the math arrives.

---

## FRAMING JUDGMENT — does Little/Seasoned + three-voices land in 2020s English?

**Yes, clearly — and it reads as a deliberate homage, not a dated affectation.** The
Little/Seasoned split is doing real structural work (build-it vs. read-it, code-leads vs.
concepts-lead), and the book *names that work* in the README and every chapter rail, so a 2020s
dev who's never touched *The Little Schemer* still reads it as a sensible two-track structure
rather than an in-joke they're missing — the homage is a bonus for those who get it, not a
prerequisite. It does not read twee: there's no Carroll-style dialogue cosplay, no forced
whimsy; "Little" and "Seasoned" function as plain difficulty labels that happen to carry a wink.
The three voices (①②③) are the stronger device of the two — they're the book's single best
scannability and pedagogy affordance, giving every chapter a fixed "concept ↔ Ruby reality ↔
why the naive impl breaks" skeleton that a modern skim-reader can lock onto immediately, and
the "trouble in ③ arises *necessarily* from ②, settles *gently* under ①" thesis genuinely
pays off chapter after chapter. The only thing standing between the device and a fully native
read is the handful of calques above (esp. the "angry" tic, which undercuts the same warm
register the three-voices framing depends on). Fix those and the framing lands without an
asterisk.
