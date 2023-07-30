use text_io::read; 
fn main() {
    let mut input: i64 = read!(); //make input be what you input
    if input == 0 { 
        println!("not a good idea") // tell you not to input 0 
        panic!() // self explanatory
    }
    loop{
    let checkeven: i64 = if let 0=input%2{
        1 //define checkeven as 1 if input is even (i have no idea how this works its just code i
          //stole from a website
    } else {
        0 //define checkeven as 0 if it's not even 
    };
    let output: i64 = if checkeven == 1{
        input/2 //divide by 2 if checkeven is 1
    } else {
        input*3+1 //times 3 plus one if checkeven is 0
    };
    println!("{}", output);
    input=output;
    if output == 1{
        break
    }
}
}
