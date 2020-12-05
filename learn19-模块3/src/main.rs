mod modA{
    #[derive(Debug)]
    pub struct A{
        pub  number:i32,
        name :String,
    }
    impl A{
        pub fn new_a()   -> A{
            A{
                number:1,
                name: String::from("A"),
            }
        }
        pub fn print_a(&self){
            println!("number: {},name: {}",self.number,self.name);
        }
    }

    pub mod  modB{
        pub fn print_B(){
           println!("1111");
        }
        pub mod modC{
            pub fn pint_C(){
             println!("C==========");
                //調用父類用super
             super::print_B();
            }
        }
    }
}

use modA::A;
use modA::A as B;


fn main() {
    println!("Hello, world!");
    let  a=A::new_a();
    a.print_a();

    B::new_a().number;

    modA::modB::modC::pint_C();
}
