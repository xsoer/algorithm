use std::ops::{Div, Sub};

// 顺序查找
pub fn sequence_search<T>(arr: &Vec<T>, value: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    for i in 0..arr.len() {
        if arr[i] == value {
            return Some(i);
        }
    }
    None
}

// 二分查找
pub fn binary_search<T>(arr: &Vec<T>, value: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;

        if arr[mid] == value {
            return Some(mid);
        }
        if arr[mid] < value {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

// 插值查找
// 不太靠谱
pub fn insertion_search(arr: &Vec<usize>, value: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = low + (value - arr[low]) / (arr[high] - arr[low]) * (high - low);

        if arr[mid] == value {
            return Some(mid);
        } else if arr[mid] < value {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

mod tests {
    use super::*;

    #[test]
    fn test_sequence_search() {
        let arr = vec![1, 2, 5, 7, 9, 10];
        assert_eq!(sequence_search(&arr, 10), Some(5));
        assert_eq!(sequence_search(&arr, 11), None);
        assert_eq!(sequence_search(&arr, 1), Some(0));

        let arr1 = vec!['a', 'b', 'd', 'g', 'k'];
        assert_eq!(sequence_search(&arr1, 'b'), Some(1));
        assert_eq!(sequence_search(&arr1, 'k'), Some(4));
        assert_eq!(sequence_search(&arr1, 'z'), None);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 5, 7, 9, 10];
        assert_eq!(binary_search(&arr, 10), Some(5));
        assert_eq!(binary_search(&arr, 11), None);
        assert_eq!(binary_search(&arr, 1), Some(0));

        let arr1 = vec!['a', 'b', 'd', 'g', 'k'];
        assert_eq!(binary_search(&arr1, 'b'), Some(1));
        assert_eq!(binary_search(&arr1, 'k'), Some(4));
        assert_eq!(binary_search(&arr1, 'z'), None);
    }

    #[test]
    fn test_insertion_search() {
        let arr = vec![1, 2, 5, 13, 7, 30, 9, 10];
        assert_eq!(insertion_search(&arr, 10), Some(7));
        assert_eq!(insertion_search(&arr, 11), None);
        assert_eq!(insertion_search(&arr, 1), Some(0));
        assert_eq!(insertion_search(&arr, 13), Some(3));
    }
}
