// 冒泡排序
//
pub fn bubble_sort<T>(arr: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    let len = arr.len();
    if len < 2 {
        return;
    }
    for i in 1..len {
        let mut flag = true;
        for j in 0..(len - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = false;
            }
        }
        if flag {
            break;
        }
    }
}

// 插入排序
//
pub fn insertion_sort<T>(arr: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    let len = arr.len();
    if len < 2 {
        return;
    }

    let mut j: usize;
    let mut cur: T;

    for i in 1..len {
        cur = arr[i];
        j = i;
        while j > 0 && arr[j - 1] > cur {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
    }
}

// 选择排序
//
pub fn selection_sort<T>(arr: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    let len = arr.len();
    if len < 2 {
        return;
    }

    let mut min_index: usize;
    for i in 0..len - 1 {
        min_index = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

// 快速排序
//
pub fn quick_sort<T>(arr: &mut Vec<T>)
where
    T: PartialEq + PartialOrd + Copy,
{
    let len = arr.len();
    if len < 2 {
        return;
    }
    quick(arr, 0, len - 1);
}

fn quick<T>(arr: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialEq + PartialOrd + Copy,
{
    if left >= right {
        return;
    }

    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && arr[r] >= arr[left] {
            r -= 1;
        }
        while l < r && arr[l] <= arr[left] {
            l += 1;
        }
        arr.swap(l, r);
    }
    arr.swap(left, l);
    if l > 1 {
        quick(arr, left, l - 1);
    }

    quick(arr, r + 1, right);
}

mod tests {
    use super::*;
    type DataType<'a> = (
        Vec<usize>,
        Vec<usize>,
        Vec<i8>,
        Vec<u64>,
        Vec<usize>,
        Vec<usize>,
        Vec<f32>,
        Vec<char>,
        Vec<&'a str>,
    );

    fn data() -> DataType<'static> {
        let arr0: Vec<usize> = vec![];
        let arr1: Vec<usize> = vec![5];
        let arr2: Vec<i8> = vec![2, -5, 0];
        let arr3: Vec<u64> = vec![2341234123, 5345341232332323, 3412342343];
        let arr4: Vec<usize> = vec![2, 1, 4, 3, 6, 5];
        let arr5: Vec<usize> = vec![2, 1, 4, 3, 6, 5, 1, 3];
        let arr6: Vec<f32> = vec![1.3, 2.3, 4.2, 0.3, 6.23, 5.43, 2.32, -1.2];
        let arr7: Vec<char> = vec!['v', 'c', 'a', 'b', 'z', 'y', '1', '3'];
        let arr8: Vec<&str> = vec!["vc", "ca", "ab", "be", "zs", "ys", "cba", "ac"];
        (arr0, arr1, arr2, arr3, arr4, arr5, arr6, arr7, arr8)
    }
    #[test]
    fn test_bubble() {
        let mut arrs = data();

        println!("排序前：{:?}", arrs.0);
        bubble_sort(&mut arrs.0);
        println!("排序后：{:?}\n", arrs.0);

        println!("排序前：{:?}", arrs.1);
        bubble_sort(&mut arrs.1);
        println!("排序后：{:?}\n", arrs.1);

        println!("排序前：{:?}", arrs.2);
        bubble_sort(&mut arrs.2);
        println!("排序后：{:?}\n", arrs.2);

        println!("排序前：{:?}", arrs.3);
        bubble_sort(&mut arrs.3);
        println!("排序后：{:?}\n", arrs.3);

        println!("排序前：{:?}", arrs.4);
        bubble_sort(&mut arrs.4);
        println!("排序后：{:?}\n", arrs.4);

        println!("排序前：{:?}", arrs.5);
        bubble_sort(&mut arrs.5);
        println!("排序后：{:?}\n", arrs.5);

        println!("排序前：{:?}", arrs.6);
        bubble_sort(&mut arrs.6);
        println!("排序后：{:?}\n", arrs.6);

        println!("排序前：{:?}", arrs.7);
        bubble_sort(&mut arrs.7);
        println!("排序后：{:?}\n", arrs.7);

        println!("排序前：{:?}", arrs.8);
        bubble_sort(&mut arrs.8);
        println!("排序后：{:?}\n", arrs.8);
    }

    #[test]
    fn test_insertion() {
        let mut arrs = data();

        println!("排序前：{:?}", arrs.0);
        insertion_sort(&mut arrs.0);
        println!("排序后：{:?}\n", arrs.0);

        println!("排序前：{:?}", arrs.1);
        insertion_sort(&mut arrs.1);
        println!("排序后：{:?}\n", arrs.1);

        println!("排序前：{:?}", arrs.2);
        insertion_sort(&mut arrs.2);
        println!("排序后：{:?}\n", arrs.2);

        println!("排序前：{:?}", arrs.3);
        insertion_sort(&mut arrs.3);
        println!("排序后：{:?}\n", arrs.3);

        println!("排序前：{:?}", arrs.4);
        insertion_sort(&mut arrs.4);
        println!("排序后：{:?}\n", arrs.4);

        println!("排序前：{:?}", arrs.5);
        insertion_sort(&mut arrs.5);
        println!("排序后：{:?}\n", arrs.5);

        println!("排序前：{:?}", arrs.6);
        insertion_sort(&mut arrs.6);
        println!("排序后：{:?}\n", arrs.6);

        println!("排序前：{:?}", arrs.7);
        insertion_sort(&mut arrs.7);
        println!("排序后：{:?}\n", arrs.7);

        println!("排序前：{:?}", arrs.8);
        insertion_sort(&mut arrs.8);
        println!("排序后：{:?}\n", arrs.8);
    }

    #[test]
    fn test_selection() {
        let mut arrs = data();

        println!("排序前：{:?}", arrs.0);
        selection_sort(&mut arrs.0);
        println!("排序后：{:?}\n", arrs.0);

        println!("排序前：{:?}", arrs.1);
        selection_sort(&mut arrs.1);
        println!("排序后：{:?}\n", arrs.1);

        println!("排序前：{:?}", arrs.2);
        selection_sort(&mut arrs.2);
        println!("排序后：{:?}\n", arrs.2);

        println!("排序前：{:?}", arrs.3);
        selection_sort(&mut arrs.3);
        println!("排序后：{:?}\n", arrs.3);

        println!("排序前：{:?}", arrs.4);
        selection_sort(&mut arrs.4);
        println!("排序后：{:?}\n", arrs.4);

        println!("排序前：{:?}", arrs.5);
        selection_sort(&mut arrs.5);
        println!("排序后：{:?}\n", arrs.5);

        println!("排序前：{:?}", arrs.6);
        selection_sort(&mut arrs.6);
        println!("排序后：{:?}\n", arrs.6);

        println!("排序前：{:?}", arrs.7);
        selection_sort(&mut arrs.7);
        println!("排序后：{:?}\n", arrs.7);

        println!("排序前：{:?}", arrs.8);
        selection_sort(&mut arrs.8);
        println!("排序后：{:?}\n", arrs.8);
    }

    #[test]
    fn test_quick() {
        let mut arrs = data();

        println!("排序前：{:?}", arrs.0);
        quick_sort(&mut arrs.0);
        println!("排序后：{:?}\n", arrs.0);

        println!("排序前：{:?}", arrs.1);
        quick_sort(&mut arrs.1);
        println!("排序后：{:?}\n", arrs.1);

        println!("排序前：{:?}", arrs.2);
        quick_sort(&mut arrs.2);
        println!("排序后：{:?}\n", arrs.2);

        println!("排序前：{:?}", arrs.3);
        quick_sort(&mut arrs.3);
        println!("排序后：{:?}\n", arrs.3);

        println!("排序前：{:?}", arrs.4);
        quick_sort(&mut arrs.4);
        println!("排序后：{:?}\n", arrs.4);

        println!("排序前：{:?}", arrs.5);
        quick_sort(&mut arrs.5);
        println!("排序后：{:?}\n", arrs.5);

        println!("排序前：{:?}", arrs.6);
        quick_sort(&mut arrs.6);
        println!("排序后：{:?}\n", arrs.6);

        println!("排序前：{:?}", arrs.7);
        quick_sort(&mut arrs.7);
        println!("排序后：{:?}\n", arrs.7);

        println!("排序前：{:?}", arrs.8);
        quick_sort(&mut arrs.8);
        println!("排序后：{:?}\n", arrs.8);
    }
}
