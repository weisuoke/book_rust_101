/*
在 Rust 1.59 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了。

这种使用方式跟之前的 let 保持了一致性，但是 let 会重新绑定，而这里仅仅是对之前绑定的变量进行再赋值。

需要注意的是，使用 += 的赋值语句还不支持解构式赋值。
 */
struct Struct {
  e: i32
}

fn main() {
  let (a, b, c, d, e);

  (a, b) = (1, 2);
  // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
  [c, .., d, _] = [1, 2, 3, 4, 5];
  Struct { e, .. } = Struct { e: 5 };

  println!("{} {} {} {} {}", a, b, c, d, e);

  assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}