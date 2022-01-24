pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    /* println!(1);
     error: format argument must be a string literal
     --> src\print.rs:5:14
      |
    5 |     println!(1);
      |              ^
      |
    help: you might be missing a string literal to format with
      |
    5 |     println!("{}", 1);
      |              +++++*/

    // Basic print format
    println!("{} is from {}", "CowFu", "Angeles City");
    println!(
        "{0} likes to fight {1}s and eat the {1}'s {2}",
        "CowFu", "Troll", "Food"
    );
    println!(
        "{name} likes to {activity}",
        activity = "sleep",
        name = "Stephen"
    );

    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}
