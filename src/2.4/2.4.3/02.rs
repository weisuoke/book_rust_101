/*
    结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体

    元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用。例如上面的 Point 元组结构体
    众所周知 3D 点是 (x, y, x) 形式的坐标点，因此我们无需再为内部的字段逐一命名为：x, y, z
 */
fn main() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}