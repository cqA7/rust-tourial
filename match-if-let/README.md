> 本节参考文档：[https://beatai.org/rust-course/basic/match-pattern/match-if-let](https://beatai.org/rust-course/basic/match-pattern/match-if-let)

## match 和 if-let

在 Rust 中，模式匹配最常用的就是 `match` 和 `if let`，本章节将对两者及相关的概念进行详尽介绍。

先来看一个关于 `match` 的简单例子：

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

这里我们想去匹配 `dire` 对应的枚举类型，因此在 `match` 中用三个匹配分支来完全覆盖枚举变量 `Direction` 的**所有**成员类型，有以下几点值得注意：

- `match` 的匹配必须要穷举出**所有**可能，因此这里用 `_` 来代表未列出的所有可能性
- `match` 的每一个分支都必须是一个**表达式**，且所有分支的表达式最终返回值的**类型必须相同**
- **X | Y**，类似逻辑运算符 `或`，代表该分支可以匹配 `X` 也可以匹配 `Y`，只要满足一个即可

其实 `match` 跟其他语言中的 `switch` 非常像，`_` 类似于 `switch` 中的 `default`。

### match 匹配

首先来看看 `match` 的通用形式：

```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```

该形式清晰的说明了何为模式，何为模式匹配：将模式与 `target` 进行匹配，即为模式匹配，而模式匹配不仅仅局限于 `match`，后面我们会详细阐述。

`match` 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行对应的代码，下面让我们来一一详解，先看一个例子：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

`value_in_cents` 函数根据匹配到的硬币，返回对应的美分数值。`match` 后紧跟着的是一个**表达式**，跟 `if` 很像，但是 `if` 后的表达式必须是一个布尔值，而 `match` 后的表达式返回值可以是任意类型，只要能跟后面的分支中的模式匹配起来即可，这里的 `coin` 是枚举 `Coin` 类型。

接下来是 `match` 的分支。一个分支有两个部分：**一个模式和针对该模式的处理代码**。第一个分支的模式是 `Coin::Penny`，其后的 `=>` 运算符将模式和将要运行的代码分开。这里的代码就仅仅是表达式 `1`，不同分支之间使用逗号分隔。

当 `match` 表达式执行时，它将目标值 `coin` 按顺序依次与每一个分支的模式相比较，如果模式匹配了这个值，那么模式之后的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支。

每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 `match` 表达式的返回值。如果分支有多行代码，那么需要用 `{}` 包裹，同时最后一行代码需要是一个表达式。

#### 使用 `match` 表达式赋值

还有一点很重要，`match` 本身也是一个表达式，因此可以用它来赋值：

```rust
enum IpAddr {
   Ipv4,
   Ipv6
}

fn main() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}
```

因为这里匹配到 `_` 分支，所以将 `"::1"` 赋值给了 `ip_str`。

#### 模式绑定

模式匹配的另外一个重要功能是从模式中取出绑定的值，例如：

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```

运行后输出：

```shell
$ cargo run
   Compiling world_hello v0.1.0 (/Users/sunfei/development/rust/world_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/world_hello`
Hello Rust
point from (0, 0) move to (1, 2)
change color into '(r:255, g:255, b:0)', 'b' has been ignored
```


#### 穷尽匹配

`match` 的匹配必须穷尽所有情况，下面来举例说明，例如：

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
    };
}
```

我们没有处理 `Direction::West` 的情况，因此会报错：

```shell
error[E0004]: non-exhaustive patterns: `West` not covered // 非穷尽匹配，`West` 没有被覆盖
  --> src/main.rs:10:11
   |
1  | / enum Direction {
2  | |     East,
3  | |     West,
   | |     ---- not covered
4  | |     North,
5  | |     South,
6  | | }
   | |_- `Direction` defined here
...
10 |       match dire {
   |             ^^^^ pattern `West` not covered // 模式 `West` 没有被覆盖
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `Direction`
```

#### `_` 通配符

当我们不想在匹配时列出所有值的时候，可以使用 Rust 提供的一个特殊**模式**，例如，`u8` 可以拥有 0 到 255 的有效的值，但是我们只关心 `1、3、5 和 7` 这几个值，不想列出其它的 `0、2、4、6、8、9 一直到 255` 的值。那么, 我们不必一个一个列出所有值, 因为可以使用特殊的模式 `_` 替代：

> 必须放在最后一个

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

通过将 `_` 其放置于其他分支后，`_` 将会匹配所有遗漏的值。`()` 表示返回**单元类型**与所有分支返回值的类型相同，所以当匹配到 `_` 后，什么也不会发生。

除了`_`通配符，用一个变量来承载其他情况也是可以的。

```rust
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}
```

然而，在某些场景下，我们其实只关心**某一个值是否存在**，此时 `match` 就显得过于啰嗦。