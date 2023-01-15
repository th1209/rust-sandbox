
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // Derefトレイトを実装すると､参照外し記法で使える!
    assert_eq!(5, *y);


    let m:MyBox<String> = MyBox::new(String::from("Rust"));
    // Derefトレイトを実装すると､参照外し型強制が効いてくれ､以下のように暗黙的な型変換が行われる
    // &MyBox<String> -> &String -> &str
    hello(&m);
    // もしも参照外し型強制がないと､以下のように大変面倒なコードを書く必要が出てくる｡
    hello(&(*m)[..])
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
