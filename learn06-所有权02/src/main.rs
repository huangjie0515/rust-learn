fn takes_ownership(some_string: String)->String{
    println!(" {} ",some_string);
    some_string
}
fn makes_copy(i:i32){
    println!("i = {}",i);
}

fn main() {
   let str=String::from("hello");
   let  str1=takes_ownership(str);
   //离开作用域无法使用
    println!(" str1 = {}",str1);
    let x=5;
    makes_copy(x);
    println!(" x = {}",x);
}
