#![allow(unused_imports)]
extern crate num;
use num::Num;
use std::fmt::{Debug, Display};

fn main() {
    println!("Recursion sample by rust");
}

fn sum<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num,
{
    _match_empty_or(&xs, &|| T::zero(), &|y, ys| y + sum(ys))
}

fn length<T>(xs: &Vec<T>) -> i32
where
    T: Copy,
{
    _match_empty_or(&xs, &|| 0, &|_, ys| 1 + length(ys))
}

fn max<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num + PartialOrd,
{
    _match_single_or(&xs, &|y| y, &|y, ys| {
        let ret = max(ys);
        if y > ret {
            y
        } else {
            ret
        }
    })
}

fn min<T>(xs: &Vec<T>) -> T
where
    T: Copy + Num + PartialOrd,
{
    _match_single_or(&xs, &|y| y, &|y, ys| {
        let ret = min(ys);
        if y < ret {
            y
        } else {
            ret
        }
    })
}

fn for_all<T, F>(xs: &Vec<T>, pred: &F) -> bool
where
    T: Copy,
    F: Fn(T) -> bool,
{
    _match_empty_or(&xs, &|| true, &|y, ys| pred(y) && for_all(&ys, pred))
}

fn exists<T, F>(xs: &Vec<T>, pred: &F) -> bool
where
    T: Copy,
    F: Fn(T) -> bool,
{
    _match_empty_or(&xs, &|| false, &|y, ys| pred(y) || exists(&ys, pred))
}

fn find<T, F>(xs: &Vec<T>, pred: &F) -> Option<T>
where
    T: Copy,
    F: Fn(T) -> bool,
{
    _match_empty_or(&xs, &|| None, &|y, ys| {
        if pred(y) {
            Some(y)
        } else {
            find(ys, pred)
        }
    })
}

fn skip<T>(xs: &Vec<T>, n: i32) -> Vec<T>
where
    T: Copy,
{
    let len = xs.len();
    if len <= 0 {
        Vec::new()
    } else if n <= 0 {
        xs.clone()
    } else {
        let (_, tail) = head_tail(&xs);
        skip(&tail, n - 1)
    }
}

fn skip_while<T, F>(xs: &Vec<T>, pred: &F) -> Vec<T>
where
    T: Copy,
    F: Fn(T) -> bool,
{
    if xs.len() <= 0 {
        Vec::new()
    } else {
        let (head, tail) = head_tail(&xs);
        if !pred(head) {
            xs.clone()
        } else {
            skip_while(&tail, pred)
        }
    }
}

fn take<T>(xs: &Vec<T>, n: i32) -> Vec<T>
where
    T: Copy,
{
    let len = xs.len();
    if len <= 0 {
        Vec::new()
    } else if n <= 0 {
        Vec::new()
    } else {
        let (head, tail) = head_tail(&xs);
        return cons(head, &take(&tail, n - 1));
    }
}

fn take_while<T, F>(xs: &Vec<T>, pred: &F) -> Vec<T>
where
    T: Copy,
    F: Fn(T) -> bool,
{
    let len = xs.len();
    if len <= 0 {
        Vec::new()
    } else {
        let (head, tail) = head_tail(&xs);
        if !pred(head) {
            Vec::new()
        } else {
            return cons(head, &take_while(&tail, pred));
        }
    }
}

fn map<T, U, F>(xs: &Vec<T>, pred: &F) -> Vec<U>
where
    T: Copy,
    U: Copy,
    F: Fn(T) -> U,
{
    _match_empty_or(&xs, &|| Vec::new(), &|y, ys| cons(pred(y), &map(ys, pred)))
}

fn collect<T, U, F>(xs: &Vec<T>, pred: &F) -> Vec<U>
where
    T: Copy,
    U: Copy,
    F: Fn(T) -> Vec<U>,
{
    _match_empty_or(&xs, &|| Vec::new(), &|y, ys| {
        append(&pred(y), &collect(ys, pred))
    })
}

fn append<T>(xs: &Vec<T>, ys: &Vec<T>) -> Vec<T>
where
    T: Copy,
{
    let mut ret = Vec::with_capacity(xs.len() + ys.len());
    for e in xs {
        ret.push(*e);
    }
    for e in ys {
        ret.push(*e);
    }
    ret
}

