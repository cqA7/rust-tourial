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

