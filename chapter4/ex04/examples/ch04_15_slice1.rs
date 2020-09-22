fn main() {
  fn print_info(name: &str, sl: &[char]) {
    println!(
      "  {:9} - {}, {:?}, {:?}, {:?}",
      name,
      sl.len(),
      sl.first(),
      sl[1],
      sl.last()
    )
  }

  let a1 = ['a', 'b', 'c', 'd'];
  println!("a1: {:?}", a1);

  print_info("&a1[..]", &a1[..]);
  print_info("&a1", &a1);
  print_info("&a1[1..3]", &a1[1..3]);
  println!("a1: {:?}", a1); // &a1[1..3] はイミュータブルなスライスが作られるためオリジナルの a1 に変化はない

  let v1 = vec!['e', 'f', 'g', 'h'];
  println!("v1: {:?}", v1);

  print_info("&v1[..]", &v1[..]);
  print_info("&v1", &v1);
  print_info("&v1[1..3]", &v1[1..3]);
}
