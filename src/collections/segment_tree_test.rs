use crate::collections::segment_tree::SegmentTree;

#[test]
fn seg_tree_range_query() {
    let mut t = SegmentTree::new(7, 0, |a, b| a + b);

    // Check if the buf size set to power of 2
    assert_eq!(t.get(7), 0);

    t.update(0, 1);
    assert_eq!(t.get(0), 1);

    t.update(1, 2);
    assert_eq!(t.get(1), 2);

    assert_eq!(t.query(0..2), 3);
    assert_eq!(t.query(0..3), 3);
    assert_eq!(t.query(0..4), 3);
    assert_eq!(t.query(0..5), 3);
    assert_eq!(t.query(0..6), 3);
    assert_eq!(t.query(0..7), 3);
    assert_eq!(t.query(0..8), 3);
    assert_eq!(t.query(0..8), 3);
    assert_eq!(t.query(1..2), 2);
}

#[test]
fn seg_tree_from_vec() {
    let t = SegmentTree::from_vec(vec![1, 2, 3, 4, 5, 6, 7], 0, |a, b| a + b);

    assert_eq!(t.query(0..2), 3);
    assert_eq!(t.query(0..7), 28);
}
