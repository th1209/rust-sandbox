fn get_int_from_file() -> Result<i32, String> {
    let path = "assets/number.txt";

    let num_str_result = std::fs::read_to_string(path)
        .map(|e| {e.to_string()});
    let num_str = match num_str_result {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    // ↑の代わりに､↓の書き方もできる
    // ?はResult型を返す時に使え､Err(E)の場合のみ､関数を早期returnして終了する
    // let num_str = std::fs::read_to_string(path)
    //     .map_err(|e| {e.to_string()})?;

    num_str
        .trim()
        .parse::<i32>()
        // mapはOk(T)の場合のみ処理される
        .map(|t| t * 2)
        // map_errorはErr(E)の場合のみ処理される
        .map_err(|e| e.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
