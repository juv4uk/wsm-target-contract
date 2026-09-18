use wsm_os_target::RUNTIME_IMPORTS;

const TABLE: &str = include_str!("../../operation-table.lisp");

#[test]
fn neutral_core_five_operation_table_is_complete_and_target_backed() {
    let rows = [
        ("0002", "wsm_atom"),
        ("0003", "wsm_eq"),
        ("0004", "wsm_cons"),
        ("0005", "wsm_car"),
        ("0006", "wsm_cdr"),
    ];

    for (semantic_id, import) in rows {
        assert!(
            TABLE.contains(&format!("(canonical-id . \"{semantic_id}\")")),
            "operation table omitted canonical identity {semantic_id}"
        );
        assert!(
            TABLE.contains(&format!("(target-operation . {import})")),
            "operation table omitted target operation {import}"
        );
        assert!(
            RUNTIME_IMPORTS.contains(&import),
            "operation table mapped {semantic_id} to non-ratified target import {import}"
        );
    }

    assert!(
        TABLE.contains("(unmapped-target-import-policy . fail-closed-no-semantic-id-inference)"),
        "operation table must forbid inferred semantic identities"
    );
}

#[test]
fn target_mechanism_imports_without_ratified_semantic_identity_stay_unmapped() {
    for mechanism in [
        "wsm_closure_new",
        "wsm_pci_config_capability",
        "wsm_mmio_read32",
        "wsm_rational_new",
        "wsm_fail",
    ] {
        assert!(
            !TABLE.contains(&format!("(target-operation . {mechanism})")),
            "target-only mechanism must not gain an invented semantic identity: {mechanism}"
        );
    }
}
