//use  mylib::factory::produce_washing_machine;
use  mylib::factory::produce_washing_machine  as  A;
fn main() {
   //produce_washing_machine::produce_washing_machine();  //模块.方法名
   A::produce_washing_machine();


   mylib::factory::produce_refrigerator::produce_re();
   mylib::factory::produce_washing_machine::produce_washing_machine();

   util::util::jwt::createJwt();
   util::util::jwt::parseJwt();

   date::dateformat::first::date_format();
   date::dateformat::first::parse_date();

   date::dateformat::two::date_format();
   date::dateformat::two::parse_date();
   println!("Hello, world!");
}


//第一步：在Cargo.toml加入配置   date = {path= "./date"}
//第二步: 在lib.rs同级新建一个rs,同时在lib.rs导入。pub mod dateformat;
//在main.rs中调用其他包中的方法
