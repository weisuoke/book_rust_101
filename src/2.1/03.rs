/*
let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：
 */
fn main() {
  let (a, mut b): (bool, bool) = (true, false);
  // a = true，不可变；b = false，可变
  println!("a = {:?}, b = {:?}", a, b);

  b = true;
  assert_eq!(a, b);
}