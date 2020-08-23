
const   constant:u32=10;

fn main() {
    //1.变量定义
    let a = 1;
    println!("a = {}",a);

    //指定类型u32
    let b:u32=2;
    println!("b = {}",b);

    //不可变变量不可被修改
    //b =1;  //cannot assign twice to immutable variable   不可变变量不能分配两次

    //可变变量  mut
    let mut c:u32=1;
    println!("修改前 c={}",c);
    c=2;
    println!("输出   c={}",c);

    //可变变量  可以改变类型
    let  c:f32=1.1;
    println!("c = {}",c);

    println!("constant  = {}",constant);
    println!("Hello, world!");
    println!("德玛西亚")
}
