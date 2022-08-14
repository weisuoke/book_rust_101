/*
  一个结构体由几个部分组成
  - 通过关键字 struct 定义
  - 一个清晰明确的结构体 名称
  - 几个有名字的结构体 字段
 */
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64
}

fn main() {
  // 为了使用 User 结构体，我们需要创建 User 结构体的实例：
  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1
  };

  /*
    NOTE:
    1. 初始化实例时，每个字段都需要进行初始化
    2. 初始化的字段顺序不需要和结构体定义时的顺序一致
   */

  // 通过 . 操作符即可访问结构体实例内部的字段值，也可以修改他们
  user1.email = String::from("anotheremail@example.com");

  // 根据已有的结构体实例，创建新的结构体实例，例如根据已有的 user1 实例来构建 user2
  let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count
  };

  // 简化
  let user3 = User {
    email: String::from("another@example.com"),
    // .. 语法表明凡是是我们乜有显示声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用
    ..user1
  };
}

/*
  简化结构体创建
  它接收两个字符串参数：email 和 username，然后使用它们来创建一个 User 结构体，并且返回。
 */
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}