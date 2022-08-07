/*
Rust 的变量在默认情况下是不可变的。前文提到，这是 Rust 团队为我们精心设计的语言特性之一，让我们编写的代码更安全，性能也更好。当然你可以通过 mut 关键字让变量变为可变的，让设计更灵活。

如果变量 a 不可变，那么一旦为它绑定值，就不能再修改 a。举个例子，在我们的工程目录下使用 cargo new variables 新建一个项目，叫做 variables 。

在 Rust 中，可变性很简单，只要在变量名前加一个 mut 即可, 而且这种显式的声明方式还会给后来人传达这样的信息：嗯，这个变量在后面代码部分会发生改变。

为了让变量声明为可变,将 src/main.rs 改为以下内容：
 */
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}