fn main() {
  fn hello() {
    println!("Hello, World!");
  }

  let ret = hello();

  // 返り値のない関数の返り値はユニット型
  assert_eq!(ret, ());

  // 指定された型がメモリ上で占める大きさをバイトで表示
  assert_eq!(std::mem::size_of::<()>(), 0);
  assert_eq!(std::mem::size_of::<u32>(), 4);
}
