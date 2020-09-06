//1.HashMap<k,v>
//2.创建HashMap
//3.读取
//4.遍历
//5.更新

//导入包
use std::collections:: HashMap;

fn main() {
    //语法 1.HashMap<k,v>


    //2.创建HashMap
    let mut  scores: HashMap<String,i32>=HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("red"),20);

    let keys=vec![String::from("Blue"),String::from("Red")];
    let values=vec![10,20];
    //将两个vec合并成HashMap
    let scores: HashMap<_,_>=keys.iter().zip(values.iter()).collect();


    //3.读取 需要考虑是否为null
    let k=String::from("Blue");
    //option语法  HashMap get会返回option
    if   let Some(v)=scores.get(&k){
        println!("v = {}",v);
    }

    let k1=String::from("test");
    let v1=scores.get(&k1);
    match v1 {
        //如果不为空的话
        Some(value)=>println!("v1 = {}",value),
        //如果为空的话
        None => println!("当前为空"),
    }
    println!("Hello, world!");
}
