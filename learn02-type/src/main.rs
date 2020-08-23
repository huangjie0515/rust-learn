fn main() {
    println!("Hello, world!");
    //基础类型

    //bool
    let is_true: bool = false;
    let is_false: bool = true;
    println!("is_true =  {}", is_true);
    println!("is_true =  {}", is_false);

    //char 在rust里面,char是32位的
    let a: char = 'a';
    let b: char = '汉';
    println!("a = {}", a);
    println!("b = {}", b);

    //i8,i16, i32,i64,u8,u16,u32,u64
    let c: i8 = -11;
    let d: i16 = 10;

    //浮点数里类型
    let e: f32 = 10.0;
    println!("c  {}", c);
    println!("d  {}", d);
    println!("e  {}", e);

    //自适应类型isize,usize
    println!("max = {}", usize::max_value()); //18446744073709551615

    //数组
    //[Type;size]    size也是数据类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]={}", arr[0]);
    println!("arr[1]={}", arr[1]);
    let test:[u32;3]=[1,2,3];
    show(test);


    //元组  (i32,u32)
    let tup:(i32,f32,char)=(-3,3.69,'好');

    let tup:(u32,f64,u32)=(110,11.11,11);
    println!("tup = {}",tup.0);//第一个元素
    println!("tup = {}",tup.1);//第二个元素

    //将元组拆开
    let (x,y,z)=tup;
    println!("x = {}",x);

}

fn show(arr: [u32; 3]) {
    println!("开始遍历");
    //遍历语法
    for  i  in  &arr{
        println!("{}",i);
    }
}

