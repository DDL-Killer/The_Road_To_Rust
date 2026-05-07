//数位立方和
// 本题目要求对于输入的一个三位整数 n ，输出 n 中三个数字立方的和。
//
// 输入格式:
// 输入在一行中给出 1 个三位整数 n 。
//
// 输出格式:
// 输出 n 中三个数字立方的和

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//
//     io::stdin().read_line(&mut input).unwrap();
//
//     let n:i32 = input.trim().parse().unwrap();
//
//     let hundreds = n/100; //百位,100整除
//     let tens = (n/10)%10; //十位,除10保留前两位,10取余
//     let units = n % 10; //个位,10取余
//
//     let sum = hundreds.pow(3)+tens.pow(3)+units.pow(3);
//     println!("{}",sum);
// }


//7-2 计算摄氏温度
// 分数 15
// 作者 李梅莲
// 单位 许昌学院
// 本题要求编写程序，计算华氏温度对应的摄氏温度。计算公式：C=5×(F−32)/9，式中：C表示摄氏温度，F表示华氏温度，输入输出数据要求为整型。
//
// 输入格式:
// 输入一个不超出1000的整数，作为华氏温度
//
// 输出格式:
// 按照下列格式输出
// fahr = 输入的华氏温度,  celsius = 计算所得摄氏温度的整数值
//
// use std::io;
//
// fn main(){
//     let mut input = String::new();
//
//     io::stdin().read_line(&mut input).unwrap();
//
//     let n:i32 = input.trim().parse().unwrap();
//
//     let f:i32 = 5*(n-32)/9;
//
//     println!("fahr = {}, celsius = {}",n,f);
// }

// 7-3 态度决定一切
// 分数 10
// 作者 zjq
// 单位 上海大学
// 将英文26个字母A~Z，或a~z对应到整数1~26，则态度 Attitude 对应的数字之和为100。编程计算给定的字符串对应的数字之和（规定非英文字母的字符均对应0）。
//
// 输入格式:
// 输入数据的第一行为一个整数n，其后共有n行字符串（字符串的长度不超过1000）分别表示n种情形。
//
// 输出格式:
// 对于每一种情况，要求输出原字符串、空格、等号、空格、结果、换行。

// use std::io::{self,BufRead};
//
// fn main(){
//     let stdin = io::stdin();
//
//     let mut lines = stdin.lock().lines();
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//
//     io::stdin().read_line(&mut input).unwrap();
//
//     let income:f64 = input.trim().parse().unwrap_or(0.0);
//
//     let mut tax:f64 = 0.0;
//
//     if income <=1.0{
//
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let parts:Vec<&str> = input.split_whitespace().collect();
//
//     if parts.len() < 3{
//         return;
//     }
//
//     let a :i32 = parts[0].parse().unwrap();
//     let b :i32 = parts[1].parse().unwrap();
//     let op = parts[2];
//
//     match op{
//         "+" => println!("{}",a+b),
//         _=>println!(" "),
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let nums:Vec<i32> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();
//
//     if nums.len()<2{
//         return;
//     }
//
//     let start = nums[0].min(nums[1]);
//     let end = nums[0].max(nums[1]);
//
//     let mut found = false;
//
//     for n in start..=end{
//         if n>=100 && n<=999{
//             let a = n/100;
//             let b = (n/10)%10;
//             let c = n % 10;
//
//             if a.pow(3)+b.pow(3)+c.pow(3) == n{
//                 println!("{} ",n);
//                 found = true;
//             }
//         }
//     }
//     if !found{
//         println!("No Answer");
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let s = input.trim();
//     let reverse:String = s.chars().rev().collect();
//
//     let num:i32 = reverse.parse().unwrap();
//     println!("{}",num*2);
// }

