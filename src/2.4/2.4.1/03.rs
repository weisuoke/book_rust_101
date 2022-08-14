fn main() {
  // String 与 &str 的转换: 取引用即可
  let s = String::from("hello, world!");
  say_hello(&s);
  say_hello(&s[..]);
  say_hello(s.as_str());
  // 实际上这种灵活用法是因为 deref 隐式强制转换。
}

fn say_hello(s: &str) {
  println!("{}", s)
}