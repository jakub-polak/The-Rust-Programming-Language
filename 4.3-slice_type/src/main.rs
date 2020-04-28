fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];

    println!("{}, {}!", hello, world);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}