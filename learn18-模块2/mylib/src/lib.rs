use  produce_refrigerator;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main(){
    produce_refrigerator::produce_re();
}