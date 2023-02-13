// ライフタイム'aと'bを持つ引数を参照に取る.
// これらのライフタイムの寿命は､最短でもこの関数のライフタイムと同じになる.
// ライフタイム省略規則その1(各引数に個別のライフタイム引数が当たるケース)で省略できる
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x:{} y:{}", x, y);
}

// こう書くとコンパイルエラー.
// ローカル変数_xのライフタイムが､関数自体のライフタイムより短くなるかららしい.
// fn failed_borrow<'a>() {
//     let _x = 12;
//     let y: &'a i32 = &_x;
// }

// 参照を返す場合にライフタイムを明示する例その1
// ライフタイム省略規則その2(1つだけ入力ライフタイム引数があるなら､出力ライフタイムはそれに合致するケース)で省略できる
fn pass_val<'a, 'b>(v: &'a i32) -> &'a i32 {
    v
}

// 参照を返す場合にライフタイムを明示する例その2
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// 返り値の参照が､変数xかyのいずれかのライフタイムと合致する.
// こう書いた場合､変数xとyのライフタイムは一致しなければならない.
fn pass_y<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    y
}

// 構造体でライフタイムを明示する場合
// ライフタイム省略規則その3(メソッドの& selfや&mut self)で省略できる
struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print_value<'a>(&'a self) {
        println!("val:{}", self.0);
    }
}

// トレイトの例
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}
// ※Defaultトレイトは､構造体のデフォルト値を定義する際に使えるトレイト
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    {
        let (mut x, y) = (4, 9);
        print_refs(&x, &y);

        let z = pass_x(&x, &y);
        println!("z:{}", z);
    }

    {
        let a = 2;
        {
            let b = 3;
            let c = pass_y(&a, &b);
            println!("c:{}", c);
        }
    }

    {
        let b: Borrowed = Default::default();
        println!("b:{:?}", b);
    }
}
