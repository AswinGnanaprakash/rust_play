use std::io;

fn main() {
    let mut input_text = String::new();
    println!("Enter a name:");
    // get the input_text from cli
    io::stdin()
        .read_line(&mut input_text)
        .expect("Input not given");
    
    let trimmed_text = input_text.trim();
    match trimmed_text.parse::<i32>() {
        Ok(i) => function_call(i),
        Err(..) => println!("A bad day for you!!!"),
    }
}

fn function_call(get_value : i32) {

    match get_value { 
        0 => println!("Empty"),
        _ => println!("Rest of the error!!!"),
    }

}