fn main() {

    println!("\nperforming basic arithmatic operations\n");

    // fn: keyword used to declare any function in rust
    
    // let: keyword to declare a variable
    
    let add = 1 + 2;
    let sub = 5 - 4;
    let mul = 3 * 6;
    let div = 6 / 2;
    let rem = 7 % 3;
    
    // println! : not a standard function but a macro
    
    println!("Add \n1 + 2 = {}", add);
    println!("Sub \n5 - 4 = {}", sub);
    println!("Mul \n3 * 6 = {}", mul);
    println!("Div \n6 / 2 = {}", div);
    println!("Rem \n7 % 3 = {}", rem);
}
