// Strucs - Used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
// struct Tuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut t = Tuple(255, 0, 0);

    // t.0 = 200;

    // println!("Tuple: {} {} {}", t.0, t.1, t.2);

    let mut p = Person::new("Vinícius", "Asdfgerg");
    println!(
        "Person {} {}, your full name is {}",
        p.first_name,
        p.last_name,
        p.full_name()
    );

    p.set_last_name("Dias");
    println!(
        "Person {} {}, your full name is {}",
        p.first_name,
        p.last_name,
        p.full_name()
    );

    println!("Person Tuple {:?}", p.to_tuple());
}
