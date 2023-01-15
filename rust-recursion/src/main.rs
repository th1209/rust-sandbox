extern crate num;
use num::Num;

fn main() {
    println!("Recursion sample by rust");
}

fn sum<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num,
{
    if xs.len() == 0 {
        T::zero()
    } else {
        let (head, tail) = head_tail(&xs);
        head + sum(&tail)
    }
}

fn length<T>(xs: &Vec<T>) -> i32
where
    T: Copy,
{
    if xs.len() == 0 {
        0
    } else {
        let (_, tail) = head_tail(&xs);
        1 + length(&tail)
    }
}

fn max<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num + PartialOrd,
{
    let len = xs.len();
    match len {
        l if l == 1 => xs[0],
        l if l > 1 => {
            let (head, tail) = head_tail(&xs);
            let ret = max(&tail);
            if head > ret {
                head
            } else {
                ret
            }
        }

        _ => panic!("Collection length was zero."),
    }
}

fn min<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num + PartialOrd,
{
    let len = xs.len();
    match len {
        l if l == 1 => xs[0],
        l if l > 1 => {
            let (head, tail) = head_tail(&xs);
            let ret = min(&tail);
            if head < ret {
                head
            } else {
                ret
            }
        }

        _ => panic!("Collection length was zero."),
    }
}

fn for_all<T, F>(xs: &Vec<T>, pred: F) -> bool
where
    T: Copy + Num + PartialOrd,
    F: Fn(T) -> bool,
{
    if xs.len() == 0 {
        true
    } else {
        let (head, tail) = head_tail(&xs);
        pred(head) && for_all(&tail, pred)
    }
}

fn exists<T, F>(xs: &Vec<T>, pred: F) -> bool
where
    T: Copy + Num + PartialOrd,
    F: Fn(T) -> bool,
{
    if xs.len() == 0 {
        false
    } else {
        let (head, tail) = head_tail(&xs);
        pred(head) || exists(&tail, pred)
    }
}

fn head_tail<T>(xs: &Vec<T>) -> (T, Vec<T>)
where
    T: Copy,
{
    fn tail<T>(xs: &Vec<T>) -> Vec<T>
    where
        T: Copy,
    {
        let len = xs.len() - 1;
        let mut tail: Vec<T> = Vec::with_capacity(len);
        for v in &xs[1..] {
            tail.push(*v);
        }
        tail
    }

    assert!(xs.len() > 0);
    (xs[0], tail(&xs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        {
            let xs: Vec<i32> = vec![];
            let sum = sum(&xs);
            assert_eq!(sum, 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let sum = sum(&xs);
            assert_eq!(sum, 10);
        }
        {
            let xs: Vec<i32> = vec![-3, -1, 0, 1, 3, 5];
            let sum = sum(&xs);
            assert_eq!(sum, 5);
        }
        {
            let xs: Vec<f64> = vec![0.0, 1.1, 2.2, 3.3, 4.4];
            let sum = sum(&xs);
            assert_eq!(sum, 11.0);
        }
    }

    #[test]
    fn test_length() {
        {
            let xs: Vec<char> = vec![];
            let length = length(&xs);
            assert_eq!(length, 0);
        }
        {
            let xs: Vec<char> = vec!['r', 'u', 's', 't'];
            let length = length(&xs);
            assert_eq!(length, 4);
        }
    }

    #[test]
    fn test_max() {
        {
            let xs: Vec<f64> = vec![4.3, -2.1, 3.9, 5.2, 2.3];
            let max = max(&xs);
            assert_eq!(max, 5.2);
        }
        {
            let xs: Vec<f64> = vec![4.2];
            let max = max(&xs);
            assert_eq!(max, 4.2);
        }
    }

    #[test]
    fn test_min() {
        {
            let xs: Vec<f64> = vec![4.3, -2.1, 3.9, 5.2, 2.3];
            let max = min(&xs);
            assert_eq!(max, -2.1);
        }
        {
            let xs: Vec<f64> = vec![4.2];
            let max = min(&xs);
            assert_eq!(max, 4.2);
        }
    }

    #[test]
    #[should_panic]
    fn test_max_error() {
        {
            let xs: Vec<f64> = vec![];
            let _ = max(&xs);
        }
    }

    #[test]
    #[should_panic]
    fn test_min_error() {
        {
            let xs: Vec<f64> = vec![];
            let _ = min(&xs);
        }
    }

    #[test]
    fn test_for_all() {
        {
            let xs: Vec<i32> = vec![];
            let result = for_all(&xs, |x| x % 2 == 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, 10, 0, 2, -4];
            let result = for_all(&xs, |x| x % 2 == 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, -1, 0, 2, -4];
            let result = for_all(&xs, |x| x % 2 == 0);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn test_exists() {
        {
            let xs: Vec<i32> = vec![];
            let result = exists(&xs, |x| x % 2 != 0);
            assert_eq!(result, false);
        }
        {
            let xs: Vec<i32> = vec![8, -1, 0, 2, -4];
            let result = exists(&xs, |x| x % 2 != 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, 10, 0, 2, -4];
            let result = exists(&xs, |x| x % 2 != 0);
            assert_eq!(result, false);
        }
    }
}
