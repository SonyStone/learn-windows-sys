use std::fmt::Display;

pub struct Droppable {
    name: String,
    tick: i32,
}

impl Droppable {
    pub fn new(name: &str) -> Self {
        Droppable {
            name: name.to_string(),
            tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }
}

impl Default for Droppable {
    fn default() -> Self {
        Droppable {
            name: "default".to_string(),
            tick: 0,
        }
    }
}

impl Display for Droppable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(> Still here {} | tick {})", self.name, self.tick)
    }
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("(> Dropping {} | tick {})", self.name, self.tick);
    }
}
