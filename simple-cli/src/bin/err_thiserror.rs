use thiserror::Error;

// thiserrorのError型を継承する
#[derive(Error, Debug)]
enum MyError {
    // errorアトリビュートで､fmt::Displayした際の表示を決めることができる
    #[error("failed to read string from {0}")]
    ReadError(String),
    // 名前の通り､発生元のエラー出力をそのまま使う
    #[error(transparent)]
    // fromアトリビュートをつけておくと､fromで受けられる
    ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "assets/number.txt";

    let num_str = std::fs::read_to_string(path)
        .map_err(|_| MyError::ReadError(path.into()))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        // fromで受けられる
        .map_err(MyError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
