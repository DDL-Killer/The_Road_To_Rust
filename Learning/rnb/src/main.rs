

fn main() {
    //     let m1 = String::from("Hello");
    //     let m2 = String::from("world");
    //
    //     greet(m1,m2);
    //     let s = format("{} {}",m1,m2);
    // }

    // let x = Box::new(1);
    // let y = x.clone();
    // println!("x:{}",x);
    // println!("y:{}",y);
    // println!("x:{:p}",x);
    // println!("y:{:p}",y);
    // println!("y:{:p}",&y);
    // println!("x:{:p}",&x);
        let mut x = 5;          // 原始数据是 mut 的

        let y = &mut x;         // y 是 x 的一个可变“别名”（借用）

        // 此时，如果你尝试通过原始变量 x 来修改或访问数据：
        // x = 6;              // ❌ 错误！x 已经被 y 借走了

        *y += 1;                // ✅ 正常！通过别名修改数据

        println!("{}", x);      // ✅ 正常！y 的生命周期在此之前结束（或在此之后不再使用）
    
}//
// fn greet(g1:String,g2:String){
//     println!("{} {}",g1,g2);
// }
