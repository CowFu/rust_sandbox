pub fn run() {
    //INTEGERS
    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128

    // arch	isize	usize uses 32 or 64-bit depending on system
    let xisize: isize = isize::MAX;
    let xusize: usize = usize::MAX;
    println!("isize: {} usize: {}", xisize, xusize);

    // addition
    let sum = 5 + 11;
    println!("sum = 5 + 11 = {}", sum);

    // subtraction
    let difference = 25.3 - 4.1;
    println!("difference = 25.3 - 4.1 = {}", difference);

    // multiplication
    let product = 5 * 25;
    println!("product = 5 * 25 = {}", product);

    // division
    let quotient = 92.7 / 2.2;
    println!("quotient = 56.7 / 32.2 = {}", quotient);

    // integers floor in division
    let floored = 2 / 3; // Results in 0
    println!("floored 2 / 3 = {}", floored);

    // remainder
    let remainder = 43 % 6;
    println!("remainder 43 % 5 = {}", remainder);

    // boolean
    let t = true;
    let f: bool = false;

    if t {
        println!("t was true!");
    } else {
        println!("oh no!");
    }
    println!("{}", f);

    // chars have single quotes, strings have double quotes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Char c: {}, z: {}, cat: {}", c, z, heart_eyed_cat);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tuple print: {:?}", tup);

    println!("tuple first element: {}", tup.0);

    // destructuring the tuple
    let (x, y, z) = tup;

    println!("deconstructed tuple is: {} {} {}", x, y, z);

    // unit type tuple, expressions implicitly return the unit value if they don't return any other value.
    let empty_tup = ();
    println!("Empty tup: {:?}", empty_tup);

    // arrays - all same type, stack not heap, fixed size [type:size]
    let a: [i8;5] = [1,2,3,4,5];
    println!("a[1]: {}", a[0]);

    // array of 5 3's [data;size]
    let a = [3; 5];
    println!("Array of all 3's: {:?}", a);

    
}
