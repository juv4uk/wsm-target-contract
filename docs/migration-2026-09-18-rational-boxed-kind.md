# Migration note: exact Rational ratified as a `Boxed` kind (2026-09-18)

`wsm-target-contract` version bumped 4 -> 5. Tracks issue #11.

## Decision

An exact Rational value is a new discriminant inside the existing
`Tag::Boxed = 7` mechanism. It does **not** receive a new raw word tag.

This follows the same representation rule already used by String and the
game-engine handle: the 64-bit word carries only a non-zero, session-local
runtime handle. The concrete kind is stored in the runtime-owned table entry.

The current 3-bit tag space is already full. More importantly, Rational is a
runtime value kind, not a new tag-level semantic primitive, so allocating a
different tag would duplicate meaning even if space existed.

## Evidence

- CML #137 proves an upstream-owned exact-rational source reaches shared
  `Ir::Rational` and currently stops at native x86 representation.
- `my-lisp-cyberpunk/host-runtime` already has a live
  `BoxedValue::Rational(i64, i64)`, boxed table storage, wrap/unwrap and
  `N/D` printer while consuming the ratified Boxed wire shape.
- wsm-os-lisp #45 tracks the bounded freestanding runtime mechanism needed by
  generated x86 code.

## What changed

- contract version 4 -> 5;
- `boxed.kinds-defined-so-far` becomes
  `(string game-handle rational)`;
- the `Tag::Boxed` documentation names Rational as a ratified kind.

## What did not change

- `Tag::Boxed` remains numeric tag 7;
- `encode_boxed` / `decode_boxed` are unchanged;
- handle zero remains invalid;
- handle lifetime remains session-local and runtime-owned;
- this contract defines no Rational arithmetic, normalization result, parser
  syntax, or expected Lisp value.

## Consumer notes

- **wsm-os-lisp**: #45 should add the freestanding allocation/access mechanism
  under opaque `RuntimeContext`; do not expose the table layout to generated
  code.
- **CML**: after the runtime mechanism lands, pin a target-contract revision
  carrying v5 and lower `Ir::Rational` through that mechanism. Do not invent
  a local tag or semantic answer.
- **my-lisp**: no language change. It remains the semantic authority for exact
  arithmetic.

**Target contract owns representation; runtime owns bounded allocation;
my-lisp owns meaning.**
