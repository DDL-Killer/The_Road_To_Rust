fn main() {
    // let number = 6;
    //
    // if number % 4 == 0{
    //     println!("4 here");
    // }else if number % 3 == 0{
    //     println!("3 here");
    // }else if number % 2 == 0{
    //     println!("2 here");
    // }else{
    //     println!("no");
    // }

    // let condition = true;
    //
    // let number = if condition{5} else{6};
    //
    // println!("The value of number is:{number}");

    // let x = 1;
    // let y = if x {0} else {1};
    // println!("{y}");

    // let mut counter = 0;
    // let result = loop{
    //     counter += 1;
    // 
    //     if counter == 10{
    //         break counter *2;
    //     }
    // };
    // println!("The result is {result}");
    
    // let mut count = 0;
    //
    // 'end:loop{
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop{
    //         println!("remaining = {remaining}");
    //         if remaining == 9{
    //             break;
    //         }
    //         if count == 2{
    //             break 'end;
    //         }
    //         remaining -=1;
    //     }
    //     count+=1;
    // }
    // println!("End count = {count}");

    let a = [5;10];
    let mut sum = 0;
    for x in a{
        sum+=x;
    }
    println!("{sum}");
}
