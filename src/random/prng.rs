// lcg-线性同余法
// RandSeed = (A * RandSeed + B) % M
// X(n+1) = (a * X(n) + c) % m
// 模m, m > 0
// 系数a, 0 < a < m
// 增量c, 0 <= c < m
// 原始值(种子) 0 <= X(0) < m
// 其中参数c, m, a比较敏感，或者说直接影响了伪随机数产生的质量。

// 线性同余法最重要的是定义了三个整数，乘数 A、增量 B和模数 M，其中A, B, M是产生器设定的常数。
// LCG的周期最大为 M，但大部分情况都会少于M。要令LCG达到最大周期，应符合以下条件：
// 1. B,M互质；
// 2. M的所有质因数都能整除A-1；
// 3. 若M是4的倍数，A-1也是；
// 4. A,B,N[0]都比M小；
// 5. A,B是正整数。

// use chrono::prelude::*;

pub fn lcg(n: i64) {
    const A: i64 = 1103515245;
    const C: i64 = 12345;
    const M: i64 = 1 << 31;
    const L: i64 = 2^32;
    println!("m={}", M);
    println!("l={}", L);

    let mut x = 12;
    for _ in 0..n {
        x = (A * x + C) % M;
        print!("{},", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcg() {
        lcg(50);
    }
}
