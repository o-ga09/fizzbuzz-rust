// fizzbuzz実装
pub fn convert(arg: i32) -> String {
    if arg % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if arg % 3 == 0 {
        return "fizz".to_string();
    } else  if arg % 5 == 0 {
        return "buzz".to_string();
    }

    return arg.to_string();
}

// テストモジュール
#[cfg(test)]
mod tests {
    use crate::fizzbuzz::convert;

    #[test]
    // 3のときにfizzと出力する
    fn test_fizz() {
        let res: String  =  convert(3);
        assert_eq!(res, "fizz");
    }

    #[test]
    // 5のときにbuzzと出力する
    fn test_buzz() {
        let res: String  =  convert(5);
        assert_eq!(res, "buzz");
    }

    #[test]
    // 15のときにfizzbuzzと出力する
    fn test_fizz_buzz() {
        let res: String  =  convert(15);
        assert_eq!(res, "fizzbuzz");
    }
    #[test]
    // 1ときに1と出力する
    fn test_number() {
        let res: String  =  convert(1);
        assert_eq!(res, "1");
    }
}