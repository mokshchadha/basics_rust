
fn main(){
    let res = multiplication(10, 20);
    println!("res {:?}", res);
}

fn multiplication(a:i32, b:i32) -> i32 {
    return a*b;
}

fn basic_math(num1:i32, num2:i32) -> (i32, i32, i32) {
    (num1* num2, num1+ num2 , num1-num2)
}