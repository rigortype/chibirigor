# The Seasoned chibirigor — runnable design sketches

The Seasoned volume's core algorithms, made into **minimal Ruby that runs standalone.** In the same
way the Little volume "verified its code by actually running it," the Seasoned volume's claims are
backed up in a form that *runs and goes green.* Zero dependencies, Ruby 3.4+ (no test framework,
either). Each file has a self-check at the end; if `PASS` lines up with `ruby <file>`, it succeeded.

| File | Chapter | What it confirms |
|---|---|---|
| [`subtype.rb`](subtype.rb) | Part 2 Subtyping and variance | width/depth subtyping, **contravariant arguments / covariant returns** (the reverse comes out false) |
| [`mu_typeeq.rb`](mu_typeeq.rb) | Part 4 Recursive types | equivalence of a μ-type's fold/unfold, **α-equivalence**, **termination by coinduction (`seen`)** |
| [`subst.rb`](subst.rb) | Part 3 Generics and type substitution | stop on **shadowing**, avoid **variable capture** with fresh variables |
| [`unification.rb`](unification.rb) | Part 5 Real type inference | gather constraints and solve by **unification** (a generic stays generic; a conflict raises) |
| [`fact_invalidation.rb`](fact_invalidation.rb) | Part 6 The complete FactStore | an immutable fact store, **bucket-specified invalidation** (reassignment / method call) |

```console
$ ruby subtype.rb
$ ruby mu_typeeq.rb
$ ruby subst.rb
$ ruby unification.rb
$ ruby fact_invalidation.rb
```

> These are *educational design sketches*, not real Rigor's code (both the type representation and the
> judgment are minimal). Even so, you can confirm that the mechanisms the Seasoned volume explained in
> words — contravariance, coinduction, capture-avoidance — actually run on your machine and return the
> right answer.

## Drift prevention — mechanically sync the prose code and the examples

Code pasted into the prose (chapter `.md`) always drifts from the implementation when copied by hand
(in fact, a formatter rewriting just `_1` to `it` is enough to drift). **`check_docs.rb`** prevents
this mechanically.

The mechanism is just three things:

1. **Are the examples all green** — `ruby <file>` exits 0 (the self-check PASSes).
2. **Verbatim sync of code** — putting `<!-- include: file.rb#region -->` on a prose ```code block
   checks that it's a **byte match** with that file's "`# region <id>` … `# endregion`" span.
3. **Verbatim sync of output** — putting `<!-- run: file.rb -->` on a prose ```text block checks that
   each line is **contained as-is in that file's actual output** (a subset is OK).

Usage (zero dependencies, stdlib only):

```console
$ ruby check_docs.rb        # check (exit 1 if there's drift)
$ ruby check_docs.rb --fix  # regenerate include blocks from the regions and sync
```

**Operation:** when you newly quote code in the prose, place `# region <id> … # endregion` on the
example side and put `<!-- include: file#id -->` on the prose side (`--fix` pours the contents in from
the region). Even if a formatter rewrites the examples, running `check_docs.rb --fix` once makes the
prose follow. Put `ruby check_docs.rb` in CI and you can stop drift from being pushed.

> **English edition note.** The shared `.rb` sketches are **English-canonical** (English comments), so
> the printed excerpts in the English chapters match the sketches directly. The English chapters
> currently **omit** the `<!-- include: -->` / `<!-- run: -->` markers — `check_docs.rb` is wired
> against the Japanese prose, and the English example output is verified by reading. Wiring
> `check_docs.rb` against `book/v2/en/` is an optional follow-up; see the contributor guide
> [STYLE.md](../../STYLE.md) and the handoff ledger.

> **Extension to the Little volume / implementation.** The same mechanism applies as-is to the Little
> volume (prose `little/*.md` ↔ `lib/chibirigor/*.rb`) — place regions on the `lib` side, put `include`
> on the prose, and add the target md directory. Applying it on the Little side is separate (see the
> handoff notes).
