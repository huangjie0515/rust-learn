//1.定义
//(1)包:cargo的一个功能,允许构建,测试和分享crate
//(2)Create:一个 模块的树形结构,形成库或二进制项目.
//(3)模块:通过use来使用,用来控制作用域和路径的私有性.
//(4)路径:一个命名例如结构体,函数或模块等项的方式.


//2.包和Crate
//(1)crate root是一个源文件,Rust编译器以它为起始点,并构成你的crate的根模块.
//(2)包提供一系列功能的一个或多个crate.
//(3)crate  root是src/main.rs或者是src/lib.rs.说明只有main.rs则说明这个包只有crate(main),如果同时拥有main.rs和其他的lib.rs
//(不一定是这个名字)则说明拥有多个crate.
//(4)crate会将一个作用域的相关功能分组到一起,使得该功能可以很方便的在多个项目之间共享.


















fn main() {
    println!("Hello, world!");









}
