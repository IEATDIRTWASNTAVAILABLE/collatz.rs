use text_io::read; 
fn main() {
    println!("Input a number that isnt 0, contains no decimals, and doesn't rise above the 64 bit limit \nThe first output is the number of steps taken, second output is what that step is");
    let mut stepscount=1;
    let mut input: i64 = read!();
    let mut output: i64 = 0;
    
    if input == 0 { 
        println!("I TOLD YOU NOT 0");
        panic!();
    }

    loop{
        if input % 2 == 0{
            output = input / 2
        } else {
            output = input * 3 + 1 
        };
        println!("{stepscount} , {output}");
        input = output;
        stepscount=stepscount + 1;
        if output == 1{
            break
        }

    }
}