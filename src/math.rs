// 最大公约数(greatest common divisor)
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    let temp = a % b;
    gcd(b, temp)
}

// 最小公倍数(least common multiple)
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

// Fibonacci
// 求第n个数
pub fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

// 翻转整数

pub fn reverse_int(n: isize) -> isize {}

mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let (a1, b1) = (1, 0);
        let res1 = gcd(a1, b1);
        assert_eq!(res1, 1);

        let (a2, b2) = (10, 5);
        let res2 = gcd(a2, b2);
        assert_eq!(res2, 5);

        let (a3, b3) = (11, 17);
        let res3 = gcd(a3, b3);
        assert_eq!(res3, 1);
    }

    #[test]
    fn test_lcm() {
        let (a1, b1) = (1, 0);
        let res1 = lcm(a1, b1);
        assert_eq!(res1, 0);

        let (a2, b2) = (10, 5);
        let res2 = lcm(a2, b2);
        assert_eq!(res2, 10);

        let (a3, b3) = (3, 17);
        let res3 = lcm(a3, b3);
        assert_eq!(res3, 51);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(3), 2);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(12), 144);
    }
}
