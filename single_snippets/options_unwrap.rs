fn give(drink: Option<&str>) {
    match drink {
         // This could get the inner side of drink!
        
        Some(inner) => println!("{}?, Great", inner),
        None => println!("No drink? Good for me!!!"),
    }
}

fn give_vegan(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "milk" {
        panic!("Police!!");
    }
    println!("That's not me!!! {} ", inside);
}


fn main() {
    give(Some("Water"));
    give(None);

    let milk = Some("milk");
    let nothing = None;

    give_vegan(Some("Water"));
    give_vegan(milk);
    // This could cause issue. Can't unwrap the `none` type
    give_vegan(nothing);
 

}