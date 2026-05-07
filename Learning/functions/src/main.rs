fn main() {
    println!("Hello, world!");

    //another_functions();
    another_functions2(5,'h');
}

fn another_functions(){
    println!("Another functions.");
}

fn another_functions2(value:i32,unit_label:char){
    println!("The measurement is:{value}{unit_label}");
}