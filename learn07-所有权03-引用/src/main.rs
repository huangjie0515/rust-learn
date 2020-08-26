fn main() {
    let s1 = gives_ownership();
    let mut s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!(" s1 = {} ", s1);
    let mut s2 = "11";
    //s2无法使用;
    println!(" s2 = {}", s2);

    println!(" hello world !");

    {
        //引用
        let s1 = String::from("hello");
        //引用:用法&
        //让我们创建一个指向值的应用,但是并不拥有她,因为不拥有这个值,所以,当引用离开其值指向的作用域后也不会丢弃.
        let s: usize = calute_length(&s1);
        println!(" s1 = {}", s1);


        let s1 = String::from("world ");
        //将s1的引用传递给s2
        let s2 = &s1;
        calute_length(s2);

        let mut s1 = String::from("111");
        modify_s(&mut s1);
        println!(" s1 ==  {}", s1);
    }
    {
        let mut s1 = String::from("jk");
        let r1 = &s1;
        let r2 = &s1;
        println!(" {} ={}", r1, r2);
        let r3 = &mut s1;
        r3.push_str(" hello world ");
        //println!(" {} ={}", r1, r2);  //r1  r2无法使用
    }
    {
        //let  mut   ref_s = dangele();
        //println!(" hello world !!!");
        //1.在任意给定时间,有了可变引用之后不能再有不可变引用
        //2.引用必须有效
    }
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(some: String) -> String {
    some
}

//引用
fn calute_length(s: &String) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(",,,,world");
}

fn dangele() -> &String {
    //lifetime
    let s = String::from("hello");
    &s
}