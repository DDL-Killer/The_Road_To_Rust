# 错误处理

* 可恢复的
* 不可恢复的->bug
* rust没有"异常"

## 不可恢复的错误:panic!()

两种导致panic的方法:

1. 代码中的某些行为导致panic
2. 显示的调用panic!()宏

默认情况下:

panic后,会打印失败信息,展开Stack,清理Stack

* panic后的相应
  * 展开Stack并清理数据
  * 立即终止

* Backtrace
  * 到达某个点之前所调用的所有函数的列表
  * 在arch里要看需要`RUST_BACKTARCE = full/非零 cargo run`

## 使用Result处理可恢复的错误

```rust
enum Result<T,E>{
	OK(T),
	Err(E),
}
```

## 出现错误时使程序产生恐慌的常用快捷方式

unwrap()

用于提起Option或Result类型内部的值

如果是None或Err,程序将panic并终止



expect()

与unwrap()类型,允许提供一个自定义的panic消失

## 传播错误

将错误返回,将调用该函数的代码来决定如何处理错误

## ?运算符

使用?运算符时

​	如果操作成功,它会解包Ok并继续执行下一行代码

​	如果操作失败,它会立刻返回Err,并将错误传播给调用者



使用?运算符可以避免大量的match或if let 语句,使代码更简洁



from函数:

​	把值从一个类型转化到另一种类型

### 什么时候使用?

函数的返回类型与?所作用的值的类型兼容



?可用于返回类型为Result Option或者实现了FomResidual的类型的函数内

## 什么时候Panic,什么时候不Panic

* 何时使用?

​	不可恢复的错误场景

* 何时使用Result

   提供恢复的错误场景

* 推荐使用Panic
  * 原型代码和示例
  * 测试代码
  * 安全性关键的输入验证
  * 调用外部不可控代码时的异常状态
* 推荐使用Result
  * 处理可预期的错误
  * HTTP请求失败
  * 解析错误
  * 用户输入验证

# 泛型

Rust中用于消除重复的工具之一

## 泛型数据类型

可使用泛型数据类型来定义函数或struct

### 在函数的定义中使用泛型

```rust
fn largest<T>(list:&[T])->&T{}
```

需要指明T的限制条件(具备什么能力)

### 在Struct定义中使用泛型

```rust
struct Point<T>{
	x:T,
	y:T,
}
```

可使用多个泛型类型参数

```rust
struct Point<T,U>{
	x:T,
	y:U,
}
```

泛型参数太多,可能意味着重构

### 在Enum定义中使用泛型

```rust
enum Option<T>{
	Some(T),
	None,
}
```

### 在方法中使用泛型

 

```rust
struct Point<T>{
	x:T,
	y:T,
}

impl<T> Point<T>{
	fn x(&self) -> &T{
        &self.x
    }	
} 
```

* Rust里面没有继承机制

## 使用泛型代码的性能

* 在Rust中使用泛型类型不会比使用具体类型让程序运行的更慢
* Rust通过单态化在编译时实现这种效率
  * 单态化是编译器将泛型代码转化为具体代码的过程

