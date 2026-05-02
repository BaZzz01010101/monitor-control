use dell_controller_core::snapshot::{Snapshot, SnapshotDiff};

#[test]
fn diffs_changed_added_and_removed_vcp_values() {
    let before = Snapshot::from_pairs("Dell U4025QW", [(0x10, 75), (0x12, 80), (0x60, 0x0F)]);
    let after = Snapshot::from_pairs("Dell U4025QW", [(0x10, 70), (0x60, 0x0F), (0xE2, 0x11)]);

    let diff = SnapshotDiff::between(&before, &after);

    assert_eq!(diff.changed_value(0x10), Some((75, 70)));
    assert_eq!(diff.removed_value(0x12), Some(80));
    assert_eq!(diff.added_value(0xE2), Some(0x11));
    assert!(!diff.has_change(0x60));
}
