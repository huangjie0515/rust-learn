fn main() {
    //1.定义结构体
    #[derive(Debug)]
    struct User {
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }
    //2.创建结构体实例
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("870000"),
        nonce: 1000,
        active: true,
    };
    //3.修改结构体字段
    let mut xiaohuang = User {
        name: String::from("xiaohuang"),
        count: String::from("870000"),
        nonce: 1000,
        active: true,
    };
    xiaohuang.nonce = 20000;

    //4.参数名字和字段名字同名的简写方法
    let name = String::from("xiao1");
    let count = String::from("1111");
    let nonce = 20099;
    let active = false;
    /*let user1 = User {
        name: name,
        count: count,
        nonce: nonce,
        active: active,
    };*/
    let user2 = User {
        name,
        count,
        nonce,
        active,
    };
    //5.从其他结构体创建实例
    let user3 = User {
        ..user2
    };
    println!("name = {}", user3.name);
    println!(" nonce = {}", user3.nonce);


    //6.元组结构体    1.字段没有名字   2.圆括号
    struct Point(i32, i32);
    let a = Point(10, 100);
    let b = Point(40, 399);
    println!(" a.x = {}, a.y = {}", a.0, a.1);


    //7.没有任何字段的类单元结构体
    struct A {}
    ;

    //8.打印结构体
    println!(" xiaoming ={:?}", xiaoming);
    println!(" xiaoming ={:#?}", xiaoming);


    println!("Hello, world!");
}
