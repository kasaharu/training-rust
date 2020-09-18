fn main() {
  let n1 = 10_000;
  let n2 = 0u8;
  let n3 = -100_isize;

  println!("{}, {}, {}", n1, n2, n3);

  let n4 = 10;
  let n5 = n3 + n4;
  println!("{}, {}", n4, n5);

  let h1 = 0xff;
  let o1 = 0o744;
  let b1 = 0b1010_0110_1110_1001;
  println!("{}, {}, {}", h1, o1, b1);

  let n6 = b'A';
  assert_eq!(n6, 65u8);
}
