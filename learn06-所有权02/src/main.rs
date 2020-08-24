fn takes_ownership(some_string:String){
    println!(" {} ",some_string);
}


fn makes_copy(i:i32){
    println!("i = {}",i);
}

fn main() {
   let str="111";
   takes_ownership(str);
}
