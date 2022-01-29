pub mod io;
fn one_two_or_three(s: &str) -> Option<u8> {
    match &s[..] {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        _ => None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let known_value = args.get(1).and_then(|input| one_two_or_three(&input));
    if let Some(v) = known_value {
        println!("we know this word! It's value is: {}", v);
    } else {
        println!("we did not understand or get input");
    }   
    println!("Interactive rebase, hello!");
    io::foo::write_foo("Hello libc world!\r\n"); 
}
