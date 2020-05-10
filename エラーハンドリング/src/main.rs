use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::path::Path;

// 1から10までの数字を予想します。
// もし予想した数字に一致したらtrueを返し、そうでなければfalseを返します。
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}

// `haystack`（干し草の山）からUnicode文字 `needle`（縫い針）を検索します。
// もし見つかったら、文字のバイトオフセットを返します。見つからなければ、`None` を
// 返します。
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

// 与えられたファイル名の拡張子を返す。拡張子の定義は、最初の
// `.` に続く、全ての文字である。
// もし `file_name` に `.` がなければ、`None` が返される。
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i + 1..])
}

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err),
    }
}

fn double_number2(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    // #1
    // guess(11);

    // #2
    // let mut argv = env::args();
    // let arg: String = argv.nth(1).unwrap(); // エラー1
    // let n: i32 = arg.parse().unwrap(); // エラー2
    // println!("{}", 2 * n);

    // #3
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i + 1..]),
    }

    // #4
    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");

    // #5
    match double_number2("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }

    // #6
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
