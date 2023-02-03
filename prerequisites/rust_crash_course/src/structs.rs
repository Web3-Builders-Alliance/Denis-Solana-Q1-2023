


struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {






  let mut p = Person::new("Mary", "Doe");
  println!("Person {}", p.full_name());
  p.set_last_name("Williams");
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());
}
