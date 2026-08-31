> 本节参考文档：[https://beatai.org/rust-course/basic/compound-type/struct](https://beatai.org/rust-course/basic/compound-type/struct)

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

```rust
#[derive(Debug)]
 struct File {
   name: String,
   data: Vec<u8>,
 }

 fn main() {
   let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };

   let f1_name = &f1.name;
   let f1_length = &f1.data.len();

   println!("{:?}", f1);
   println!("{} is {} bytes long", f1_name, f1_length);
 }
```

上面定义的 `File` 结构体在内存中的排列如下图所示：

![](https://img.up-4ever.site/20260828165745198.png)

从图中可以清晰地看出 `File` 结构体两个字段 `name` 和 `data` 分别拥有底层两个 `[u8]` 数组的所有权（`String` 类型的底层也是 `[u8]` 数组），通过 `ptr` 指针指向底层数组的内存地址，这里你可以把 `ptr` 指针理解为 Rust 中的引用类型。

该图片也侧面印证了：**把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段**。

> `String` 和 `Vec<T>` 在结构体里面主要保存的是 **指针（ptr）+ 长度（len）+ 容量（capacity）**，真正的数据通常存放在堆上；`File` 则把这两个“管理数据的结构”放在自己内部。

## 元组结构体

结构体必须要有名称，但是结构体的**字段**可以没有名称，这种结构体长得很像元组，因此被称为元组结构体，例如：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用。例如上面的 `Point` 元组结构体，众所周知 3D 点是 `(x, y, z)` 形式的坐标点，因此我们无需再为内部的字段逐一命名为：`x`, `y`, `z`。

## 单元结构体

单元结构体（Unit Struct）是一种没有任何字段的结构体。

```rust
struct User;

fn main() {
    let user = User;
}
```

这里的 `User` 就是一个单元结构体。

单元结构体主要用于表示一种类型或身份，常用于 Trait 实现。

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {

}
```

## 结构体数据的所有权

在之前的 `User` 结构体的定义中，有一处细节：我们使用了自身拥有所有权的 `String` 类型而不是基于引用的 `&str` 字符串切片类型。这是一个有意而为之的选择：因为我们想要这个结构体拥有它所有的数据，而不是从其它地方借用数据。

你也可以让 `User` 结构体从其它对象借用数据，不过这么做，就需要引入[生命周期(lifetimes)](https://beatai.org/rust-course/basic/lifetime)这个新概念（也是一个复杂的概念），简而言之，生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小。

总之，如果你想在结构体中使用一个引用，就必须加上生命周期，否则就会报错：

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

编译器会抱怨它需要生命周期标识符：

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter // 需要一个生命周期
  |
help: consider introducing a named lifetime parameter // 考虑像下面的代码这样引入一个生命周期
  |
1 ~ struct User<'a> {
2 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     username: &str,
3 ~     email: &'a str,
  |
```

## 使用 `#[derive(Debug)]` 来打印结构体的信息

在前面的代码中我们使用 `#[derive(Debug)]` 对结构体进行了标记，这样才能使用 `println!("{:?}", s);` 的方式对其进行打印输出，如果不加，看看会发生什么:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

首先可以观察到，上面使用了 `{}` 而不是之前的 `{:?}`，运行后报错：

```shell
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

提示我们结构体 `Rectangle` 没有实现 `Display` 特征，这是因为如果我们使用 `{}` 来格式化输出，那对应的类型就必须实现 `Display` 特征，以前学习的基本类型，都默认实现了该特征:

```rust
fn main() {
    let v = 1;
    let b = true;

    println!("{}, {}", v, b);
}
```

上面代码不会报错，那么结构体为什么不默认实现 `Display` 特征呢？原因在于结构体较为复杂，例如考虑以下问题：你想要逗号对字段进行分割吗？需要括号吗？加在什么地方？所有的字段都应该显示？类似的还有很多，由于这种复杂性，Rust 不希望猜测我们想要的是什么，而是把选择权交给我们自己来实现：如果要用 `{}` 的方式打印结构体，那就自己实现 `Display` 特征。

接下来继续阅读报错：

```shell
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

上面提示我们使用 `{:?}` 来试试，这个方式我们在本文的前面也见过，下面来试试:

```rust
println!("rect1 is {:?}", rect1);
```

可是依然无情报错了:

```shell
error[E0277]: `Rectangle` doesn't implement `Debug`
```

好在，聪明的编译器又一次给出了提示:

```shell
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

让我们实现 `Debug` 特征，Oh No，就是不想实现 `Display` 特征，才用的 `{:?}`，怎么又要实现 `Debug`，但是仔细看，提示中有一行： `add #[derive(Debug)] to Rectangle`， 哦？这不就是我们前文一直在使用的吗？

首先，Rust 默认不会为我们实现 `Debug`，为了实现，有两种方式可以选择：

- 手动实现
- 使用 `derive` 派生实现

后者简单的多，但是也有限制，具体见[附录 D](https://beatai.org/rust-course/appendix/derive)，这里我们就不再深入讲解，来看看该如何使用:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

此时运行程序，就不再有错误，输出如下:

```shell
$ cargo run
rect1 is Rectangle { width: 30, height: 50 }
```

这个输出格式看上去也不赖嘛，虽然未必是最好的。这种格式是 Rust 自动为我们提供的实现，看上基本就跟结构体的定义形式一样。

当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 `{:#?}` 来替代 `{:?}`，输出如下:

```shell
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

此时结构体的输出跟我们创建时候的代码几乎一模一样了！当然，如果大家还是不满足，那最好还是自己实现 `Display` 特征，以向用户更美的展示你的私藏结构体。关于格式化输出的更多内容，我们强烈推荐看看这个[格式化输出章节](https://beatai.org/rust-course/basic/formatted-output#debug-%E7%89%B9%E5%BE%81)。

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

Trait、impl、struct 的关系

```
struct
  ↓
定义“是什么”
  ↓
具体类型


trait
  ↓
定义“能做什么”
  ↓
一组行为/能力


impl
  ↓
把某种能力赋予某个类型
```

例如：

```rust
struct Dog;

trait Speak {
    fn speak(&self);
}

impl Speak for Dog {
    fn speak(&self) {
        println!("汪汪");
    }
}
```

就是：

```
Dog
 │
 │ 实现
 ↓
Speak
 │
 ↓
拥有 speak() 能力
```

单元结构体 + Trait

两者经常一起出现：

```rust
struct ConsoleLogger;

trait Logger {
    fn log(&self);
}

impl Logger for ConsoleLogger {
    fn log(&self) {
        println!("logging...");
    }
}
```

这里：

- `ConsoleLogger` → 一个**没有数据的具体类型**
- `Logger` → 定义“日志记录”这个**能力**
- `impl Logger for ConsoleLogger` → 让 `ConsoleLogger` **具备这个能力**

### 一句话记忆

> **单元结构体：没有数据，只表示一个类型。**
> **Trait：不负责存数据，只描述一个类型“能做什么”。**
> **`impl`：让具体类型获得 Trait 定义的能力。**

