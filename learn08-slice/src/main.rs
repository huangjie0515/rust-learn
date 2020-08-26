
//1.字符串slice是String中一部分值的引用
//2.字面值就是slice
//3.其他类型slice
fn main() {
    let s=String::from("hello world!!!");
    let h1=&s[0..5];//包含左边界 ,不含右边界
    let h2=&s[..5]; //从0开始
    let h3=&s[..=4];
    let h4=&s[6..];
    let h5=&s[..];
    println!(" h1 = {}",h1);
    println!(" h2 = {}",h2);
    println!(" h3 = {}",h3);
    println!(" h4 = {}",h4);
    println!(" h5 = {}",h5);
    let ss=String::from("hello");
    let s1=&ss[0..1];
    println!("---- {}  ----",s1);
}
