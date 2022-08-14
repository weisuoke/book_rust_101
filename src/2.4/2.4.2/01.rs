fn main() {
  // 元组是由多种类型组合到一起形成的，因此它是符合类型，元组的长度是固定的，元组元素的顺序也是固定的

  // 可以通过以下语法创建一个元组
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // 用模式匹配解构元组
  // 首先创建一个元组，然后将其绑定到 tup 上，接着使用 let (x, y, z) = tup; 来完成一次模式匹配
  // 因为元组是(n1, n2, n3) 形式的，因此我们用一模一样的 (x, y, z) 形式来进行匹配，元组中对应的值会绑定到变量 x, y, z 上
  // 这就是解构：用同样的形式把一个复杂对象中的值匹配出来
  let (x, y, z) = tup;
  println!("The value of y is: {}", y);

  // 用 . 来访问元组
  // 和其他语言的数组、字符串一样，元组的索引从 0 开始
  let m = tup;
  let five_hundred = m.0;
  let six_point_four = m.1;
  let one = m.2;

  // 元组的使用示例
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("The length of '{}' is {}", s2, len);
}

// calculate_length 函数接收 s1 字符串的所有权，然后计算字符串的长度，接着把字符串所有权和字符串长度再返回给 s2 和 len 变量
fn calculate_length (s: String) -> (String, usize) {
  let length = s.len();   // len() 返回字符串的长度

  (s, length)
}