fn main() {
  let s1 = "abc1";
  let s2 = "abc2";
  println!("{}", s1);
  assert!(s1 < s2);
  assert!(s1 != s2);

  let s3 = "文字列を複数行に渡って書くと
            改行やスペースが入る";
  let s4 = "行末にバックスラッシュを付けると\
            改行などが入らない";

  println!("{}", s3);
  println!("{}", s4);

  let s5 = "文字列に\"と\\を含める";
  let s6 = r#"文字列に"と\を含める"#;
  println!("{}", s5);
  println!("{}", s6);

  let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";
  let mut lines = fruits.lines();

  let apple_line = lines.next();
  assert_eq!(apple_line, Some("あかりんご, あおりんご"));
  assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
  assert_eq!(lines.next(), None);

  if let Some(apples) = apple_line {
    assert!(apples.starts_with("あか"));
    assert!(apples.contains("りんご"));
    assert_eq!(apples.find("あお"), Some(17));

    let mut apple_iter = apples.split(",");
    assert_eq!(apple_iter.next(), Some("あかりんご"));

    let green = apple_iter.next();
    assert_eq!(green, Some(" あおりんご"));
    assert_eq!(green.map(str::trim), Some("あおりんご"));
    assert_eq!(apple_iter.next(), None);
  } else {
    unreachable!();
  }

  let s = "かか\u{3099}く";
  println!("{}", s);

  let mut iter = s.chars();
  assert_eq!(iter.next(), Some('か'));
  assert_eq!(iter.next(), Some('か'));
  assert_eq!(iter.next(), Some('\u{3099}'));
  assert_eq!(iter.next(), Some('く'));
  assert_eq!(iter.next(), None);

  // 可変の str
  let mut s1 = "abcあいう".to_string();
  let s2 = s1.as_mut_str();

  s2.make_ascii_uppercase();
  assert_eq!(s2, "ABCあいう");
}
