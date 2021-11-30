fn main() {
    // println!("Hello, world!");
    // ! が付くとマクロらしい(非関数)

    fizzbuzz(72);

    println!( "{}",out_str() );
}

// fn test() -> String { // 返り値がある場合は型をこんな感じで指定するらしい
//     return "testString";
// }

fn fizzbuzz( n: usize ) { // 引数は typescript みたいな 変数名:型 パターン
    for i in 0..n { // 0..n レンジリテラル この範囲のイテレーション的な
        if i % 15 == 0 { // boolean しか指定できない
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i); // println! 文字列に"{}"入れ込めばフォーマッティング出来る
        }
    }
}

// (例えば) `文字` を返してみる
fn out_str() -> String {
    return "文字".to_string();
}


// 偶数二乗合計だって
// fn square_sum( n: isize ) -> isize { // isize 自PCアーキテクチャに合うビット分のサイズを取る整数型らしい 64bitPcなら isize===i64
    // (0..n)
    //     .filter(|i| i%2 == 0)
// }