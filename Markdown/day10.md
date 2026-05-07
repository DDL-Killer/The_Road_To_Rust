# HashMap<K,V>

存储键值对(K,V)的映射的集合

Hashing函数

通过K来查找

## 创建HashMap

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);
```

同质:

Key类型必须相同

Value类型必须相同

## 访问HashMap里面的值,使用get方法

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

## HashMap与所有权

* 对于实现Copy trait的值,直接复制到map里
* 对于"具有所有权"的类型的值,"移动"到map里

## 更新HashMap

## Hashing函数

默认使用SipHash,相对安全,并不是最快的

可通过指定hasher来切换Hashing函数
