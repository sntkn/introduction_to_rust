use thiserror::Error;

#[derive(Error, Debug)] // thiserror 拡張
enum MyError {
    #[error("failed to read string from {0}")]
    ReadError(String),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError), // From<T>
}

fn get_int_from_file() -> Result<i32, MyError> {
    // <- 書いてないけどエラーは anyhow::Error を返す
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::from)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