fn partition<T, F>(xs: &Vec<T>, pred: &F) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    F: Fn(T) -> bool,
{
    _match_empty_or(&xs, &|| (Vec::new(), Vec::new()), &|y, ys| {
        let t_f = partition(&ys, pred);
        return if pred(y) {
            (cons(y, &t_f.0), t_f.1)
        } else {
            (t_f.0, cons(y, &t_f.1))
        };
    })
}

fn _match_empty_or<T, U, F1, F2>(xs: &Vec<T>, empty_case: &F1, not_empty_case: &F2) -> U
where
    T: Copy,
    F1: Fn() -> U,
    F2: Fn(T, &Vec<T>) -> U,
{
    if xs.len() == 0 {
        empty_case()
    } else {
        let (head, tail) = head_tail(&xs);
        not_empty_case(head, &tail)
    }
}

fn _match_single_or<T, U, F1, F2>(xs: &Vec<T>, single_case: &F1, multiple_case: &F2) -> U
where
    T: Copy,
    F1: Fn(T) -> U,
    F2: Fn(T, &Vec<T>) -> U,
{
    let len = xs.len();
    match len {
        l if l == 1 => single_case(xs[0]),
        l if l > 1 => {
            let (head, tail) = head_tail(&xs);
            multiple_case(head, &tail)
        }
        _ => panic!("Collection length was zero."),
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

fn cons<T>(x: T, xs: &Vec<T>) -> Vec<T>
where
    T: Copy,
{
    let len = xs.len() + 1;
    let mut ret: Vec<T> = Vec::with_capacity(len);
    ret.push(x);
    for v in xs {
        ret.push(*v);
    }
    ret
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
            let result = for_all(&xs, &|x| x % 2 == 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, 10, 0, 2, -4];
            let result = for_all(&xs, &|x| x % 2 == 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, -1, 0, 2, -4];
            let result = for_all(&xs, &|x| x % 2 == 0);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn test_exists() {
        {
            let xs: Vec<i32> = vec![];
            let result = exists(&xs, &|x| x % 2 != 0);
            assert_eq!(result, false);
        }
        {
            let xs: Vec<i32> = vec![8, -1, 0, 2, -4];
            let result = exists(&xs, &|x| x % 2 != 0);
            assert_eq!(result, true);
        }
        {
            let xs: Vec<i32> = vec![8, 10, 0, 2, -4];
            let result = exists(&xs, &|x| x % 2 != 0);
            assert_eq!(result, false);
        }
    }

    #[test]
    fn test_find() {
        {
            let xs: Vec<i32> = vec![];
            let ret = find(&xs, &|x| x % 2 == 0);
            assert_eq!(ret, None);
        }
        {
            let xs: Vec<i32> = vec![-1, 3, 7, 0, 5];
            let ret = find(&xs, &|x| x % 2 == 0);
            assert_eq!(ret, Some(0));
        }
        {
            let xs: Vec<i32> = vec![-1, 3, 7, -3, 5];
            let ret = find(&xs, &|x| x % 2 == 0);
            assert_eq!(ret, None);
        }
    }

    #[test]
    fn test_skip() {
        {
            let xs: Vec<i32> = vec![];
            let ret = skip(&xs, 10);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            assert_eq!(skip(&xs, -1).len(), 5);
            assert_eq!(skip(&xs, 0).len(), 5);
            assert_eq!(skip(&xs, 4).len(), 1);
            assert_eq!(skip(&xs, 5).len(), 0);
            assert_eq!(skip(&xs, 6).len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = skip(&xs, 1);
            assert_eq!(ret.len(), 4);
            assert_eq!(ret[0], 1);
        }
    }

    #[test]
    fn test_skip_while() {
        {
            let xs: Vec<i32> = vec![];
            let ret = skip_while(&xs, &|x| x % 2 == 0);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = skip_while(&xs, &|x| x < 5);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = skip_while(&xs, &|x| x >= 5);
            assert_eq!(ret.len(), 5);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = skip_while(&xs, &|x| x <= 2);
            assert_eq!(ret.len(), 2);
            assert_eq!(ret[0], 3);
        }
    }

    #[test]
    fn test_take() {
        {
            let xs: Vec<i32> = vec![];
            let ret = take(&xs, 10);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2];
            assert_eq!(take(&xs, 4).len(), 3);
            assert_eq!(take(&xs, 3).len(), 3);
            assert_eq!(take(&xs, 2).len(), 2);
            assert_eq!(take(&xs, 1).len(), 1);
            assert_eq!(take(&xs, 0).len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = take(&xs, 2);
            assert_eq!(ret.len(), 2);
            assert_eq!(ret[1], 1);
        }
    }

    #[test]
    fn test_take_while() {
        {
            let xs: Vec<i32> = vec![];
            let ret = take_while(&xs, &|x| x % 2 == 0);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = take_while(&xs, &|x| x > 4);
            assert_eq!(ret.len(), 0);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = take_while(&xs, &|x| x <= 4);
            assert_eq!(ret.len(), 5);
        }
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let ret = take_while(&xs, &|x| x <= 2);
            assert_eq!(ret.len(), 3);
            assert_eq!(ret[2], 2);
        }
    }

    #[test]
    fn test_map() {
        {
            let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
            let mapped: Vec<i32> = map(&xs, &|x| x * 2);
            assert_eq!(mapped.len(), 5);
            assert_eq!(mapped[0], 0);
            assert_eq!(mapped[1], 2);
            assert_eq!(mapped[4], 8);
        }
        {
            let xs: Vec<f64> = vec![0.0, 1.3, 2.0, 3.5, 4.2];
            let mapped: Vec<i32> = map(&xs, &|x| x as i32);
            assert_eq!(mapped.len(), 5);
            assert_eq!(mapped[1], 1);
            assert_eq!(mapped[4], 4);
        }
    }

    #[test]
    fn test_collect() {
        {
            let xs: Vec<u32> = vec![];
            let collected: Vec<u32> = collect(&xs, &|_| vec![]);
            assert_eq!(collected.len(), 0);
        }
        {
            let xs: Vec<u32> = vec![1, 2, 5];
            let collected = collect(&xs, &|x| {
                let capacity = usize::try_from(x).unwrap();
                let mut v = Vec::with_capacity(capacity);
                for i in 0..=x {
                    v.push(i);
                }
                v
            });
            assert_eq!(collected.len(), (1 + 1) + (2 + 1) + (5 + 1));
            assert_eq!(collected[0], 0);
            assert_eq!(collected[1], 1);
            assert_eq!(collected[5], 0);
            assert_eq!(collected[10], 5);
        }
        {
            // こういう風に､組み合わせを作れるのがcollectの使い所
            let values = vec![0, 1, 2];
            let bools = vec![true, false];
            let collected = collect(&values, &|x| {
                return collect(&bools, &|b| {
                    return vec![(x, b)];
                });
            });
            assert_eq!(collected.len(), values.len() * bools.len());
            assert_eq!(collected[0].0, 0);
            assert_eq!(collected[0].1, true);
            assert_eq!(collected[5].0, 2);
            assert_eq!(collected[5].1, false);
        }
    }

    #[test]
    fn test_partition() {
        {
            let xs:Vec<i32> = vec![];
            let t_f = partition(&xs, &|_| true);
            assert_eq!(t_f.0.len(), 0);
            assert_eq!(t_f.1.len(), 0);
        }
        {
            let xs = vec![0, 1, 2];
            let t_f = partition(&xs, &|_| true);
            assert_eq!(t_f.0.len(), 3);
            assert_eq!(t_f.1.len(), 0);
        }
        {
            let xs = vec![0, 1, 2];
            let t_f = partition(&xs, &|_| false);
            assert_eq!(t_f.0.len(), 0);
            assert_eq!(t_f.1.len(), 3);
        }
        {
            let xs = vec![0, -3, 2, 2, -5];
            let t_f = partition(&xs, &|x| x >= 0);
            assert_eq!(t_f.0.len(), 3);
            assert_eq!(t_f.1.len(), 2);
        }
    }
}
