use anyhow::{Context, Result};

// 通常のResult型の代わりに､anyhowのResult型を使う
// anyhowのResult型は､正常型の型指定しか無い. エラーの場合は､anyhow::Error型となり存在を隠蔽してくれる.
fn get_int_from_file() -> Result<i32> {
    let path = "assets/number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

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
