// 汉明距离（Hamming distance）是指两个相同长度的序列在相同位置上有多少个符号不同，对二进位序列来说就是“相异的比特数目”
// 复杂度为 O(n)，空间复杂度为 O(1)

pub fn hamming_distance_num(num1: usize, num2: usize) -> u32 {
    let xor = num1 ^ num2;
    println!("xor {xor:b}");
    xor.count_ones() // count_ones返回二进制内有多少个1
}


// 手动求出二进制内有多少个1
pub fn hamming_distance_num2(num1: usize, num2: usize) -> u32 {
    let mut xor = num1 ^ num2;
    let mut cnt = 0;
    while xor != 0 {
        cnt += xor & 1;
        xor >>= 1;
    }
    cnt as u32
}

pub fn hamming_distance_str(str1: &str, str2: &str) -> u32 {
    let mut cnt = 0;
    let mut str1 = str1.chars();
    let mut str2=  str2.chars();
    loop {
        match (str1.next(), str2.next()) {
            (Some(s1), Some(s2)) => {
                if s1 != s2 {
                    cnt += 1
                }
            },
            (Some(_), None) | (None, Some(_)) => {
                panic!("str1 and str2 need have same length");
            },
            (None, None )=> break,
        }
    }

    cnt as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_num() {
        let num1 = 100;
        let num2 = 200;
        let res = hamming_distance_num(num1, num2);
        println!("res {}", res);
        let res2 = hamming_distance_num2(num1, num2);
        assert_eq!(res, res2)
    }

    #[test]
    fn hamming_str() {
        let a = "abc";
        let b = "dbc";
        let res = hamming_distance_str(a, b);
        println!("str {res}")
    }
}