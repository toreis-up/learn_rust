struct Sheep { naked: bool, name: &'static str }

trait Animal {
  // 関連関数のシグネチャ
  // Selfはこのトレイトを実装している型になる。
  fn new(name: &'static str) -> Self;

  // メソッドのシグネチャ
  // これらの関数は文字列を返す
  fn name(&self) -> &'static str;
  fn noise(&self) -> &'static str;

  // メソッドのデフォルトの挙動を定義することもできる。
  fn talk(&self) {
    println!("{} says {}", self.name(), self.noise());
  }
}

impl Sheep {
  fn is_naked(&self) -> bool {
    self.naked
  }

  fn shear(&mut self) {
    if self.is_naked() {
      // メソッドをある型に実装する際に、その型のトレイトメソッドを使用することができる。
      println!("{} is already naked...", self.name());
    } else {
      println!("{} gets a haircut", self.name);

      self.naked = true;
    }
  }
}

// AnimalというトレイトをSheepに実装する
impl Animal for Sheep {
  // Selfは実装対象の型 ここではSheep
  fn new(name: &'static str) -> Sheep {
    Sheep { name: name, naked: false}
  }

  fn name(&self) -> &'static str {
    self.name
  }

  fn noise(&self) -> &'static str {
    if self.is_naked() {
      "baaaaaaah?"
    } else {
      "baaaaaaah!!"
    }
  }

  // デフォルトのトレイトメソッドはオーバーライドすることができる
  fn talk(&self) {
    println!("{} pauses briefly... {}", self.name, self.noise());
  }
}

fn main() {
  let mut dolly: Sheep = Animal::new("Dolly");

  dolly.talk();
  dolly.shear();
  dolly.talk();
  dolly.shear();
  
}