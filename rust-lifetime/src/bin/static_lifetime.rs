// スタティックライフタイムは､単純なスタティックライフタイムとスタティックライフタイムという2つの文法がある.
// 両者は全然異なるものなので､よく区別する!

use std::fmt::Debug;

// 1.スタティックライフタイムの例 スタティックライフタイムとは､アプリケーションの終了時まで生きるライフタイム
static NUM: i32 = 18;
// スタティックライフタイムを圧縮する例
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

// 2.スタティックライフタイム境界の例 
// スタティックライフタイム境界とは､ざっくりいうと型Tには参照を含まない(型Tが構造体やベクタなどの複合型でも､その中身も参照は含まない)ことを強制するライフタイム境界.
fn print_it(input: impl Debug + 'static) {
    println!("static value passed in is: {:?}", input);
}

fn main() {
    {
        {
            let static_string = "I'm in read-only memory.";
            println!("static_string:{}", static_string);
    
            let lifetime_num = 9;
            let coerced_static = coerce_static(&lifetime_num);
            println!("coerced_static:{}", coerced_static);
        }

        // 上記の後でも､当然スタティックライフタイムな変数は最後まで生き残る
        println!("Num:{} stays accessible!", NUM);
    }

    {
        let i = 5;
        print_it(i);
        // ↓は参照なので､スタティックライフタイム境界が付いているのでコンパイルエラーになる!
        // print_it(&i);
    }
}