use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f32 {
    let len: f32 = list.len() as f32;

    let mut accum: f32 = 0.0;

    for num in list {
        accum += *num as f32;
    }

    return accum / len;
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut list = list.clone();
    list.sort();
    let len = list.len();

    if len == 0 {
        return 0;
    }

    return list[len / 2];
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut max_key = <i32>::min_value();
    let mut max_val = 0;

    for &num in list {
        let count = counts.entry(num).or_insert(0);
        *count += 1;

        if *count > max_val {
            max_key = num;
            max_val = *count;
        }
    }

    return max_key;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
