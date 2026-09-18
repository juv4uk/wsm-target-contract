# Migration note: Rational runtime imports ratified (2026-09-18)

Target contract version 6 adds the minimal freestanding mechanism by which
generated code may ask the runtime to materialize and inspect the already
ratified Boxed Rational kind.

## Added runtime imports

- `wsm_rational_new(RuntimeContext*, i64 numerator, i64 denominator) -> Value`
- `wsm_rational_numerator(RuntimeContext*, Value) -> i64`
- `wsm_rational_denominator(RuntimeContext*, Value) -> i64`

The runtime fixes the boxed discriminant to Rational. No generic
`boxed_alloc(kind,...)` is admitted, so generated WSM code cannot select or
forge arbitrary internal Boxed kinds.

## Representation unchanged

- `Tag::Boxed` remains 7.
- `encode_boxed` / `decode_boxed` are unchanged.
- handle zero remains invalid.
- the table remains session-local and runtime-owned.
- this contract defines no Rational arithmetic, reduction law, parser syntax,
  or expected Lisp value.

## Projection source-of-truth repair

The generated `runtime-imports` row now derives from the crate's
`RUNTIME_IMPORTS` array rather than a second hard-coded list. This also
restores the already-ratified `wsm_mmio_*` imports that the old projection
accidentally omitted.

Consumers:
- wsm-os-lisp#45 implements the bounded pure-ASM runtime mechanism.
- cml#137 consumes it after the runtime lands.

**Target contract owns representation and import names; runtime owns bounded
allocation and validation; my-lisp owns Rational meaning.**
