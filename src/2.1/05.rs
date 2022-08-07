/*
Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：
 */
fn main() {
  let x = 5;
  // 在 main 函数的作用域内对之前的 x 进行遮蔽
  let x = x + 1;

  {
    // 在当前的花括号作用域内，对之前的 x 进行遮蔽
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
  }

  println!("The value of x is: {}", x);
}