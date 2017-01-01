use std::collections::*;

pub fn make_inverted_index(documents: Vec<&str>) -> HashMap<&str, HashSet<i64>> {
  let mut map = HashMap::new();

  for (i, d) in documents.iter().enumerate() {
    for word in d.split_whitespace() {
      if !map.contains_key(word) {
        let mut set = HashSet::new();
        set.insert(i as i64);
        map.insert(word, set);
      } else {
        map.get_mut(word).unwrap().insert(i as i64);
      }
    }
  }
  return map;
}
