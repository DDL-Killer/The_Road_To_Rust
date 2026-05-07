## 控制流

### IF表达式

```rust
fn main(){
	let number = 3;
	if number < 5{
		println!("Condition was true");
	}else{
		println!("Condition was false");
	}
}
```

### 循环

* loop
* while
* for

#### loop

```rust
fn main(){
	loop{
		println!("again!");
	}
}
```

break:停止loop

continue:跳出本次迭代



从loop返回值:break表达式



可以用loop标签(label)结束外层循环

#### while

```rust
fn main(){
	let mut number = 3;
	
	while number != 0{
		println!("{number}!");
		
		number -= 1;
	}
	println!("LIFTOFF!");
}
```

#### for

```rust
fn main(){
	let a = [10,20,30,40,50];
	
	for element in a{
		println!("the value is: {element}");
	} 
}
```

