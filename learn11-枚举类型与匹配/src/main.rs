fn main() {
    //1.类型于c语言的方法定义
    enum IpAddKind {
        V4,
        V6,
    }
    struct IpAddress {
        kind: IpAddKind,
        address: String,
    }
    let i1 = IpAddress {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    //2.rust语言提倡的方法定义
    enum IpAddress2 {
        v4(String),
        v6(String),
    }
    let i2 = IpAddress2::v4(String::from("127.0.0.1"));
    let i3 = IpAddress2::v6(String::from("127.0.0.1"));

    //3.可以是不同类型
    enum IpAddress3 {
        v3(u8, u8, u8, u8),
        v5(String),
    }
    let i4 = IpAddress3::v3(127, 0, 0, 1);
    let i6 = IpAddress3::v5(String::from("127.0.0.1"));

    //4.经典用法
    enum Message {
        Quit,
        //类单元结构体
        Move { x: i32, y: i32 },
        //类型
        Write(String),
        //元组
        Change(i32, i32, i32), //元组
    }
    ;


    println!("Hello, world!");
}
