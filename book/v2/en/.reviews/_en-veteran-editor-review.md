# English edition — veteran technical-book editor review (read-feel)

Reviewer lens: a US technical-book editor (1990s–2010s lineage) reading the English
transcreation (`book/v2/en/`) for read-feel — economy, signposting, promise→payoff, active
voice, antecedents, rhythm, and residual translation stiffness. The JA original
(`book/v2/ja/`) is the source of truth; the spaced em dash ` — ` is mandated house style and
is never flagged. Coverage: README, glossary, all Little parts (0–9), all Seasoned parts
(1–8), the Seasoned examples README, and appendices a1–a5.

## Headline

**This reads like a deliberately-voiced English technical book, not a translation.** The
register is consistent across both volumes, the three-perspective scaffolding lands, the
promise→payoff spine (the §1-3 / §7-4 foreshadow-and-recover bookend) is genuinely well
built, and the spaced-em-dash rhythm is used with control. **No ERROR-level defects of
grammar or broken logic were found in the reader-facing prose** — the one ERROR below is a
reflow/line-break artifact, not a sentence fault.

The findings are concentrated in two recurring, fixable patterns plus a handful of
sentence-level stiff spots:

1. **The 現場 calque** — "on the ground" / "on-the-ground" / "on-the-ground experience"
   appears in a1 (×2) and Seasoned Part 2, where the same source word (現場) is rendered
   cleanly as "in practice" / "working code" elsewhere. One global pass.
2. **The definite-article-on-a-code-span tell** — "the `x + 1`", recurring in Little Part 4/5,
   a leak of the JA topic marker. Drop the article.
3. **Em-dash *density*** (not the em dash itself) — a few sentences stack three or four breaks
   and collapse; the mandated style is fine, the count defeats it (Little Part 1:67).

---

## Cross-cutting (do these as global passes)

### IMPROVE — the 現場 calque: "on the ground" → "in practice"
The JA 現場 gets a clean rendering in most places ("never frighten working code") but leaks as
a calque in three spots. Source-confirmed at Seasoned Part 2 (現場の経験).

- **appendix/a1-special-types.md:60-62** — "take not-stopping the on-the-ground code" →
  "take soundness, or keep the working code running."
- **appendix/a1-special-types.md:172** — "more useful on the ground than dutifully
  propagating the empty set" → "more useful in practice than…".
- **seasoned/part2-subtyping-and-variance.md:206** — "It reached the same form from
  on-the-ground experience without passing through a mathematical proof" → "It reached the
  same form from real-world experience…" (or "from practice").

### IMPROVE — the definite-article-on-a-code-span tell ("the `x + 1`")
A JA topic-marker leak; English wants the code span bare. Recurs in Little Part 4 (≈163, 166)
and similar spots in Part 5. Global find-and-fix: "the `x + 1`" → "`x + 1`".

---

## README / glossary / volume READMEs

Clean. README.md, little/README.md, seasoned/README.md, glossary.md, and
seasoned/examples/README.md all read fluently — well-signposted, active, native asides. The
examples README's "Drift prevention" section and the a4 correspondence tables are
particularly well done. No findings.

---

## The Little chibirigor (parts 0–9)

Part 0 reads strongly: the two-promises framing, the data-flow figure, and the "the three
perspectives" setup all land. Parts 2 and 3 are the most fluent of the set. Findings:

### IMPROVE — little/part1-literals-and-arithmetic.md:67-68 — em-dash density collapse
> "When a type is unknown — when there's no way to check — we **fall back to `Dynamic`
> (`untyped`)** — the mark for "unknown / no way to check — keep quiet.""

Three em-dash breaks plus a fourth dash *inside* the quote in one sentence; "no way to check"
also repeats within ten words. The mandated style is fine — the *count* defeats it here.
Suggest: "…we **fall back to `Dynamic` (`untyped`)**, the mark for "unknown; keep quiet.""

