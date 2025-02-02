// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}
fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // placeholder("blue");
    string_slice("blue");

    // placeholder("red".to_string());
    string("red".to_string());

    // placeholder(String::from("hi"));
    string(String::from("hi"));

    // placeholder("rust is fun!".to_owned());
    string("rust is fun!".to_owned());

    // placeholder("nice weather".into());
    string("nice weather".into());

    // placeholder(format!("Interpolation {}", "Station"));
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // placeholder(&String::from("abc")[0..1]);
    string_slice(&String::from("abc")[0..1]);

    // placeholder("  hello there ".trim());
    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
