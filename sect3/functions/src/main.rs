fn main() {
    println!("Hello, world!");

    let x = {
        let y = 3;
        y + 1 // セミコロンをつけないと式として返される
    };

    another_function(x, 'h');

    let x = plus_one(x);

    another_function(x, 'c');
}

fn another_function(x: i32, c: char) {
    println!("Another function is called with {}, {}", x, c); // 別の関数
}

fn plus_one(x: i32) -> i32 {
    x + 1
}