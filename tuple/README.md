> 本节参考文档：https://beatai.org/rust-course/basic/compound-type/tuple

## 元组

元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。

可以通过以下语法创建一个元组：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

可以使用模式匹配或者 `.` 操作符来获取元组中的值。

### 模式匹配解构元组

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

### 用 `.` 来访问元组

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

和其它语言的数组、字符串一样，元组的索引从 0 开始。

