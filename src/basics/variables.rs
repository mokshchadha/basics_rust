use std::vec;

fn main(){
    //&str and String
    let fixed_str :&str = "Fixed length string";
    let mut flexible_str : String = String ::from("This string will grow");

    if flexible_str.len() >1 {
        flexible_str =  String::from("hello 1");
    }else {
        flexible_str= String::from("hello 2");
    }

    println!("{1}, {0}", fixed_str, flexible_str);

    // Arrays
    let  arr : [i32; 5] = [1, 2, 3,4,5];
    println!("{:?}", arr);

    //Vectors 
    let vec_1: Vec<i32> = vec![4, 5,6];
    println!("{:?}", vec_1);

    //tuples
    let my_info :(&str, i32, &str, i32) = ("Salary", 9000, "Age", 12);
    println!("{:?}", my_info);

}