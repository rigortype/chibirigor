# impls ― end-of-chapter snapshots of where the code has reached

Following chibivue's [`book/impls`](https://github.com/chibivue-land/chibivue/tree/main/book/impls),
we lay out, as a fully working tree, **how far the code in your hands has come** at the point you
finish reading each Part of the prose. But whereas chibivue maintains every full copy by hand, here
we **treat "only the files that changed" as the source of truth and machine-generate the complete
tree** (to reduce accidents, i.e. drift).

## How it works (single source → generated artifacts → verification gates)

```
impls/
├── steps/            ← source of truth (edited by hand)
│   ├── part1/
│   │   ├── lib/...           only the files *newly added/replaced* in that chapter
│   │   └── test_stage.rb     a smoke test that pins down the behavior reached at that stage
│   └── part2/
│       ├── lib/chibirigor/dispatch.rb   (new)
│       ├── lib/chibirigor/type.rb       (replace: into the Type:: namespace)
│       ├── lib/chibirigor/type_of.rb    (replace: delegate to Dispatch)
│       ├── lib/chibirigor.rb            (replace: add a require)
│       └── test_stage.rb
└── dist/             ← generated artifacts (not edited by hand; committed for browsing)
    ├── part1/lib/... complete tree (state after finishing part1)
    └── part2/lib/... complete tree (part1 + part2 diffs overlaid)
```

`tools/gen_impls.rb` **overlays in order** the `lib/` directories of `steps/part1..partN` (the same
path in a later stage overwrites, i.e. replaces, the earlier one) and emits the **complete tree** for
each stage reached into `dist/partN/`.

```console
$ make impls          # generate dist
$ make impls-verify   # generate + run each stage's test_stage.rb (is each stage's behavior green?)
$ make impls-check    # regenerate and verify dist hasn't been hand-edited (in sync with steps)
```

## Why "per-file forward compose" rather than "a chain of patches"

- **A chain of patches** (cumulatively applying `0N.patch`) is position-dependent: a change in an
  early stage makes later patches cascade into conflicts. This book reorders chapters (we have a
  track record of doing so), so that approach is a poor fit.
- **Per-file forward compose** means that even when chapters are swapped, you **only renumber the
  directory names**. Changes are localized to "the files that changed at that stage" and don't
  conflict.
- **A reverse projection that strips the final `lib/` from the back** is hard, because the final lib
  already weaves in all Parts (`checker.rb` already has baseline/rbs, `annotator.rb` already has sig
  synthesis). With forward compose, *the teaching code itself* at each stage is the source.

## Three points that reduce accidents

1. **Single source of truth**: each stage's changed files live in exactly one place under `steps/`
   (no maintaining N copies by hand).
2. **Snapshots are generated artifacts**: `dist/` is machine-generated. Don't touch it by hand
   (`make impls-check` detects it).
3. **A per-stage verification gate**: `test_stage.rb` pins down "the behavior reached at Part N"
   (e.g. Part 1 **rounds addition to Integer**, and Part 2 also yields Integer via dispatch). It's
   already wired into `make all`.

## Connecting to the prose (the next step)

If we make the prose's code blocks **pull directly from the snapshots** via
`<!-- include: ../../impls/dist/partN/lib/... #region -->`, then the prose ↔ snapshot ↔ stage tests
are bound to a single source, and drift disappears in principle (extending the same idea as the
existing `check_docs.rb` to the stages).

## Current status

- **Only the Part 1 / Part 2 prototype** (a proof of concept). For Part 3 onward, add "the files
  that changed in that chapter plus a `test_stage.rb`" to `steps/partN/`, and `make impls` will
  generate the complete tree automatically. The goal is to make the final stage match `lib/` (i.e.
  the finished form).
