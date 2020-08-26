
//1.字符串slice是String中一部分值的引用
//2.字面值就是slice
//3.其他类型slice
fn main() {
    let s=String::from("hello world!!!");
    let h1=&s[0..5];
    let h1=&s[0..5];
    println!(" h = {}",h1);
    println!("Hello, world!");
    //slice
}
