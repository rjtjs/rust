use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.trim().to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    let mut name = String::new();

    stdin().read_line(&mut name).expect("Failed to read input");

    name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("radahn", "hey starscourge!"),
        Visitor::new("malenia", "hey rotting flower!"),
        Visitor::new("godrick", "hey, dumbass"),
    ];

    loop {
        println!("Enter a name or press ENTER to quit: ");

        let name = get_name();

        let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on visitor list", name);
                    visitor_list.push(Visitor::new(&name, "new tarnished"));
                }
            }
            
        }
    }

    println!("Final visitor list: ");
    println!("{:#?}", visitor_list);
}
