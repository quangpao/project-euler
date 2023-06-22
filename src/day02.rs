#[cfg(test)]
mod day02 {


    /**
     * Problem 3 - Largest prime factor
     */
    fn largest_prime_factor(mut number: u64) -> u64 {
        if is_prime(number) {return number}

        let mut max: u64 = 2;
        let mut iter: u64 = 2;
        loop {
            if !is_prime(iter) {iter += 1;continue}
            if number % iter == 0 {
                number /= iter;
                max = max.max(iter);
                continue
            }
            iter += 1;
            if iter > number {return max}
        }
    }

    fn is_prime(number: u64) -> bool {
        let mut i: u64 = 2;
        loop {
            if i * i > number {break}
            if number % i == 0 {return false}
            i += 1;
        }
        true
    }

    //---------------------------------------------------------//
    /**
     * Problem 4 - Largest palindrome product
     */
    fn largest_palindrome_product(number : i32) -> i32 {
        let max = 10_u64.pow(number as u32);
        let min = max - 10_u64.pow(((number + 1) >> 1) as u32);
        fn compare(number1: u64, number2: u64) {
            
        }
       

        1 as i32
    }

    #[test]
    fn problem03_test() {
        assert_eq!(largest_prime_factor(13195), 29);
        assert_eq!(largest_prime_factor(600851475143), 6857);
        assert_eq!(largest_prime_factor(8), 2);
        assert_eq!(largest_prime_factor(7), 7);

    }

    #[test]
    fn problem04_test() {
        assert_eq!(largest_palindrome_product(4), 1);
    }
}