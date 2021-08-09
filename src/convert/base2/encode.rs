// 十进制转二进制
pub fn encode10(n: isize) -> String {
    // 如果输入0，直接返回0
    if n == 0 {
        return "0".to_owned();
    }

    let mut num = n;
    // 辗转相除法
    let mut res = vec![];
    while num != 0 {
        let m = num % 2;
        num /= 2;
        res.push(m.to_string());
    }
    res.reverse();
    res.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base2_encode() {
        let num = 20;
        for i in 0..num {
            let res = encode10(i);
            println!("num={}, res={}", i, res);
            // assert_eq!(res, "11".to_owned());
        }
    }
}
