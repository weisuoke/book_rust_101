fn main() {
  // 字符
  // 如果你想要以 Unicode 字符的方式遍历字符串，最好的办法是使用 chars 方法
  for c in "中国人".chars() {
    println!("{}", c)
  }

  // 字节
  // 这种方式是返回字符串的底层字节数组表现形式
  for b in "中国人".bytes() {
    println!("{}", b)
  }
}