#[macro_export]
macro_rules! e2e_test {
    ($func:ident $($args:expr),*) => {
        let result;
        if $func($($args),*) {
            result = "PASSED";
        } else {
            result = "FAILED";
        }
        println!("{} - {}", stringify!($func), result);
    };
}

#[cfg(test)]
mod tests {
    fn test_zero_args() -> bool {
        true
    }

    fn test_one_arg(_data: u8) -> bool {
        true
    }

    fn test_multiple_args(_num: u8, _text: &String, _data: &Vec<u32>) -> bool {
        true
    }

    #[test]
    fn test_macro_e2e_test_no_args() {
        e2e_test!(test_zero_args);
    }

    #[test]
    fn test_macro_e2e_test_single_arg() {
        e2e_test!(test_one_arg 250);
    }

    #[test]
    fn test_macro_e2e_test_multiple_args() {
        e2e_test!(test_multiple_args 250, &String::from("HelloWorld"), &vec![30, 40, 50]);
    }
}
