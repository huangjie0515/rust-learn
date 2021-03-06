extern crate crypto;  //extern  表示引用外部庫

use  crypto::digest::Digest;
use  crypto::sha3::Sha3;

fn main() {
    let mut  hasher=Sha3::sha3_256();
    hasher.input_str("hello world");
    let result =hasher.result_str();
    println!("hasher=={}",result);
    println!("Hello, world!");
}
