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

在实际场景中，有一种情况很常见：根据已有的结构体实例，创建新的结构体实例，例如根据已有的 `user1` 实例来构建 `user2`：

```rust
  let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
```

老话重提，如果你从 TypeScript 过来，肯定觉得啰嗦爆了：竟然手动把 `user1` 的三个字段逐个赋值给 `user2`，好在 Rust 为我们提供了 `结构体更新语法`：

```rust
  let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

因为 `user2` 仅仅在 `email` 上与 `user1` 不同，因此我们只需要对 `email` 进行赋值，剩下的通过结构体更新语法 `..user1` 即可完成。

`..` 语法表明凡是我们没有显式声明的字段，全部从 `user1` 中自动获取。需要注意的是 `..user1` **必须在结构体的尾部使用**。

> 结构体更新语法跟赋值语句 `=` 非常相像，因此在上面代码中，`user1` 的部分字段所有权被转移到 `user2` 中：`username` 字段发生了所有权转移，作为结果，`user1` 无法再被使用。
> 
> 聪明的读者肯定要发问了：明明有三个字段进行了自动赋值，为何只有 `username` 发生了所有权转移？
> 
> 仔细回想一下[所有权](https://beatai.org/rust-course/basic/ownership/ownership#%E6%8B%B7%E8%B4%9D%E6%B5%85%E6%8B%B7%E8%B4%9D)那一节的内容，我们提到了 `Copy` 特征：实现了 `Copy` 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，其中 `bool` 和 `u64` 类型就实现了 `Copy` 特征，因此 `active` 和 `sign_in_count` 字段在赋值给 `user2` 时，仅仅发生了拷贝，而不是所有权转移。
> 
> 值得注意的是：`username` 所有权被转移给了 `user2`，导致了 `user1` 无法再被使用，但是并不代表 `user1` 内部的其它字段不能被继续使用，例如：

```rust
#[derive(Debug)]
struct User {
	active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
	let user1 = User {
	    email: String::from("someone@example.com"),
	    username: String::from("someusername123"),
	    active: true,
	    sign_in_count: 1,
	};
	let user2 = User {
	    active: user1.active,
	    username: user1.username,
	    email: String::from("another@example.com"),
	    sign_in_count: user1.sign_in_count,
	};
	println!("{}", user1.active);
	// 下面这两行会报错
	println!("{:?}", user1);
	println!("{:?}", user1.username);
}
```


### 结构体的内存排列



![](https://img.up-4ever.site/20260828165745198.png)


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

---

关于 `#[derive(Debug)]`

`#[derive(Debug)]` 用于让 Rust **自动为结构体、枚举等类型实现 `Debug` trait**，从而可以使用 `{:?}` 或 `{:#?}` 进行调试输出。

```rust
#[derive(Debug)]
struct User {
    name: String,
}

let user = User {
    name: String::from("Tom"),
};

println!("{:?}", user);
// User { name: "Tom" }

println!("{:#?}", user);
// User {
//     name: "Tom",
// }
```

**要点：**

- `derive`：让 Rust 自动生成 trait 的实现
- `Debug`：用于调试输出
- `{:?}`：紧凑的调试格式
- `{:#?}`：格式化的、易读的调试格式
- 结构体中的字段也必须支持 `Debug`，才能自动派生 `Debug`
- `Debug` 主要用于**开发和调试**，不是面向最终用户的输出格式

> `#[derive(Debug)]` = **自动让类型支持 `{:?}` 调试打印。**