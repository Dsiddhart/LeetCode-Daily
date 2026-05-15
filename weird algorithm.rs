use std::io;
fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
    let mut num:u64= input.trim().parse().expect("failed");

    let mut v: Vec<u64> = Vec::new();

    v.push(num);

    while num !=1 {
        if num %2 ==0 {
            num =num/2;
        }
        else {
            num = 3*num +1;
        }
        v.push(num);
    }
    for i in &v{
        print!("{} ", i);
    }
   
}
