#[derive(Debug)]
struct Dog {
    //定义结构体
    name: String,
    weight: f32,
    height: f32,
}

//定义方法
impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn show() {
        println!(" oh oh oh  oh");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.0,
        height: 90.9,
    };
    println!(" dog ={:#?}", dog);

    println!(" name ={} ", dog.get_name());


    Dog::show();
}
