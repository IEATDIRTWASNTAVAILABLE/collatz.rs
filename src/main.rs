use text_io::read; 
fn main() {
    let mut input: i64 = read!();
    loop{
    let checkeven: i64 = if let 0=input%2{
        1
    } else {
        0
    };
    let output: i64 = if checkeven == 1{
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
