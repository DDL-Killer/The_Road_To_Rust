# Enum枚举

* 定义了一组可能的值

* 枚举里面可以存数据
* 也可以使用`impl`为枚举创建方法

## Option Enum

```rust
enum Option<T>{
	None,
	Some(T),
}
```

* 表示某个值可能存在或不存在
* Option比Null好在哪?
  1. 表示出某可能不存在
  2. Option<T>与T是不同类型
  3. 强迫你得处理这种情况

# match表达式

match表达式的可能性必须"穷尽"

如果不想修改权限,可以加上引用符号来防止所有权移动

# if let匹配一种情况

# 项目代码组织

## crate

crate是组织和共享代码的基本构建块

* binary crate:可执行的,需要main函数
* library crate:没有main函数,无法执行,定义一些功能,可共享使用

### crate root

编译crate的入口点(源代码文件)

* binary crate:src/main.rs
* library crate:src/lib.rs

## package



由一个或多个crates组成

* 包含Cargo.toml文件
* package的规则
  * 可有多个binary crates
  * 最多只能有一个library crate
  * 但至少有一个crate
* 使用`cargo new project_lib --lib`就可以创建lib

## Module

将代码组织成更小 更易管理的单元的方法

* 使用mod声明
* 可有子模块
* 路径
  * 绝对路径:从crate模块开始
  * 相对路径:从所在模块开始
* public vs private
  * 所有的东西默认对父模块私有
  * 父模块不能使用子模块的私有
  * 子模块可以使用祖先模块的项
  * 使用pub关键字可以让其变成public
  * 相对路径使用super self关键字
  * Strut和Enum
    * Struct:需要为本身和字段单独设置pub
    * Enum:只要其本身是pub,所有变体pub
* 引用
  * function:引用到父模块
  * struct enum:引用完整路径
  * 如果引用到项目同名,要么引用父级名称,要么用as起一个别名
* 一般把 mod 放在lib.rs里或者专门放在models.rs(与方法同名)里又或者创建一个目录(与方法同名)在里面放mod.rs又或者创建models目录,在里面放与方法同名的rs文件
* 如果想加第三方库
  1. 直接在Cargo.toml里面加
  2. `cargo add 库名称`

# 常见的集合类型

## Vectors

* 在单一数据结构存储多个值
* 在内存中连续存储(相邻)
* 元素必须是同类型
* Vec<T>来自标准库