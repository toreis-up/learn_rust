// https://doc.rust-jp.rs/rust-by-example-ja/custom_types/structs.html

#[derive(Debug)]
struct Person {
    // The 'a defines a lifetime
    name: String,
    age: u8,
}

// A unit struct
// ユニット
struct Unit;

// A tuple struct
// タプル
struct Pair(i32, f32);

// A struct with two fields
// 2つのフィールドを持つ（クラシックな）構造体
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    // 長方形は座標空間上における左上隅と右下隅の位置によって指定できる
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
  let Rectangle {top_left: Point {x: left_x, y: top_y}, bottom_right: Point {x: right_x, y: bottom_y}} = rectangle;
    (top_y - bottom_y) * (right_x - left_x)
}

fn square(point: Point, length: f32) -> Rectangle {
  Rectangle {
    top_left: Point {
      x: point.x,
      y: point.y + length,
    },
    bottom_right: Point {
      x: point.x + length,
      y: point.y,
    }
  }
}


fn main() {
    // Create struct with field init shorthand
    // 構造体をフィールド初期化の簡略記法で生成
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    // 構造体のデバッグ表示を行う
    println!("{:?}", peter);


    // Instantiate a `Point`
    // `Point` のインスタンス化
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    // 構造体の更新記法を用いて、別の構造体のフィールドの値を基に
    // 新たなpointを生成
    let bottom_right = Point { x: 5.2, ..point };
    
    let bottom_right = Point { y: 6.3, ..bottom_right };
 
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    // `bottom_right.y` の値は `point.y` と同一になるが、
    // これは `point` のフィールドの値を用いて生成したためである
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // `let`を使用してpointをデストラクトする。
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        // 構造体の定義とインスタンスの作成を同時に行う
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    
    println!("area: {}", rect_area(_rectangle));


    // Instantiate a unit struct
    // ユニットをインスタンス化
    let _unit = Unit;

    // Instantiate a tuple struct
    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    // タプルのフィールドにアクセス
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    // タプルをデストラクト
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    
    let new_point = Point {x: 0.0, y: 0.0};
    let new_square = square(new_point, 5.0);
    
    println!("new square area: {}", rect_area(new_square));
}
