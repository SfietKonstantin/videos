extern crate videos;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use videos::output::produce_output;

#[test]
fn test_produce_output() {
    let mut result: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    result.insert(0, vec![1, 2, 3].iter().cloned().collect());
    result.insert(1, BTreeSet::new());
    result.insert(2, vec![1, 4].iter().cloned().collect());

    assert_eq!("2\n0 1 2 3\n2 1 4\n", produce_output(result));
}