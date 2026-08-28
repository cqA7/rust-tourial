> 本节参考文档：https://beatai.org/rust-course/basic/compound-type/struct

## 结构体

结构体跟之前讲过的[元组](https://beatai.org/rust-course/basic/compound-type/tuple)有些相像：都是由多种类型组合而成。但是与元组不同的是，结构体可以为内部的每个字段起一个富有含义的名称。因此结构体更加灵活更加强大，你无需依赖这些字段的顺序来访问和解析它们。

### 结构体语法

#### 定义结构体

一个结构体由几部分组成：

- 通过关键字 `struct` 定义
- 一个清晰明确的结构体 `名称`
- 几个有名字的结构体 `字段`

例如：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

该结构体名称是 `User`，拥有 4 个字段，且每个字段都有对应的字段名及类型声明，例如 `username` 代表了用户名，其类型是 String（一种支持内容修改的字符串类型）。

#### 创建结构体实例

为了使用上述结构体，我们需要创建 `User` 结构体的**实例**：

```rust
let user = User {
	email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

注意:

1. 初始化实例时，**每个字段**都需要进行初始化
2. 初始化时的字段顺序**不需要**和结构体定义时的顺序一致

#### 访问结构体字段

通过 `.` 操作符即可访问结构体实例内部的字段值，也可以修改它们：

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

需要注意的是，必须要将结构体实例声明为可变的，才能修改其中的字段，Rust **不支持**将某个结构体**某个字段**标记为可变。

#### 简化结构体创建

下面的函数类似一个构建函数，返回了 `User` 结构体的实例：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

当函数参数和结构体字段同名时，可以直接使用缩略的方式进行初始化，跟 TypeScript 中一模一样。

#### 结构体更新语法





---

在编码中发现：user_servie.rs 文件中，通过 `use crate::user::User;` 和 `use crate::User;` 都可以正常编译，这个很奇怪。明明 `User` 是定义在 `user.rs` 中的，通过后者怎么能访问的到呢？

带着这个问题，问了下 codex，结论如下：

原因在于 Rust 的可见性规则 + `use` 别名的工作方式。

**关键链条**

1. `main.rs` 是 crate 根,c它里面有一行 `use user::User;`。这个 `use` 声明会在 **crate 根模块**里创建一个名为 `User` 的私有别名(它指向 `user::User`)。
2. `user_service` 是 `main` 的子模块(由 `mod user_service;` 声明)。在 Rust 中,一个模块里的私有 item 对该模块**及其所有后代模块**可见。也就是说,祖先模块的私有成员,后代模块可以访问。
3. 所以 `user_service.rs` 里写 `use crate::User;`,`crate::User` 解析到 crate 根的那个私有 `use` 别名,而 `user_service` 作为后代模块有权访问它,因此不报错。

**对比说明**

- 这里能用到的是 `main.rs` 里那行 `use user::User;` 创建的别名,而不是 `user.rs` 里 `pub struct User` 本身。
- 如果把 `main.rs` 里的 `use user::User;` 删掉,`crate::User` 就找不到了,会报 `unresolved import` / `cannot find type User in crate root`。
- 顺带一提,你注释掉的那行 `use crate::user::User;` 其实也能通过:`mod user;` 是 crate 根里的私有模块,同样对后代可见,所以 `crate::user::User` 也能解析。两条路径都因"祖先私有成员对后代可见"而成立。

**一句话总结**:`crate::User` 没报错,是因为 crate 根 `main.rs` 用 `use user::User;` 把 `User` 这个名字带进了根作用域,而 **Rust 允许子模块访问祖先模块的私有 item**。

如果想让根层的 `User` 也能被 crate 外部(比如别的 crate)使用,需要写成 `pub use user::User;`(再导出);现在只是内部模块能用,所以当前的私有 `use` 已经够用了。