权限是定义在位置上的

位置上任何可以放在赋值语句左边的东西



位置包括:

​	变量

​	位置的解引用

​	位置的数组访问

​	位置的字段访问

​	上述的任意组合



数据不可以同时被别名和被修改

## 切片

String Slices

创建:[起始索引..结束索引] *不包括结束索引位置的元素*

```rust
let s = String::from("hello world");

let hello:&str = &s[0..5];
let world:&str = &s[6..11];
let s2:&String = &s;
```

Slice是特殊的引用类型,因为他们是"fat"指针,带有元数据



注意:字符串切片范围索引必须出现在有效的UTF-8字符边界上,如果你尝试在多字节字符的中间创建字符串切片,程序将会出错并退出



* 字符串字面值是切片

  `let s = "Hello, world!";`

  * s的类型是&str:它是指向二进制文件特定位置的切片
  * 这也是为什么字符串字面量是不可变的
  * &str是一个不可变的引用

## 结构体

自定义类型,其字段可包含多种数据类型,需要给每个结构体命名



在 Rust 中，当你使用 `..user1`（更新语法/Struct Update Syntax）时，实际上是对 `user1` 中的各个字段执行了**赋值**操作。

### 1. 核心规则：Move 还是 Copy？

- **如果字段实现了 `Copy` 特性**（如 `i32`, `bool`, `f64`, 数组等）：该字段会被**复制**到新结构体中，`user1` 仍然拥有该字段。
- **如果字段没有实现 `Copy` 特性**（如 `String`, `Vec`, 自定义结构体等）：该字段的所有权会被 **Move（移动）** 到新结构体中，`user1` 就不再完整，无法再整体使用。

------

### 2. 代码示例演示

假设我们有如下结构体：

```rust
struct User {
    active: bool,      // 实现了 Copy
    username: String,  // 没有实现 Copy (所有权类型)
    email: String,     // 没有实现 Copy (所有权类型)
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };

    // 使用更新语法创建 user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 这里会发生什么？
    };

    // 情况分析：
    // 1. user1.email 被新值覆盖了，所以没被借走。
    // 2. user1.active 是 bool，执行了 Copy，user1.active 还能用。
    // 3. user1.username 被 Move 到了 user2！
    
    // println!("{}", user1.username); // ❌ 报错！value borrowed here after move
    println!("{}", user1.active);     // ✅ 正常！因为是 Copy 类型
}
```

### 3. `user1` 权限的变化总结

当你执行 `let user2 = User { ..user1 };` 后：

1. **整体失效**：一旦有任何一个非 `Copy` 类型的字段被移出，你便不能再把 `user1` 作为一个完整的变量使用（例如不能传给函数 `display(user1)`）。
2. **部分可用**：你依然可以访问 `user1` 中那些**没有被移出**（即被 `user2` 显式覆盖了）或者**属于 `Copy` 类型**的字段。
3. **部分解构**：这种情况在 Rust 中被称为“部分移动”（Partial Move）。

### 4. 只有一种情况 `user1` 会完全没事

如果 `User` 结构体中**所有的字段**都实现了 `Copy` 特性，那么 `..user1` 之后，`user1` 依然完全有效，没有任何变化。



---

* 字符没有名

  ```rust
  struct Color(i32,i32,i32);
  
  fn main(){
  	let black = Color(0,0,0);
  }
  ```

  

* 无字段的Struct

  ```rust
  struct AlwaysEqual;
  
  fn main(){
  	let subject = AlwaysEqual;
  }
  ```


* 如果想直接打印结构体,可以在一切之前加一段`#[derive(debug)]`,之后便可以用`println!({:?})`或者`println!({:#?})`打印

* 可以用`impl`关键字为结构体添加方法

  * ```rust
    impl Struct_name {
    	fn area(&self) -> u32{
    		self.width * self.height
    	}
    }
    ```

* 关于Display和Debug的区别?

  * ```rust
    use std::fmt;
    
    // 自动派生 Debug
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    // 手动实现 Display
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "坐标点({}, {})", self.x, self.y)
        }
    }
    
    fn main() {
        let p = Point { x: 10, y: 20 };
        
        println!("Debug 输出: {:?}", p);   // Point { x: 10, y: 20 }
        println!("Display 输出: {}", p);   // 坐标点(10, 20)
    }
    ```

    