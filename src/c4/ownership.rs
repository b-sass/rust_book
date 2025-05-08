pub fn ownership() {
    let a = Box::new(15);
    let b = a;
    // println!("{a}"); // Does not compile ( value borrowed after move )
    println!("{b}");
}

pub fn suffix() {
    let dad = String::from("Bob"); // Create string on the heap
    let son = add_suffix(dad);
    // println!("Dad: {dad}") // Does not compile
    println!("Son: {son}");
}

pub fn suffix_copy() {
    let dad = String::from("Alex");
    let son = add_suffix(dad.clone());
    println!("Dad: {}, Son: {}", dad, son);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr."); // Add suffix to string
    name
}