### IMPROVE — little/part4-union.md:174-175 — buried spine ("The reason … is that …")
> "The reason `(1 | 2) + 1` shows up as `2 | 3` when you run `exe/chibirigor` yourself (the
> chapter's minimal version would say `untyped`) is that this distribution lives on the
> Dispatch side."

A 12-word parenthetical plus a relative clause split subject from verb; the reader holds the
whole thing in suspension. (The parallel payoff at part2:316 lands because it's shorter.)
Suggest: "Run `exe/chibirigor` yourself and `(1 | 2) + 1` shows up as `2 | 3` — the chapter's
minimal version would say `untyped` — because this distribution lives on the Dispatch side."

### IMPROVE — little/part3-scope-and-statements.md:53-54 — near-miss idiom
> "The reason runs slightly ahead — in Part 4–5 we want to hold *separate* notes per `if`
> branch…"

"The reason runs slightly ahead" is a literal rendering (理由は少し先にある) that reads as a
near-miss idiom; a reader parses it twice. Suggest: "The reason lies a little further on — in
Part 4–5…".

### IMPROVE — little/part7-accepts-and-trinary.md:243-244 — vague payoffs
> "Make acceptance strict and the caller is forced to write needless type conversions and
> gets cramped. Make the return lenient and whoever uses that value loses out."

"gets cramped" is an incomplete idiom (cramped how?) and "loses out" is vague about the harm.
The "Make X and Y" parallel is good rhythm; firm the payoffs: "…forced into needless type
conversions, hemmed in. Make the return lenient and everyone downstream inherits the
uncertainty."

### IMPROVE — little/part8-rbs-and-signatures.md:317-319 — over-compressed metaphor
> "The rest is homework re-treated in the Seasoned volume *with proper names* — at the close
> of the main volume, let's survey where it's headed."

"homework re-treated … with proper names" is opaque on first read, and "let's survey where
it's headed" promises a forward survey the summary table below doesn't quite deliver as one.
Suggest: "The rest is left as work the Seasoned volume takes up and gives proper names. The
handoff list below maps where each piece is headed."

### IMPROVE — little/part9-gradual-philosophy.md:140-142 — flat summary tag + tense slip
> "`untyped` was `Top` with a "be quiet" marker (`Dynamic`) laid on it; `never` was `Bot`
> itself; `void`, as *lattice behavior*, was next to `Top` — that was the relationship."

"— that was the relationship" is a flat tag (そういう関係です) the semicolon list has already
shown; the sentence lands harder without it. The three past-tense verbs ("was/was/was") also
slip out of the surrounding present tense for what are standing facts. Suggest present tense
and drop the tag: "…`void`, as *lattice behavior*, sits next to `Top`."

### NIT — little/part6-hash-and-tuple.md:143-144 — "passes more demands" reads backward
> "The side with more keys passes more demands, so it's the subtype."

"passes more demands" (要求を満たす) reads as "passes along," the opposite sense. Suggest
"*meets* more demands."

### NIT — little/part5-narrowing.md:181-183 — stacked prepositional phrases
> "another expression of the "don't frighten" value, on the same axis as the judgment, in the
> `possible?` guard above, to "not narrow when it's impossible.""

Three stacked phrases make the sentence's end wander. Suggest: "…— the same axis as the
`possible?` guard's judgment above, to "not narrow when it's impossible.""

### NIT — little/part2-method-dispatch.md:64-66 — heavy fronted clause
> "That most of "find the type of an expression" comes down, in the end, to knowing "**what
> does this method of this receiver return**" is exactly why."

A fronted noun-clause subject landing on a bare "is exactly why" forces a backward hunt.
Reorder subject-first: "That's exactly why most of "find the type of an expression" comes
down, in the end, to knowing…".

### NIT — little/part1-literals-and-arithmetic.md:171 — redundant apposition
> "Here the **third perspective — ③, where Rigor ran into trouble — surfaces naturally:**"

③ *is* the third perspective; the apposition is redundant and the nested dashes-in-bold crowd
the line. Fold the numeral into the noun: "Here the **third perspective ③ — where Rigor ran
into trouble — surfaces naturally:**".

### NIT — little/part9-gradual-philosophy.md:120-123 — buried verb
> "…it makes visible the silence that chibirigor's "stay quiet if you don't know," flipped
> over, turns into "quietly miss.""

The double-comma interruption buries "turns into" and the silence→stay-quiet→quietly-miss
chain is hard to track in one pass. Split or set off with dashes.

---

## The Seasoned chibirigor (parts 1–8)

The densest, highest-risk prose — and it holds up. Part 1 (the bidirectional map) and Part 7
(soundness) are especially well structured; the foreshadow-in-§1-3 / recover-in-§7-4 bookend
is real and earns its payoff. Parts 3 (substitution) and 4 (μ/coinduction) handle genuinely
hard material with clean signposting. Findings are sentence-level.

### IMPROVE — seasoned/part2-subtyping-and-variance.md:206 — "on-the-ground experience"
See the cross-cutting calque note above. → "from real-world experience" / "from practice."

### IMPROVE — seasoned/part1-bidirectional-typing.md:129-134 — long suspended close to §1-3
> "To sum up — working code goes unfrightened not because "there's no `⇐` position" but
> because **"synthesis collapses the unknown to `untyped`, and checking doesn't punish
> `untyped`."** It's a property statable only by laying chibirigor's two gradual design
> judgments (① totalizing synthesis, ② lenient checking) onto the bidirectional framework."

The second sentence ("It's a property statable only by laying…onto the bidirectional
framework") is the one genuinely effortful sentence in an otherwise crisp section — heavy
nominalization ("a property statable only by laying"). The point is strong; the phrasing makes
the reader work. Suggest: "You can state this property only by laying chibirigor's two gradual
design judgments — ① totalizing synthesis, ② lenient checking — onto the bidirectional
framework."

### IMPROVE — seasoned/part5-real-inference.md:300-302 — long FP-invariance sentence
> "Zero false positives stays invariant too: an empty array `[].map { ... }` has `untyped`
> elements so the body is `untyped` too, and an unknown receiver `foo.map { ... }` "doesn't
> presume it's an array," so it doesn't check the body and falls back to `untyped`."

A single sentence carries two distinct cases joined by "and…and"; the second "so" clause runs
long. Read cleaner as two: "Zero false positives stays invariant too. An empty array
`[].map { ... }` has `untyped` elements, so the body is `untyped`; and for an unknown receiver
`foo.map { ... }`, chibirigor doesn't presume it's an array, so it skips the body and falls
back to `untyped`."

### NIT — seasoned/part7-soundness.md:99 — parenthetical doubles the clause
> "…so preservation is trivially kept in the sense of consistency (`~`) (when types stop
> fitting, just fall back to `untyped`)."

The parenthetical "(when types stop fitting, just fall back to `untyped`)" restates the
clause that precedes it and abuts a second paren `(~)`; two parens collide. Fold into the
sentence: "…so preservation is trivially kept in the sense of consistency `~`: when types stop
fitting, it just falls back to `untyped`."

### NIT — seasoned/part4-recursive-types.md:172-173 — telegraphic theory/engineering split
> "To the same problem of "how to stop the unfolding of recursion," 『しくみ』/TAPL answer
> *theoretically* (correct equivalence test by coinduction), Rigor *in engineering* (safely
> cut off with fuel)."

The verb is elided in both halves ("answer … , Rigor …"); telegraphic. Suggest: "To the same
problem — how to stop the unfolding of recursion — 『しくみ』/TAPL answer *theoretically* (a
correct equivalence test by coinduction); Rigor answers *in engineering* (cut it off safely
with fuel)."

### NIT — seasoned/part8-toward-rigor.md:159-161 — fragment-y first doorway
> "**『しくみ』/TAPL's terminus is chibirigor's starting point.** *Beyond* a static, sound
> checker — from gradual typing, we began (not required, but reading them alongside reads
> continuously)."

"*Beyond* a static, sound checker — from gradual typing, we began" is fragmentary and the
final parenthetical is terse. The point (the two books' end is this book's beginning) is good;
the sentence stutters. Suggest: "We begin past a static, sound checker — at gradual typing.
(Neither book is required, but read alongside, they run continuously into this one.)"

---

## Appendices (a1–a5)

Strong overall. a4-bibliography.md reads clean. Findings:

### ERROR — appendix/a5-other-languages.md:54-56 — one-word orphan line (reflow artifact)
> "Here there's neither `extends` nor `implements` — **no inheritance relation at all.** It's
> / decided by *shape* alone:"

"It's" is stranded at line-end with "decided" wrapping two lines down — a reflow artifact that
breaks the paragraph. Rejoin: "…**no inheritance relation at all.** It's decided by *shape*
alone:".

### IMPROVE — appendix/a1-special-types.md:60-62 — "take not-stopping the on-the-ground code"
See cross-cutting calque. → "the attitude is exactly opposite — take soundness, or keep the
working code running."

### IMPROVE — appendix/a1-special-types.md:50-52 — terse-to-cryptic clause
> "Its behavior is close to `any`, but it isn't collapsed into one word — 'where it stayed
> quiet' remains as structure."

"it isn't collapsed into one word" is terse to the point of cryptic; make the subject
explicit. Suggest: "Its behavior is close to `any`, but the two axes aren't collapsed into a
single word — 'where it stayed quiet' survives as structure."

### IMPROVE — appendix/a1-special-types.md:111 — "tantamount to"
> "…honestly admits this reality as a type, tantamount to saying 'this method **could return
> any value.**'"

"tantamount to" is the one register spike in the section. → "…honestly admits this reality as
a type — in effect saying…".

### IMPROVE — appendix/a3-tooling.md:142-143 — runaway parenthetical
A paren opened at "(the same idea as PHPStan's `dumpType()`" runs across two sentences before
closing at "doesn't turn diagnostics red)", and the reader loses where it ends. Break it out
into running prose (see suggested rewrite in the a3 detail). 

### IMPROVE — appendix/a2-narrowing-patterns.md:62-63 — number agreement
> "Ruby's `=~` and named captures (`(?<name>...)`) have a behavior almost unique to it"

Plural subject, singular "it" with no clean antecedent. Suggest: "Ruby's `=~` with named
captures (`(?<name>...)`) has a behavior almost unique to Ruby — on a successful match it binds
`String` to local variables."

### NIT — appendix/a1-special-types.md:53 — heavy fronted nominalized subject
> "That Little Part 9's `rigor check --explain` can map every fail-soft site is because this
> `Dynamic` marker doesn't vanish."

Reorder: "Little Part 9's `rigor check --explain` can map every fail-soft site precisely
because this `Dynamic` marker doesn't vanish."

### NIT — appendix/a3-tooling.md:309 — operator-precedence ambiguity in trace text
> "► dispatch: 1 | -1.+(2) → 3 | 1 (distribute Union to members)"

`1 | -1.+(2)` parses, to a Ruby eye, as `1 | (-1.+(2))` because `.` binds tighter than `|`;
intent is `(1 | -1).+(2)`. If this is authored display text, parenthesize. **Verify against
the real `tracer.rb` output before changing** — if the tool emits it this way, leave it.

### NIT — appendix/a5-other-languages.md:34 — slightly wordy
> "the same as the habit, in Java, of writing `if (x != null) { … }` and then touching a
> field" → "the same as the Java habit of writing `if (x != null) { … }` before touching a
> field".

---

## What reads clean (no findings)

README.md · glossary.md · little/README.md · seasoned/README.md · seasoned/examples/README.md
· appendix/a4-bibliography.md · little/part0 (strong) · little/part2–3 (most fluent) ·
seasoned/part1, part3, part4, part6, part7 (dense material, clean signposting).
