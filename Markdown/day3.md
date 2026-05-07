## 变量 & 可变性

* 声明变量

  `let some_number = 1;`

​	默认:变量是不可变的

* 声明可变变量

  `let mut another_number = 2;`

## 常量

* 声明常量

  ```rust
  const SPECIAL_NUMBER:i32 = 3;
  const THREE_HOURS_IN_SECONDS:u32 = 60*60*3;
  ```

* 使用`const`声明
* 不可以使用`mut`
* 必须标注类型
* 可在任意作用域声明
* 仅可使用常量表达式赋值

## 变量遮蔽

```rust
let my_number = 1;
let my_number = 2;
```

* 可以用与之前变量相同的名字声明一个新变量:第一个变量被第二个变量"遮蔽"

* 就是创建了一个新的变量,只不过名字相同

## 数据类型

* Scalar标量类型:表示一个单一的值
* Compound复合类型:可以将多个值组合在一个类型

### Scalar标量类型

* Integer整数
* Floating Point浮点
* Boolean布尔
* Character字符

#### 整数类型Integer

| Length  | Signed(有符号) | Unsigned(无符号) |
| ------- | -------------- | ---------------- |
| 8-bit   | i8             | u8               |
| 16-bit  | i16            | u16              |
| 32-bit  | i32            | u32              |
| 64-bit  | i64            | u64              |
| 128-bit | i128           | u128             |
| arch    | isize          | usize            |

#### 整型字面值Integer Literals

| Number literals   | 示例        |
| ----------------- | ----------- |
| Decimal十进制     | 98_222      |
| Hex十六进制       | 0xff        |
| Octal八进制       | 0o77        |
| Binary二进制      | 0b1111_0000 |
| Byte(u8 only)字节 | b'A'        |

#### 浮点类型Floating Point

* f32:4字节
* f64(默认):8字节
* 都有符号(signed)

#### 布尔类型Boolean

* 两个值:true,false
* 1字节

#### 字符类型Character

`let some_char:char = 'A'`

* 4字节
* 表示一个Unicode标量值

### Compound Type

* Tuple元组
* Array数组

#### Tuple元组

```rust
let my_tuple = ('A',1,1.2);
let tup:(i32,f64,u8) = (500,16.4,1);
let five_hundred = tup.0; //访问元组里面的元素
```

* 固定长度
* 可含不同类型的元素

#### Array数组

```rust
let my_arr = [1,2,3];
let my_arr_typed:[i32;3] = [1,2,3];
let a = [3;5]; //let a = [3,3,3,3,3];
let first = my_arr[0];
```

* 固定长度

* 元素类型相同

  
