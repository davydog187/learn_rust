mod stats {
    use std::collections::HashMap;

    fn mean(list: &Vec<i32>) -> f32 {
        let len: f32 = list.len() as f32;

        let mut accum: f32 = 0.0;

        for num in list {
            accum += *num as f32;
        }

        return accum / len;
    }

    fn median(list: &Vec<i32>) -> i32 {
        let mut list = list.clone();
        list.sort();
        let len = list.len();

        if len == 0 {
            return 0;
        }

        return list[len / 2];
    }

    fn mode(list: &Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut max_key = std::i32::MIN;
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

}
