fn main() {
  fn double(n: i32) -> i32 {
    n + n
  }

  fn abs(n: i32) -> i32 {
    if n >= 0 {
      n
    } else {
      -n
    }
  }

  let mut f: fn(i32) -> i32 = double;
  let f_bad = double;
  assert_eq!(f(-42), -84);
  assert_eq!(f_bad(-42), -84);
  assert_eq!(double(-42), -84);
  assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

  f = abs;
  assert_eq!(f(-42), 42);
}
