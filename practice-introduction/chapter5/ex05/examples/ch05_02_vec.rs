fn main() {
  let v1 = vec![false, true, false];
  let v2 = vec![0.0, -1.0, 1.0, 0.5];

  assert_eq!(v1.len(), 3);
  assert_eq!(v2.len(), 4);

  let v3 = vec![0; 100];
  assert_eq!(v3.len(), 100);

  let v4 = vec![vec!['a', 'b', 'c'], vec!['d']];
  println!("{:?}", v4);

  let mut v6 = vec!['a', 'b', 'c'];
  v6.push('d');
  v6.push('e');
  println!("{:?}", v6);

  assert_eq!(v6.pop(), Some('e'));
  println!("{:?}", v6);
  v6.insert(1, 'f');
  assert_eq!(v6.remove(2), 'b');
  assert_eq!(v6, ['a', 'f', 'c', 'd']);

  let mut v7 = vec!['g', 'h'];
  v6.append(&mut v7);
  assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
  assert_eq!(v7, []);

  let v8 = vec!['i', 'j'];
  v6.extend_from_slice(&v8);
  assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
  assert_eq!(v8, ['i', 'j']);
}
