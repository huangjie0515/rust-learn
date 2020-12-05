
//1.當編寫一個函數，但是該函數可能會失敗，當時除了在函數中處理錯誤外，還可以將錯誤傳給調用者，讓調用者決定如何處理，這被稱爲
//傳播錯誤
//2.傳播錯誤的簡寫方式，提倡的方式
//3.更進一步的簡寫
//4.什麼時候用panic!，什麼時候用Result
   //(1)示例、代碼原型，測試用panic! unwrap\expect
   //(2)實際項目中應該用Result
//5.Option和Result

use std::io;
use std::fs::File;
use std::error::Error;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let r=read_username_from_file();
    match r {
        Ok(s)  =>  println!("s =={}",s),
        Err(err)  => println!("err  = {:?}",err),
    }
}

/*fn  read_username_from_file() -> Result<String,io::Error>{
    let f=File::open("hello.txt");
    let mut f=match f {
        Ok(file) => file,
        Err(err) =>return Err(err),
    };
    let mut  s=String::new();
    match f.read_to_string(&mut s) {
        Ok(_)  => Ok(s),
        Err(error) =>  Err(error),
    }
}
*/


fn  read_username_from_file() -> Result<String,io::Error>{
   let mut  s=String::new();
    File::open("hello.txt111")?.read_to_string(&mut s);
    Ok(s)
}