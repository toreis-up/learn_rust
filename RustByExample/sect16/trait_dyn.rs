struct Sheep {}
struct Cow {}

trait Animal {
  fn noise(&self) -> &'static str;
}

// SheepにAnimalトレイトを実装する
impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaaaah!"
  }
}

// CowにAnimalトレイトを実装する
impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "moooooo!!"
  }
}

// Animalが実装されてるstructを返す、けどどの具象が帰ってくるのかはコンパイル時点ではわからないし、なんなら私も知らない
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

fn main() {
  let random_number = 0.234;
  let animal = random_animal(random_number);
  println!("You've randomly chosen an animal, and it says {}", animal.noise());
}