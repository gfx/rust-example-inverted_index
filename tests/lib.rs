extern crate inverted_index;

#[macro_use] extern crate maplit;

use std::collections::*;
use inverted_index::*;

#[test]
fn test_inverted_index() {
  let inverted_index = make_inverted_index(vec![
  "it is what it is",
  "what is it",
  "it is a banana",
  ]);

  let mut expected = HashMap::new();

  expected.insert("a", hashset!{2 as i64});
  expected.insert("banana", hashset!{2 as i64});
  expected.insert("is", hashset!{0 as i64, 1 as i64, 2 as i64});
  expected.insert("it", hashset!{0 as i64, 1 as i64, 2 as i64});
  expected.insert("what", hashset!{0 as i64, 1 as i64});

  assert_eq!(inverted_index, expected);
}
