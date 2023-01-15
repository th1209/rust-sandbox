fn main() {
    let xs: Vec<i32> = vec![0, 1, 2, 3, 4];
    let sum = sum(&xs);
    println!("sum:{}", sum);
}


fn sum(xs: &Vec<i32>) -> i32 {
    if xs.len() == 0 {
        0
    } else {
        let (head, tail) = head_tail(&xs);
        head + sum(&tail)
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
    }
}
