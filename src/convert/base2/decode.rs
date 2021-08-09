// 二进制转化成十进制
pub fn decode10(b: String) -> isize {
    if b.is_empty() || b == "0".to_owned() {
        return 0;
    }

    let mut b_num: Vec<char> = b.chars().collect();
    let mut num: isize = 0;
    b_num.reverse();
    for (i, v) in b_num.into_iter().enumerate() {
        num += 2isize.pow(i as u32) * v.to_digit(10).unwrap() as isize;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode10() {
        let nums = [
            ("".to_owned(), 0),
            ("0".to_owned(), 0),
            ("1000".to_owned(), 8),
            ("101".to_owned(), 5),
            ("10001".to_owned(), 17),
        ];
        for n in nums.iter() {
            let res = decode10(n.0.clone());
            println!("num={}, res={}", n.0, res);
            assert_eq!(res, n.1);
        }
    }
}
