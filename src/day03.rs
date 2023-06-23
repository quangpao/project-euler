#[cfg(test)]
mod day03 {
    use std::collections::HashMap;


    /**
     * Problem 5 - Smallest multiple
     */
    fn smallest_mult(number: u32) -> u64 {
        let mut map: HashMap<u32, u32> = HashMap::new();
        for i in (2..=number).rev() {
            if map.contains_key(&i) {continue;}
            let factor_map = factor(i);
            for (key, &values) in factor_map.iter() {
                if !map.contains_key(key) || (map.contains_key(key) && map.get(key).unwrap() < &values) {
                    map.insert(*key, values);
                }
            }
        }
        sum_up(map)
    }

    fn factor(number: u32) -> HashMap<u32,u32> {
        let mut number_cln = number;
        let mut result: HashMap<u32, u32> = HashMap::new();
        let mut iter = 2;

        while number_cln > 1 {  
            if number_cln % iter == 0 {
                if !result.contains_key(&iter) {
                    result.insert(iter, 1);
                } else {
                    result.insert(iter, result.get(&iter).unwrap() + 1);
                }
                number_cln /= iter;
                continue;
            }
            iter += 1
        }
        result
    }

    fn sum_up(map: HashMap<u32, u32>) -> u64 {
        let mut result = 1;
        for (&key, &values) in map.iter() {
            result *= key.pow(values);
        }
        result as u64
    }

    #[test]
    fn problem05_test() {
        assert_eq!(smallest_mult(5),60);
        assert_eq!(smallest_mult(7),420);
        assert_eq!(smallest_mult(10),2520);
        assert_eq!(smallest_mult(13),360360);
        assert_eq!(smallest_mult(20),232792560);
    }
}