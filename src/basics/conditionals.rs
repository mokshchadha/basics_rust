
fn main(){

    let marks = 95;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        _ => grade = 'C',
    }
     
     println!("{:?}",grade);
}

 