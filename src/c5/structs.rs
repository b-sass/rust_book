struct Product {
    name: String,
    section: String,
    quantity: u32,
    available: bool,
}

// Tuple structs
#[derive(Debug)]
struct RGB(u8, u8, u8);
struct Point(i32, i32, i32);

pub fn structs() {
    let carrots = Product {
        name: String::from("Carrot"),
        section: String::from("Vegetables"),
        quantity: 13,
        available: true,
    };

    if carrots.available {
        println!("Carrots are available.");
    } else { println!("Out of stock!") }

    // carrots.quantity -= 1; carrots is not mutable therefore can't change any values

    let mut apples = add_product(String::from("Apple"),
                                 String::from("Vegetables"),
                                 0);

    apples.quantity = 15;
    apples.available = true; // Can change fields as apples is mutable

    let potatoes = Product {
        name: String::from("Potato"),
        quantity: 20,
        ..carrots // Take the rest of values from carrots (section, available)
    };
}

pub fn tuple_structs() {
    let colors = RGB(32, 64, 128);
    println!("{:?}", colors);
}

fn add_product(name: String, section: String, quantity: u32) -> Product {
    Product {
        name, // Can skip field names as parameter names are the same
        section,
        quantity,
        available: quantity > 0,
    }
}
