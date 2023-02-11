fn variables_and_mutability() {
    let x = 5;

    let x = x + 1;

    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
}

fn main() {
    variables_and_mutability();
    data_types();
}