//1.rust通过所有权机制来管理内存,编译器在编译就会根据所有权规则对内存的使用机型检查



//2.堆和栈
//堆上:变量   //栈上:临时变量
//编译的时候数据的类型大小是固定的,就是分配在栈上的
//编译的时候数据类型大小不固定的,就是分配在堆上  (字符串大小不固定)


//3.作用域
//代码块变量作用域旨在代码块里面



//4.String内存回收


//5.移动


//6.clone


//7.栈上数据拷贝

//8.函数和作用域

fn main() {
    println!("Hello, world!");

    let x:i32=1;
    {
        let y:i32=1;  //局部变量  当前作用域有效
    }
    println!( "x = {}",x);
    let s2="11111111111111";
    {
        //字符串类型会放在堆上,编译时不知道字符串的大小
        let mut s1 =String::from("hello");
        s1.push_str(" world");
        println!(" s1 ==== {}",s1);
    }


    {
        let x=1;
        //基本类型的变量放在栈上,允许赋值给其他变量
        let y=x;
        println!(" y =   {}",y);
    }

    {
        //字符串存储在堆上
        //String类型离开作用域的时候会调用drop方法
        let x=String::from("123");
        let y=x;
        println!(" y =  {}",y);

        let  h1=String::from("hello");
        let  h2=h1; //h1已经释放过内存
        //println!(" h1 = {}",h1);   //value borrowed here after move   移动到h2,h1就无效
        println!(" h2 = {}",h2)
    }

    {
        let h1=String::from("hello");
        let h2 =h1.clone();
        println!(" h2 = {}",h2);
        println!(" h1 = {}",h1);
    }
    {
        let a="111";
        let b="222";
    }


    let a=1;
    //栈上拷贝
    let b=a;
    println!(" a = {}",a);
    println!(" b = {}",b);
    {
        let s1=String::from("11");
        let s2=s1;
        //move to s2,s1 invalid
        println!(" s2 ={}",s2);

        let s1="111";
        let s2=s1;
        println!(" s1 = {}",s1);
        println!(" s2 = {}",s2);
        //clone
        //浅拷贝
        let mut  s3=s2.clone();
        s3="11111111111111";
        println!(" s2 = {}",s2);
        println!(" s3= {}",s3);

        //copy
        //基本类型都是放在栈上,栈上都可以拷贝;
        let a=1;
        let b=a;
        println!(" a ={}",a);

    }
    //常用的具有copy trait有:
    //所有的整型
    //浮点型
    //字符类型
    //元组




}
