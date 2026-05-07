# String

字节的集合

Rust核心语言:str(&str)

String:来自标准库

​	可变 可增长 拥有所有权 UTF-8



## 创建String

```rust
let mut s = String::new();
```

## 附加

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

## 连接String

* +:使用add方法

  * ```rust
    fn add(self,s:&str)->String{}
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    ```

* 使用format!宏

  * ```rust
    let s = format!("{s1}-{s2}-{s3}");
    ```

## 对String进行索引访问

Rust不允许通过索引访问String的元素

## 对String进行切片

如果真的需要使用索引来创建String切片,必须更加具体,可使用[]配合Range:

```rust
let hello = "3241jksdahfjk";

let s = &hello[0..4];
```

## 遍历String

* 单个Unicode标量值->.chars()方法

* 原始字节->.bytes()方法