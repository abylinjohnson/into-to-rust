//Struct = used to create custom data types

//Traditional Struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

//Tuple Struct
struct TColor(u8,u8,u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    //Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Get Full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set Last name 
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name Tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut c = Color{
        red: 233,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color : {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255,0,0);
    println!("Color : {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("John", "Doe");
    p.set_last_name("Pink");
    println!("Person: {}", p.full_name());
    println!("Name Tuple: {:?}", p.to_tuple());
    
}