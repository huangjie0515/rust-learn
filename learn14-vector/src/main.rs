//1.创建空的vector
//2.创建包含初始值的vector
//3.丢弃vector
//4.读取元素
//5.遍历
//6.使用枚举
fn main() {
    //1.mut 创建
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    //2.创建空的vector
    let v = vec![1, 2, 3, 4];

    //3.
    {
        let v1 = vec![1, 3, 4];
    }

    //4.
    let one: &i32 = &v[0];
    println!(" one ={}", one);

    println!(" one = {}", *one);


    match v.get(1) {
        Some(value) => println!(""),
        _ => {}
    };
    //4.2 推荐的方法
    match v.get(9) {
        Some(value) => println!(" value = {}", value),
        _ => println!("第九个none"),
    }

    //5.遍历
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //5.1不可变的遍历
    for i in &v2 {
        println!(" i = {}", i);
    }


    //5.2可变的遍历
    for i in &mut v2 {
        *i += 1;
        println!(" i ={}", i);
    }
    //打印详情
    println!(" v2 = {:#?}", v2);


    //6.枚举使用
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }
    ;
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(4.00),
    ];

    //补充
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];//不可变引用
    v.push(6);//mutable borrow occurs here
    //可变之后,不能用之前的不可变引用
    println!(" first = {}", first);

    println!("Hello, world!");
}
