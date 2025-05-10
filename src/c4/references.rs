pub fn references() {
    let mut age: Box<i32> = Box::new(21);

    let my_age = *age; // Read age from heap
    *age += 1; // Modify heap value
    
    println!("{}", my_age);
    println!("{}", age);
    println!("{}", *age); // Same result as above#

    // reference to age on the heap
    let r_age = &*age;
    println!("r_age -> {r_age}");

    let name = String::from("John");
    println!("My name is {name}");
    greet(name);
    // println!("{name}"); Cannot use name after transferring ownership to the function

    let a1 = age.abs(); // Implicit function reference
    let a2 = i32::abs(*age); // Explicit reference

    let s = String::from("John");
    let s1 = s.len(); // Implicit reference
    let s2 = String::len(&s); // Explicit reference

}

pub fn borrowing() {
    // immutable references (shared references)
    let mut friends = vec!("Alice", "Bob", "Charlie");
    let sister = &friends[0]; // Borrowed first index of friends
    // *sister = "Eve"; Doesn't work - can't alias and mutate values at the same time
    println!("friends -> {:?}", friends); // Can still read friends when it's borrowed
    println!("sister -> {sister}");

    friends[0] = "Eve"; // Once ownership is given back to friends, the vector can be mutated again
    println!("friends -> {:?}", friends);

    // mutable references (unique references)
    let mut friends = vec!("David", "Eve", "Fred");
    let brother: &mut &str = &mut friends[2];
    //println!("friends -> {:?}", friends); // friends lost all permissions after being borrowed
    *brother = "Dutch";
    
    println!("brother -> {brother}");
    println!("friends -> {:?}", friends); // Friends got all permissions back one brother is not in use
    
    // Notes:
    // shared references allow for aliases but no mutations
    // unique references allow for mutations but no aliases
    
}

fn greet(name: String) {
    println!("Hello {name}"); // "John" is deleted from the heap as name is not used
}

