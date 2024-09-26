// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called.

    //  Print text to the console.
    println!("Hello World!");

    // Loop
    let mut count = 0u32;

    println!("Counting to 10,000!");

    loop {
        count += 1;

        println!("{}", count);

        if count == 10000 {
            println!("The End!");

            // Exit loop
            break;
        }
    }
}