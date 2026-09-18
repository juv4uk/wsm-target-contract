use wsm_os_target::{Tag, decode_boxed, encode_boxed};

const CONTRACT: &str = include_str!("../../target-contract.lisp");

#[test]
fn rational_is_ratified_as_an_existing_boxed_kind_without_new_wire_tag() {
    assert!(
        CONTRACT.contains("(kinds-defined-so-far . (string game-handle rational))"),
        "#11 requires rational to be a ratified discriminant inside the existing Boxed runtime object"
    );

    assert_eq!(
        Tag::Boxed as u8,
        7,
        "#11 must reuse the existing final 3-bit tag rather than allocate a new Rational tag"
    );

    let word = encode_boxed(1).expect("existing Boxed wire shape must still admit handle 1");
    assert_eq!(
        decode_boxed(word),
        Some(1),
        "#11 must not change Boxed handle encoding"
    );
}
