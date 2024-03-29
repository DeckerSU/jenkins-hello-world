fn main() {
    let str = "Hello, world!";
    let hex_string = hex::encode(str);
    println!("{}", str);
    println!("{}", hex_string);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let flag = true;
        assert_eq!(flag, true);
    }
}