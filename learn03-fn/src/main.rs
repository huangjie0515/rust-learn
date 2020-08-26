//无参无返回值
fn other_fun(){
    println!("this is  a  function");
}

//有参无返回值
fn other_fun01(a:u32,b:u32){
    let c:u32=a+b;

    println!("c-----{}",c );
}
//有参有返回值
fn other_fun02(a:i32,b:i32) -> i32{
    return  a+b;
}


fn main() {
    other_fun();
    other_fun01(2,2);
    let result=other_fun02(1,2);
    println!("Hello, world!");

    //语句是执行一些操作,但是不返回值的指令
    let y=1;  //语句,不返回值
    let z={
        let x=1;
        x+1
    };

}
