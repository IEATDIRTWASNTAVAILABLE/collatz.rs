use text_io::read; 
fn main() {
    println!("Input a number that isnt 0, contains no decimals, and doesn't rise above the 64 bit limit \nThe first output is the number of steps taken, second output is what that step is");
    let mut stepscount=1;
    let mut input: i64 = read!();
    if input == 0 { 
        println!("I TOLD YOU NOT 0");
        panic!();
    }
    loop{
    let output: i64 = if let 0=input%2{
        input/2
    } else {
        input*3+1 
    };
    println!("{stepscount} , {output}");
    input=output;
    stepscount=stepscount+1;
    if output == 1{
        break
    }

}
}
