# Migration note: `Tag::Boxed = 7` ratified (2026-09-10)

`wsm-target-contract` version bumped 2 -> 3. `Tag::Boxed = 7` is now
canonical — the last free value in the current 3-bit tag space (0-7 all
assigned; no room remains without a breaking `TAG_BITS` change).

## What changed

- `Tag::Boxed = 7` added to `wsm-os-target::Tag`.
- `encode_boxed(handle)` / `decode_boxed(word)` added, same shape as
  `encode_symbol`/`encode_capability`: non-zero handle in the payload bits,
  reject `handle == 0` or `handle > BOXED_HANDLE_MAX`.
- **Scope is session-local**, not image-local: a `Boxed` handle indexes a
  runtime-owned table created fresh per host session (e.g. an append-only,
  non-interning string arena). It does not survive across sessions or
  processes, unlike `Symbol`/`Closure` which are image-local.
- The concrete boxed kind (String today; Vector/NumericBuffer reserved for
  later) is a discriminant carried by the table entry the handle points to,
  **not** encoded in the tagged word itself — this is why one tag value can
  serve multiple future kinds without another ABI change.
- `target-contract.wsm` regenerated from `render_contract()`; a
  `.gitattributes` entry now pins it to LF so `committed_wsm_projection_is_current`
  does not fail on a Windows checkout with `core.autocrlf=true` (unrelated to
  this ABI change, but were blocking every run here).

## For consumers

- **`wsm-my-lisp`**: your `BoxedTable`/`BoxedValue` design (image-local ->
  should be session-local per the handle above) already matches this shape;
  swap your local `TAG_BOXED=7` from tentative/local-constant to importing
  `wsm_os_target::Tag::Boxed` once you pin a `wsm-target-contract` revision.
- **`cml`**: no `Ir::String`/boxed lowering exists yet in either backend — no
  migration needed until one is added; when it is, encode through
  `encode_boxed`, not a hand-rolled tag=7 constant.
- **`my-lisp`**: no change to language semantics. This is a machine-ABI
  ratification only, per this repo's scope; String remains its own `Value`
  variant regardless of how it is projected onto a tagged word downstream.
- **`fpga-lisp`**: no boxed-value support implied by this change; flagged
  here only so the tag space (now fully assigned) is visible before any
  future FPGA-side tag work.
