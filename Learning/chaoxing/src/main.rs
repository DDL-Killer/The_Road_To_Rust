// use std::env;
//
// fn main() {
//     // 1. 获取命令行参数并收集到向量中
//     let args: Vec<String> = env::args().collect();
//
//     // 2. 检查参数数量（程序名 + 2个参数 = 3）
//     if args.len() < 3 {
//         println!("用法: cargo run <数字1> <数字2>");
//         return;
//     }
//
//     // 3. 将字符串参数解析为 i32 类型
//     // .parse().expect() 用于简单处理可能的输入错误
//     let num1: i32 = args[1].parse().expect("请输入有效的整数");
//     let num2: i32 = args[2].parse().expect("请输入有效的整数");
//
//     // 4. 调用逻辑并解构元组
//     let (sum, prod) = calculate(num1, num2);
//
//     // 5. 验证与输出
//     assert_eq!((sum, prod), (num1 + num2, num1 * num2));
//
//     println!("输入: {} 和 {}", num1, num2);
//     println!("求和: {}, 乘积: {}", sum, prod);
//     println!("验证成功！✅");
// }
//
// fn calculate(a: i32, b: i32) -> (i32, i32) {
//     (a + b, a * b)
// }

// fn main() {
//     // 1. 定义长度为 5 的 u8 数组
//     let arr: [u8; 5] = [10, 20, 30, 40, 50];
//
//     // 2. 通过切片获取第 2 到第 4 个元素
//     // 注意：Rust 索引从 0 开始。
//     // 第 2 到第 4 个元素对应的索引是 1, 2, 3。
//     // 范围选择使用 [开始索引..结束索引]，结束索引是不包含在内的（左闭右开）。
//     // 所以 [1..4] 包含索引 1, 2, 3。
//     let slice: &[u8] = &arr[1..4];
//
//     // 打印一下切片内容确认：应该是 [20, 30, 40]
//     println!("切片内容: {:?}", slice);
//
//     // 3. 计算切片中所有元素的平均值
//     // 使用 .iter() 迭代并求和
//     // 因为是 u8，求和可能会溢出，实际开发中建议先转为 u32
//     let sum: u32 = slice.iter().map(|&x| x as u32).sum();
//     let count = slice.len() as u32;
//
//     // 计算平均值（保留整数）
//     let average = sum / count;
//
//     // 4. 打印结果
//     println!("切片的平均值 (20+30+40)/3 = {}", average);
//
//     // 验证结果
//     assert_eq!(average, 30);
// }

// fn main() {
//     // 1. 创建一个空的 Vec<i32>
//     // 必须声明为 mut，因为后面需要修改它
//     let mut vec = Vec::new();
//
//     // 2. 依次追加 5, 8, 2, 9
//     vec.push(5);
//     vec.push(8);
//     vec.push(2);
//     vec.push(9);
//
//     // 3. 修改索引为 2 的元素为 7
//     // 索引 2 对应的是原本的数字 2
//     if let Some(elem) = vec.get_mut(2) {
//         *elem = 7;
//     }
//     // 或者直接使用下标访问（如果不担心索引越界的话）：
//     // vec[2] = 7;
//
//     // 4. 扩展 Vec，添加数组 [1, 3] 的所有元素
//     let arr = [1, 3];
//     // extend 接收一个迭代器，它会把迭代器里的元素逐个追加到末尾
//     vec.extend(arr.iter());
//
//     // 5. 验证 Vec 最终内容
//     println!("最终 Vec 内容: {:?}", vec);
//     assert_eq!(vec, vec![5, 8, 7, 9, 1, 3]);
//
//     println!("所有验证已通过！✨");
// }

// fn main() {
//     // 1. 定义测试输入
//     let input1 = "RustLang";
//     let input2 = "hello";
//
//     // 执行处理逻辑
//     let result1 = process_string(input1);
//     let result2 = process_string(input2);
//
//     // 2. 使用 assert_eq! 验证结果
//     assert_eq!(result1, "RUSTLANG");
//     assert_eq!(result2, "hello_rust");
//
//     // 3. 打印结果
//     println!("输入: \"{}\" -> 处理结果: \"{}\"", input1, result1);
//     println!("输入: \"{}\" -> 处理结果: \"{}\"", input2, result2);
// }
//
// fn process_string(s: &str) -> String {
//     // 判断逻辑：转小写后是否以 "rust" 开头
//     // 注意：to_lowercase() 会生成一个新的 String
//     if s.to_lowercase().starts_with("rust") {
//         // 满足条件：转为全大写
//         s.to_uppercase()
//     } else {
//         // 不满足条件：拼接 "_rust"
//         // 方式一：使用 format! 宏（最直观）
//         format!("{}_rust", s)
//         // 方式二：也可以使用 push_str
//         // let mut new_s = s.to_string();
//         // new_s.push_str("_rust");
//         // new_s
//     }
// }

fn main() {
    // 1. 定义变量
    let weight: f64 = 5.5; // 商品重量
    let is_vip: bool = true; // 是否为 VIP

    // 2. 使用嵌套表达式直接计算最终运费
    let final_fee = {
        // 首先计算基础运费
        let base_fee = if weight <= 1.0 {
            10.0
        } else if weight <= 5.0 {
            20.0
        } else {
            30.0
        };

        // 根据是否为 VIP 应用折扣并作为块的返回值
        if is_vip {
            base_fee * 0.8
        } else {
            base_fee
        }
    };

    // 3. 打印结果
    println!("商品重量: {} kg", weight);
    println!("VIP 身份: {}", if is_vip { "是" } else { "否" });
    println!("最终计算运费为: {:.2} 元", final_fee);
}