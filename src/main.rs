use std::io;

fn main() {
    let mut user_string = String::new();
    println!("Type whatever, the program will now count how many characters you type!");

    io::stdin()
        .read_line(&mut user_string) // Gives the program access to your string
        .expect("Failed to read line");
        println!("{}", user_string.trim().len());
    // Turns out, doing it without .trim would actually do some fucky shit like
    // "Hello\n", and .trim removes the new line. Why the fuck are we making new
    // lines without being told? Especially ones that are purely fucking empty?

}
