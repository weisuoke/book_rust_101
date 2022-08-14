fn main() {
  let s = String::from("Hello");

  // 在使用 Rust 的 .. range 序列语法时，如果你想从索引 0 开始，可以使用如下的方式，这两个是等效的：
  let slice = &s[0..2];
  let slice = &s[..2];

  // 同样的, 如果你的切片想要包含 String 的最后一个字节符，则可以这样使用
  let len = s.len();
  let slice = &s[4..len];
  let slice = &s[4..];

  // 你也可以截取完整的 String 切片
  let slice = &s[0..len];
  let slice = &s[..];
}