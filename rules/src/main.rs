fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);

    // print!("{}", s1); // error because s1 has been borrowed

    let s2 = gives_ownership();

    println!("outter {}", s2); // reuse previous value by return new params
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    println!("inner: {}", some_string);

    some_string
}