//1.rust語言將錯誤分爲兩個類別，可恢復錯誤和不可恢復錯誤。


//（1）可恢復錯誤通常代表向用戶報告錯誤和重試操作是合理的情況，例如未找到文件。rust中使用Result<T,E>來實現。
//(2) 不可恢復錯誤是Bug的同義詞，如嘗試訪問超過數據結尾的位置。rust中通過panic!實現。


//2.panic!


//3.使用BACKTRACE=1

//4.Result<T,E>
//enum Result<T,E>{
//   Ok(T),
//   Err(T),
//
// }
//
//
//




//5.簡寫
use  std::fs::File;
fn main() {
    /*println!("Hello, world!");
    let f=File::open("hello.txt");
    let r=match f {
        Ok(file) =>file,
        Err(error) =>panic!("error:{:?}",error),
    };*/

    //File::open("hello.txt").unwrap();


    File::open("hello.txt").expect("Failed to  open file");


}
