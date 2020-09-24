// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // string literals return &str
    string_slice("blue");
    // &str is converted to a String
    string("red".to_string());
    // String::from() returns a String
    string(String::from("hi"));
    // to_owned() returns a String
    string("rust is fun!".to_owned());
    // into() returns a String
    string("nice weather".into());
    // format! returns a String
    string(format!("Interpolation {}", "Station"));
    // &String::from() returns a &str where if the & is not included in the front then
    // ownership is given
    string_slice(&String::from("abc")[0..1]);
    // trim() removes the whitespaces and then returns a &str
    string_slice("  hello there ".trim());
    // to_string() returns a String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // to_lowercase() returns a String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
