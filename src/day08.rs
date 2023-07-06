#[cfg(test)]

mod day08 {

    /**
     * Problem 16 - Power digit sum
     */
    fn power_digit_sum(exponent: u64) -> u64 {
        let mut vec = vec![2]; //[2] => [4] => [8] => [16] => [6,1] => [12,2] => [2,3]
        for _ in 1..exponent {
            for i in 0..vec.len() {
                 vec[i] *= 2;
            }
            for i in 0..(vec.len() - 1) {
                if vec[i] >= 10 {
                    vec[i+1] += vec[i] / 10;
                    vec[i] %= 10;
                }
            }
            let len = vec.len() - 1;
            if vec[len] >= 10 {
                vec.push(vec[len] / 10);
                vec[len] %= 10;
            }
            
        }
        vec.iter().sum()
    }

    #[test]
    fn problem16_test() {
        assert_eq!(power_digit_sum(15), 26);
        assert_eq!(power_digit_sum(128), 166);
        assert_eq!(power_digit_sum(1000), 1366);
    }

}