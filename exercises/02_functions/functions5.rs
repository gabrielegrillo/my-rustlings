// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // at first my intuition thought of this line
    // return num * num;
    // but in rust, since it is a expression-based language, you can return implicitly the value by
    // removing the return statement and the semicolon.
    num * num 
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
