# Trait

一个Trait定义了特定类型所具有的功能



可以使用Trait以一种抽象的方式来定义共享的行为



可以使用Trait Bounds来指定哪些类型才是我们想要的泛型类型

## 定义Trait

类型的行为:可在该类型上调用的方法



Trait定义:将不同的方法签名毁成一个方法签名

​	由此定义一套共享的方法

## Trait的实现规则

只要trati或类型其中之一属于当前crate,就可以实现该trait



一致性和孤儿规则

孤儿规则要求trait或类型必须属于当前crate,防止冲突实现

避免多个crate为同一类型实现相同trait导致的歧义,保证代码稳定性

## 默认实现

```rust
pub trait Summary{
	fn summarize(&self) -> String{
		String::from("(Read more...)")
	}
}
```

