//1.option是标准定义的一个枚举,形式:
//enum Option<T{
//Some(T),
//None,
// }

//2.使用方式
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("hello world"));

    let absent_number: Option<i32> = None;
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y {
        Some(i) => {
            temp = i;
        }
        None => {
            println!("do nothing")
        }
    };

    let sum = x + temp;  //无法使用,不是同一个类型
    println!("Hello, world!");
}
