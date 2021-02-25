fn main() {
  use std::collections::HashMap;


  let mut m1 = HashMap::new();
  m1.insert("a", 1);
  m1.insert("b", 3);
  assert_eq!(m1.len(), 2);
  assert_eq!(m1.get("b"), Some(&3));
  assert_eq!(m1.get("a"), Some(&1));
  assert_eq!(m1.get("c"), None);

  let d = m1.entry("d").or_insert(0);
  *d += 7;
  *d += 3;
  assert_eq!(m1.get("d"), Some(&10));

  // let m2 = vec![("a", 1), ("b", 3)].into_iter().collect::<HashMap<_, _>>();
}
