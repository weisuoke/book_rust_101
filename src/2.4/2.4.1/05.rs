fn main() {
  let mut s = String::from("Hello ");
  s.push('r');
  println!("追加字符 push() -> {}", s);

  s.push_str("ust!");
  println!("追加字符串 push_str() -> {}", s);
}