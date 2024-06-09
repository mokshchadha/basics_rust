fn main() {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone());
    println!("vec_1 is {:?}", vec_1);

    let vec_2 = gives_ownership();
    println!("vec_2 is {:?}", vec_2);
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}
