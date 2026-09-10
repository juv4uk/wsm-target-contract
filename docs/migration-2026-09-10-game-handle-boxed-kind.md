# Migration note: game-engine handle ratified as a `Boxed` kind, not `Capability` (2026-09-10)

`wsm-target-contract` version bumped 3 -> 4. Closes issue #2.

## Decision

An opaque game-engine object reference (e.g. a RED4ext RTTI handle obtained
via `ExecuteGlobalFunction`) is a new discriminant *inside* the existing
`Tag::Boxed = 7` mechanism — the same tag already carrying `String` — not a
new `CapabilityKind` under `Tag::Capability = 6`.

## Why not `Tag::Capability`

Two independent, non-stylistic reasons, both structural:

1. `CapabilityDescriptor.instance` is a hard `u8` (max 255). A play session
   can trivially hold more than 255 live RTTI references (entities, items,
   NPCs); `Capability`'s encoding cannot represent that.
2. `CapabilityDescriptor`'s nonce is documented as an *unforgeability*
   guarantee for boot-provisioned hardware resources (PciConfig/Mmio/Dma/
   Interrupt). A handle obtained from a plain host function call carries no
   such guarantee from the game engine — reusing `Capability` here would
   imply a security property that does not actually hold.

`Boxed` makes no such promise, and its existing session-local, non-image-local
scope is already the right lifetime match: a game handle does not outlive
the host session, exactly like the existing `String` boxed kind.

## What changed

- `wsm-os-target::Tag::Boxed`'s doc comment records the ratified rationale.
- `target-contract.wsm`'s `boxed.kinds-defined-so-far` is now
  `(string game-handle)`, still generic — no new encode/decode function was
  added here, because `Boxed`'s wire shape (non-zero session-local handle,
  discriminant inside the table entry) already covers this case exactly as
  it covers `String`. The discriminant enum itself (`BoxedValue` or
  equivalent) is a consumer's own data structure, not part of this crate.

## For consumers

- **`wsm-my-lisp`**: add a new `BoxedValue` variant (e.g. `GameHandle`) to
  your existing `BoxedTable`, the same non-interning append-only shape you
  already use for `Str`. No `wsm-os-target` API change is required on your
  side beyond consuming this doc.
- **`cml`**: no change to `platform_call_contract`'s structure. When a
  concrete `wsm_*` import for a game-handle-producing call is ratified
  separately, add it as an allowlist entry the same way as any other
  `wsm_*` symbol — this note only settles the *value representation*, not
  a specific new host function.
- **`my-lisp`**: no language semantics change. `docs/cyberpunk-opaque-
  capability-semantics.md`'s model (opaque, identity-only equality, no
  type-level permission distinction) already covers this regardless of the
  machine tag, since the language never observes the tag directly.
