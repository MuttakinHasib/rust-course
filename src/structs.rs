// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct

struct Color(u8, u8, u8);

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn get_info(&self) -> String {
        format!("{} {}", self.name, self.age)
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.green = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    let c = Color(255, 0, 0);

    println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Hasib", 24);
    println!("Person: {} {}", p.name, p.age);

    p.set_name("Muttakin Islam Hasib");

    println!("Person: {}", p.get_info());
}
