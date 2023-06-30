#[cfg(test)]

mod day06 {
    
    /**
     * Problem 12 - Highly divisible triangular number
     */
    fn divisible_triangle_number(n: u32) -> u32 {
        let mut increase = 1;
        let mut curr = 0;
        loop{
            curr += increase;
            let mut count = 0;
            for i in 1..((curr as f64).sqrt().round() as u32) {
                if curr % i == 0 {
                    count += 2;
                }
            }
            if curr % ((curr as f64).sqrt().round() as u32) == 0 {
                count += 1;
            }
            if count >= n {return curr}
            increase += 1;
        }
    }

    #[test]
    fn problem12_test() {
        assert_eq!(divisible_triangle_number(5), 28);
        assert_eq!(divisible_triangle_number(23), 630);
        assert_eq!(divisible_triangle_number(167), 1385280);
        assert_eq!(divisible_triangle_number(374), 17907120);
        assert_eq!(divisible_triangle_number(500), 76576500);
    }
}