fn main() {}

fn cannot_borrow_mut_more_than_once() {
    let mut vec_1 = vec![1, 2, 3];
    let ref1 = &mut vec_1;

    // let ref2 = &mut vec_1; -- this will be error

    println!("{:?}", ref1)
}
