fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    shadowing();
}

fn shadowing() {
    let x = 5;
    let x = x + 1; // shadowing

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");
}

fn memorySafe() {
    let a = [3;5];
    // println!("a[8] is ${a[8]}"); // will throw error when cargo run
}