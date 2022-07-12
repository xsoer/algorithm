pub fn subfn() -> i32 {
    let mut a = 1;
    // fn add(i: i32) -> i32 {
    //     a + i
    // }  can't capture dynamic environment in a fn item.  use the `|| { ... }` closure form instead

    // 闭包可以传入外部遍历，而函数不行
    let mut add = |i: i32| -> i32 {
        a += i;
        a
    };
    add(10)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn t_subfn() {
        let a = subfn();
        println!("subfn {}", a);
    }
}