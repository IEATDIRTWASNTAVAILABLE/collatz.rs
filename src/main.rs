use text_io::read; 
fn main() {
    let mut input: i64 = read!(); //make input be what you input
    if input == 0 { 
        println!("not a good idea"); // tell you not to input 0 
        panic!(); // self explanatory
    }
    loop{
    let output: i64 = if let 0=input%2{
        input/2
    } else {
        input*3+1 
    };
    println!("{}", output);
    input=output;
    if output == 1{
        break
    }
}
}
