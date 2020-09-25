fn main() {
  let mut a1 = [5, 4, 3, 2];
  let s1 = &mut a1[1..3];

  s1[0] = 6;
  s1[1] *= 10;
  s1.swap(0, 1);
  assert_eq!(s1, [30, 6]);

  assert_eq!(a1, [5, 30, 6, 2]);

  // 不変スライスと可変スライス共通の操作
  let a2: [i32; 0] = [];
  assert!(a2.is_empty());
  assert_eq!(a2.len(), 0);
  assert_eq!(a2.first(), None);

  let a3 = ["zero", "one", "two", "three", "four"];
  let s3 = &a3[1..4];
  assert!(!s3.is_empty());
  assert_eq!(s3.len(), 3);
  assert_eq!(s3.first(), Some(&"one"));
  assert_eq!(s3[1], "two");
  assert_eq!(s3.get(1), Some(&"two"));
  assert_eq!(s3.get(3), None);

  assert!(s3.contains(&"two"));
  assert!(s3.starts_with(&["one", "two"]));
  assert!(s3.ends_with(&["two", "three"]));

  // 可変のスライスだけで共通の操作
  let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

  &mut a4[2..6].sort();
  assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

  let (s4a, s4b) = &mut a4.split_at_mut(5);
  println!("{:?}", s4a);
  println!("{:?}", s4b);

  s4a.reverse();
  assert_eq!(s4a, &[8, 2, 0, 4, 6]);

  s4b.sort_unstable();
  assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);
}
