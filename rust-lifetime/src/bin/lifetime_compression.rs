// 呼び出し元に注目!
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// 2つのライフタイムが異なる参照があって､片一方をどちらか短い方のライフタイムにやり込めることを､ライフタイムの圧縮という.
// ここでは､ライフタイム'aは､最低でも'bと同じ長さと扱われる.
// 試しに↓のように書き換えるとコンパイルエラーになってくれる
// fn choose_first<'a, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;
    {
        let second = 3;
        
        // 一見コンパイルエラーになりそうだけど､Rustコンパイラがライフタイムをできるだけ短く見積もっているので､コンパイルエラーにならない.
        println!("The product is {}", multiply(&first, &second));

        // ライフタイムの圧縮が効く例
        println!("First is {}", choose_first(&first, &second));
    }
}
