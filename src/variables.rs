pub fn run() {
    //mut keyword to make variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadow a varable by re-using let
    let x = x + 1;

    {
        //scope change
        let x = x * 2;
        println!("The value of x is {}", x);
    }

    println!("Now the value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    print!("{}\n", THREE_HOURS_IN_SECONDS);

    let spaces = "     ";
    let spaces: usize = spaces.len();

    // print! macro doesn't have newline at the end
    print!("{} spaces in that string\n",spaces);

}
