pub fn run() {
    // integers are copied
    let a = 32;
    let b = a;
    println!("a: {}, b: {}", a, b);
    
    // String are moved, they're pointers to the heap
    let a = String::from("eh");
    let b = &a;
    println!("a: {}, b: {}", a, b);
}
