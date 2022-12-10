fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 96.6 - 4.9;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Result is 0

    // remainder
    let remainder = 43 % 4;

    /// boolean
    let t = true;

    let f: bool = false;

    /// char
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    let kana_a = 'ア';

    /// tuple
    let tup: (i32, f64, u8) = (600, 5.2, 3);

    let (x, y, z) = tup; // パターンマッチングによる分解

    let six_hundred = tup.0; // tupの0番目にアクセス

    let five_point_two = tup.1; // tupの1番目にアクセス

    let three = tup.2; // tupの2番目にアクセス

    /// array
    // Rustの配列は固定系なので、一度宣言されたらサイズを伸ばすことも縮めることもできない
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32: 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // 初期値3を5個持った配列を初期化

    let first = a[0]; // 配列の1番目にアクセス

    let second = a[1]; // 配列の2番目にアクセス
}
