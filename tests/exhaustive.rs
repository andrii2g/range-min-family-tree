use range_min_family_tree::CartesianTree;

#[test]
fn every_interval_of_every_short_ternary_array_matches_a_scan() {
    let mut arrays_checked = 0;
    let mut intervals_checked = 0;

    for length in 1..=6 {
        for mut encoding in 0..3_i64.pow(length) {
            let mut values = vec![0; length as usize];
            for value in &mut values {
                *value = encoding % 3;
                encoding /= 3;
            }
            let tree = CartesianTree::new(&values);
            arrays_checked += 1;
            for left in 0..values.len() {
                for right in left..values.len() {
                    let mut expected = left;
                    for index in left + 1..=right {
                        if values[index] < values[expected] {
                            expected = index;
                        }
                    }

                    let query = tree.query(left, right).unwrap();
                    assert_eq!(
                        query.minimum_index, expected,
                        "{values:?} [{left}, {right}]"
                    );
                    for (start, path) in [(left, &query.left_path), (right, &query.right_path)] {
                        assert_eq!(path.first(), Some(&start));
                        assert_eq!(path.last(), Some(&expected));
                        for step in path.windows(2) {
                            assert_eq!(tree.nodes()[step[0]].parent, Some(step[1]));
                        }
                    }
                    intervals_checked += 1;
                }
            }
        }
    }

    assert_eq!(arrays_checked, 1_092);
    assert_eq!(intervals_checked, 19_956);
}

#[test]
fn empty_tree_and_invalid_intervals() {
    let empty = CartesianTree::new(&[]);
    assert_eq!(empty.root(), None);
    assert!(empty.nodes().is_empty());
    assert_eq!(empty.query(0, 0), None);

    let tree = CartesianTree::new(&[3, -1]);
    assert_eq!(tree.query(1, 0), None);
    assert_eq!(tree.query(0, 2), None);
}
