fn main() {
  let c1 = 'A';
  let c2 = 'a';

  assert!(c1 < c2);
  assert!(c1.is_uppercase());

  let c3 = '0';
  assert!(c3.is_digit(10));

  let c4 = '\t';
  let c5 = '\n';
  let c6 = '\'';
  let c7 = '\\';
  let c8 = '\x7F';
  let c9 = '漢';

  println!("{}, {}, {}, {}, {}, {}", c4, c5, c6, c7, c8, c9);

  let c10 = '\u{5b57}';
  let c11 = '\u{1f600}';

  println!("{}, {}", c10, c11);
}