// use std::io;
//
// fn get_sum(mut n:u64)->u64{
//     let mut sum = 0;
//     while n > 0{
//         sum+= n%10;
//         n/=10;
//     }
//     sum
// }
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let n :usize = input.trim().parse().unwrap();
//
//     for _ in 0..n{
//         let mut num_str = String::new();
//         io::stdin().read_line(&mut num_str).unwrap();
//         let num:u64 = match num_str.trim().parse(){
//             Ok(v) => v,
//             Err(_) => continue,
//         };
//
//         let origin = get_sum(num);
//         let mut really = true;
//
//         for i in 2..=9{
//             let pro = num *i;
//             if get_sum(pro) != origin{
//                 really = false;
//                 break;
//             }
//         }
//
//         if really{
//             println!("{}",origin);
//         }else{
//             println!("No");
//         }
//
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let n:usize = input.trim().parse().unwrap();
//
//     if n == 0{
//         println!("0 0 0");
//         return;
//     }
//
//     let mut scorse  = String::new();
//     io::stdin().read_line(&mut scorse).unwrap();
//
//     let grade:Vec<i32> = scorse.split_whitespace().filter_map(|s| s.parse().ok()).collect();
//     let mut excellent = 0;
//     let mut middle = 0;
//     let mut low = 0;
//     for s in grade{
//         if s >= 85{
//             excellent+=1;
//         }else if s >= 60{
//             middle+=1;
//         }else{
//             low +=1;
//         }
//         println!("{} {} {}",excellent,middle,low);
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let n = input.trim();
//     let num:u64 = n.parse().unwrap();
//
//     if num == 0{
//         println!("0");
//     }else{
//         let result = 1 + (num-1)%9;
//         println!("{}",result);
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     for word in input.split_whitespace(){
//         if let Ok(n)=word.parse::<u32>(){
//             println!("{}",(n as u16).count_ones());
//         }
//     }
// }

// use std::io;
//
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     for word in input.split_whitespace(){
//         if let Ok(n) = word.parse::<i64>(){
//             let abs = n.abs();
//             if abs == 0{
//                 println!("No");
//                 continue;
//             }
//             if abs.count_ones() == 1{
//                 println!("{}",abs.trailing_zeros());
//             }else{
//                 println!("No");
//             }
//         }
//     }
// }

// use std::io;
// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//
//     let mut lter = input.split_whitespace();
//
//     let m : f64 = lter.next().unwrap_or("0").parse().unwrap();
//     let n : f64 = lter.next().unwrap_or("0").parse().unwrap();
//     if n == 0.0{return ;}
//
//     let mut pass = 0;
//     let mut good = 0;
//
//     for _ in 0..(n as usize){
//         if let Some(str) = lter.next(){
//             if let Ok(score) = str.parse::<f64>(){
//                 if score >= m*0.6{
//                     pass+=1;
//                 }
//                 if score >= m*0.8{
//                     good+=1;
//                 }
//             }
//         }
//     }
//     let psrate= (pass as f64 / n) *100.0;
//     let gdrate = (good as f64 / n) * 100.0;
//     let okkk = psrate>=75.0 && gdrate >=25.0;
//     let res = if okkk{"Yes"}else{"No"};
//     println!("{} {:.2}% {:.2}%",res,psrate,gdrate);
// }
fn main(){

}

fn isperfec (num:i32,cnt:&mut i32) -> bool{
    if num <= 1{
        *cnt == 0;
        return false;
    }

    let mut sum =0;
    let mut local_cnt= 0;

    for i in 1..num{
        if num % i == 0{
            sum+=i;
            local_cnt+1;
        }
    }

    *cnt = local_cnt;
    sum == num
}

fn longest<'a>(x: &'a str ,y:&'a str)->&'a str{
    if x.len()>=y.len(){
        x
    }else{
        y
    }
}

fn is_prime(num:i32,cnt:&mut i32)->bool{
    *cnt = 0;

    if num <= 1{
        return false;
    }

    for i in 2..num{
        if num % i == 0{
            *cnt += 1;
        }
    }
    *cnt ==0
}

fn factorial(n:u32,call_count :&mut u32) -> u32{
    *call_count+=1;

    if n == 0{
        1
    }else{
        n * factorial(n-1,call_count)
    }
}

fn count_vowels(s:&str ,consonants :&mut u32) -> u32{
    let mut vow = 0;
    let mut con = 0;

    for c in s.chars(){
        if c.is_ascii_alphabetic(){
            let low = c.to_ascii_lowercase();
            match low{
                'a'|'e'|'i'|'o'|'u'=>{
                    vow+=1;
                }
                _ =>{
                    con+1;
                }
            }
        }
    }
    *consonants = con;
    vow
}