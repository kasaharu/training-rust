fn main() {
  let a1 = [false, true, false]; // [bool: 3] 型
  let a2 = [0.0, -1.0, 1.0, 0.5]; // [f64: 4] 型

  assert_eq!(a2.len(), 4);
  assert_eq!(a1.len(), 3);

  let a3 = [0; 100];
  assert_eq!(a3.len(), 100);

  let a4 = [['a', 'b'], ['c', 'd']];
  assert_eq!(a4.len(), 2);

  let array1 = ['H', 'e', 'l', 'l', 'o'];
  assert_eq!(array1[1], 'e');

  let mut array2 = [0, 1, 2];
  array2[1] = 10;
  assert_eq!(array2, [0, 10, 2]);
  assert_eq!(array2.get(1), Some(&10));
  assert_eq!(array2.get(4), None);

  let array4 = ['a'; 5];
  for ch in array4.iter() {
    print!("{}, ", *ch);
  }

  let mut array5 = [1; 5];
  for n in array5.iter_mut() {
    *n *= 2;
    print!("{}, ", *n);
  }
}
