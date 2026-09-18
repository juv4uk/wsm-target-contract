use wsm_os_target::{RUNTIME_IMPORTS, Tag, decode_boxed, encode_boxed};

#[test]
fn rational_runtime_imports_are_ratified_without_changing_boxed_wire_shape() {
    for symbol in [
        "wsm_rational_new",
        "wsm_rational_numerator",
        "wsm_rational_denominator",
    ] {
        assert!(
            RUNTIME_IMPORTS.contains(&symbol),
            "#12 requires target-contract to ratify runtime import {symbol}"
        );
    }

    assert_eq!(
        Tag::Boxed as u8,
        7,
        "#12 must not allocate a new Rational tag"
    );
    let word = encode_boxed(1).expect("existing Boxed handle 1 must remain valid");
    assert_eq!(
        decode_boxed(word),
        Some(1),
        "#12 must preserve Boxed wire encoding"
    );
}
