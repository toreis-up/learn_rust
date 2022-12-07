use rand::Rng;
use std::cmp::Ordering;
use std::io; // Cでいうstdio.h的な？

fn main() {
    // 単純に表示
    println!("Guess the number!");

    // 不変の変数(変数と言えるのだろうか笑) secret_numberへランダムに生成した値をいれる
    let secret_number = rand::thread_rng().gen_range(1..101);

    // secret_numberを表示
    // println!("The secret number is : {}", secret_number);

    loop {
        // これも
        println!("Please input your guess.");

        // ユーザーの入力を格納するための変数
        // mutをつけることで可変にする
        // →mutがなかったら不変
        //
        // String::new()で得られたString型のインスタンスをguessへ
        let mut guess = String::new();

        // stdin関数はターミナルの標準入力へのハンドルを表す型 std::io::Stdinのインスタンスを返すらしい
        io::stdin()
            .read_line(&mut guess) // &mut guessにユーザからの入力を格納
            .expect("Failed to read line"); // プログラムをクラッシュさせ、引数として渡されたものを表示する

        // guessはString型で宣言したが、secret_numberはi32(int)で型の不一致が起きる
        // 同一の名前で宣言することで型の覆い隠しができる
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // 正常にパースができているならnumをいれる
            Err(_) => continue, // エラーならloopを最初に戻す
        };

        println!("You guessed: {}", guess); // {}はプレースホルダー

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breakでloopを抜け出せる
            }
        }
    }
}
