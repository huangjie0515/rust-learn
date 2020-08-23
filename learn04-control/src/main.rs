fn main() {
    let y = 0;
    if y == 1 {
        println!("y =1");
    }

    //if else {  }
    if y==1{
        println!("y =1 ");
    }else {
        println!(" y !=1");
    }

    //if else if{}
    if y==1{
        println!(" y =1 ");
    }else if y==2 {
        println!(" y =2 ");
    }else if y==3{
        println!(" y =3 ");
    }

    //let中使用if  语句
    let condition=true;
    let x=if condition{
        5
    }else {
        6
    };
    println!("x = {}",x);

    //loop
    let mut counter =0;
    loop {
        println!("in loop ");
        if counter==10{
            break;
        }
        counter+=1;
    }

    let result= loop {
        counter +=1;
        if counter==20{
            break   counter*2;
        }
    };
    println!(" result ={}",result);

}
