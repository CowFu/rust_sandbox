pub fn run() {
    // MUST declare type for each parameter
    // Expressions return a value statements do not
    println!("The value of x is: {}", square_function(3));
    statement_function(3)
}

/// Returns the square of the parameter passed to it
/// 
/// ### Parameters
/// 
/// * `x` i32 value to be doubled
fn square_function(x: i32) -> i32 {
    x * x
}

/// Documentation for the function
/// 
/// ### Parameters
/// 
/// * `y` i32 will be printed
fn statement_function(y: i32) {
    println!("{}", &y);
}