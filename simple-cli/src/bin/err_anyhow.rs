use anyhow::{bail, ensure, Context, Result};

// 通常のResult型の代わりに､anyhowのResult型を使う
// anyhowのResult型は､正常型の型指定しか無い. エラーの場合は､anyhow::Error型となり存在を隠蔽してくれる.
fn get_int_from_file() -> Result<i32> {
    let path = "assets/number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

    // bailやensureを使うと､簡単に早期returnが実現できる
    if num_str.len() >= 10 {
        bail!("it may be too large number");
    }
    ensure!(num_str.starts_with("1"), "first digit is not 1");

    // contextやwith_contextは､エラーの場合のみ処理される
    // (with_contextは変数キャプチャしたい時に使うこと)
    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .context("failed to parse string")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
