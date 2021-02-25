fn main() {
  let f1 = 10.0;
  let f2 = -1_234.56f32;
  let f3 = 578.6E+77;

  println!("{}, {}, {}", f1, f2, f3);
  println!("10.4 ceil:  {} ", 10.4f32.ceil());
  println!("10.4 round: {} ", 10.4f32.round());
  println!("10.4 floor: {} ", 10.4f32.floor());
  println!("10.5 ceil:  {} ", 10.5f32.ceil());
  println!("10.5 round: {} ", 10.5f32.round());
  println!("10.5 floor: {} ", 10.5f32.floor());
  println!("{}, {} ", f1, f1.to_string());
  println!("{} ", std::f32::consts::PI);
}
