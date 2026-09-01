> 本节参考文档：[https://beatai.org/rust-course/basic/flow-control](https://beatai.org/rust-course/basic/flow-control)

## 流程控制

### 使用 `if` 来做分支控制

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

以上代码有以下几点要注意：

- **`if` 语句块是表达式**，这里我们使用 `if` 表达式的返回值来给 `number` 进行赋值：`number` 的值是 `5`
- 用 `if` 来赋值时，要保证每个分支返回的类型一样（事实上，这种说法不完全准确，见[这里](https://beatai.org/rust-course/appendix/expressions#if%E8%A1%A8%E8%BE%BE%E5%BC%8F)），此处返回的 `5` 和 `6` 就是同一个类型，如果返回类型不一致就会报错

### 使用 `else if` 处理多重条件

```rust
fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

程序执行时，会按照自上至下的顺序执行每一个分支判断，一旦成功，则跳出 `if` 语句块，最终本程序会匹配执行 `else if n % 3 == 0` 的分支，输出 `"number is divisible by 3"`。

有一点要注意，就算有多个分支能匹配，也只有第一个匹配的分支会被执行！

## 循环控制

在 Rust 语言中有三种循环方式：`for`、`while` 和 `loop`，其中 `for` 循环是 Rust 循环王冠上的明珠。

### for 循环

`for` 循环是 Rust 的大杀器：

```rust
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
```

以上代码循环输出一个从 1 到 5 的序列，简单粗暴，核心就在于 `for` 和 `in` 的联动，语义表达如下：

```rust
for 元素 in 集合 {
  // 使用元素干一些你懂我不懂的事情
}
```

注意，使用 `for` 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 `container` 的引用）。如果不使用引用的话，所有权会被转移（move）到 `for` 语句块中，后面就无法再使用这个集合了)：

```rust
for item in &container {
  // ...
}
```

> 对于实现了 `copy` 特征的数组（例如 [i32; 10]）而言， `for item in arr` 并不会把 `arr` 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 `arr` 。

如果想在循环中，**修改该元素**，可以使用 `mut` 关键字：

```rust
for item in &mut collection {
  // ...
}
```

总结如下：

| 使用方法                          | 等价使用方式                                            | 所有权   |
| ----------------------------- | ------------------------------------------------- | ----- |
| `for item in collection`      | `for item in IntoIterator::into_iter(collection)` | 转移所有权 |
| `for item in &collection`     | `for item in collection.iter()`                   | 不可变借用 |
| `for item in &mut collection` | `for item in collection.iter_mut()`               | 可变借用  |

如果想在循环中**获取元素的索引**：

```rust
fn main() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
```

有同学可能会想到，如果我们想用 `for` 循环控制某个过程执行 10 次，但是又不想单独声明一个变量来控制这个流程，该怎么写？

```rust
for _ in 0..10 {
  // ...
}
```

可以用 `_` 来替代 `i` 用于 `for` 循环中，在 Rust 中 `_` 的含义是忽略该值或者类型的意思，如果不使用 `_`，那么编译器会给你一个 `变量未使用的` 的警告。

#### continue

使用 `continue` 可以跳过当前当次的循环，开始下次的循环：

```rust
 for i in 1..4 {
     if i == 2 {
         continue;
     }
     println!("{}", i);
 }
```

上面代码对 1 到 3 的序列进行迭代，且跳过值为 2 时的循环，输出如下：

```shell
1
3
```

#### break

使用 `break` 可以直接跳出当前整个循环：

```rust
 for i in 1..4 {
     if i == 2 {
         break;
     }
     println!("{}", i);
 }
```

上面代码对 1 到 3 的序列进行迭代，在遇到值为 2 时的跳出整个循环，后面的循环不再执行，输出如下：

```shell
1
```

### while 循环

如果你需要一个条件来循环，当该条件为 `true` 时，继续循环，条件为 `false`，跳出循环，那么 `while` 就非常适用：

```rust
fn main() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }

    println!("我出来了！");
}
```

该 `while` 循环，只有当 `n` 小于等于 `5` 时，才执行，否则就立刻跳出循环，因此在上述代码中，它会先从 `0` 开始，满足条件，进行循环，然后是 `1`，满足条件，进行循环，最终到 `6` 的时候，大于 5，不满足条件，跳出 `while` 循环，执行 `我出来了` 的打印，然后程序结束：

```shell
0!
1!
2!
3!
4!
5!
我出来了！
```

当然，你也可以用其它方式组合实现，例如 `loop`（无条件循环，将在下面介绍） + `if` + `break`：

```rust
fn main() {
    let mut n = 0;

    loop {
        if n > 5 {
            break
        }
        println!("{}", n);
        n+=1;
    }

    println!("我出来了！");
}
```

可以看出，在这种循环场景下，`while` 要简洁的多。

## loop 循环

对于循环而言，`loop` 循环毋庸置疑，是适用面最高的，它可以适用于所有循环场景（虽然能用，但是在很多场景下， `for` 和 `while` 才是最优选择），因为 `loop` 就是一个简单的无限循环，你可以在内部实现逻辑通过 `break` 关键字来控制循环何时结束。

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

以上代码当 `counter` 递增到 `10` 时，就会通过 `break` 返回一个 `counter * 2` 的值，最后赋给 `result` 并打印出来。

这里有几点值得注意：

- **break 可以单独使用，也可以带一个返回值**，有些类似 `return`
- **loop 是一个表达式**，因此可以返回一个值