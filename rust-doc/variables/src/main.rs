use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

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
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 + 4.3;

    // multiplicatiopn
    let product = 4 * 30;

    // addition
    let quotient = 56.7/32.2;
    let truncated = -5 /3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'Z';
    let z: char = 'Z'; // with explicit type annotation
    let heat_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("THe value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    
    let six_point_four = x.1;

    let one = x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let months = ["January", "February","March","April","May","June","July","August","September","October","November","December",];

    fn array_slicing(array_data: [i32; 5]) {
        println!("Please enter an array index.");

        let mut index = String::new();
        
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    }


}

fn temp_calculator(unit:String) {
    let opposite = if unit == "fahrenheit" {
        "celsius" 
    } else {
        "fahrenheit" 
    };
    println!("Please enter a temperature in {opposite}: ");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Please enter a number");
    let converted_temp: f64 = if unit == "fahrenheit" {
        (temperature*1.8)+32.0
    } else {
        (temperature-32.0)/1.8
    };
    let converted_temp = converted_temp.round();
    println!("The temperature in {opposite} is: {converted_temp}");

}

fn fahrenheit_to_celsius() {
    println!("Would you like to convert to celsius or fahrenheit? (c/f) ");
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");
    if user_choice.trim() == "f" {
        temp_calculator("fahrenheit".to_string());
    } else if user_choice.trim() == "c" {
        temp_calculator("celsius".to_string());
    } else {
        println!("Please enter either 'c' or 'f'");
        fahrenheit_to_celsius()
    }
}

fn init_fibonnaci() -> u32 {
    let mut user_length = String::new();
    io::stdin()
        .read_line(&mut user_length)
        .expect("could not read line");
    let user_length: u32 = user_length
        .trim()
        .parse()
        .expect("Please enter a positive real number");
    user_length
}

fn fibonacci(length: u32) {
    if length == 0 {
        println!("Please enter a number greater than 0")
    } else if length == 1 {
        println!("{}",1);
    } else {
        let mut vec: Vec<u32> = vec![1, 1];
        for i in 2..length as usize{
            vec.push(&vec[i-1] + &vec[i-2]);
        }
        println!("Your fibonacci sequence is: ");
        for x in &vec {
            println!("{x}");
        }
    }
}

fn implement_fibonacci() {
    println!("Please enter a length for your fibonacci sequence");
    let fibonnacci_length = init_fibonnaci();
    fibonacci(fibonnacci_length);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(){
    let file_path = "lyrics.txt";
    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");
    // println!("With text:\n{:?}", contents);
    // println!("{}", contents.len());
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip); 
            }
        }
    }
}

fn main() {
    // variables_and_mutability();
    // data_types();
    // fahrenheit_to_celsius()
    // implement_fibonacci();
    read_file();
}


