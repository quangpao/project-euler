#[cfg(test)]

mod day05 {

    /**
     * Problem 10 - Summation of primes
     */
    fn prime_summation(number: u32) -> u64 {
        let mut vec = vec![0; number as usize];
        let mut result = 0;
        for i in 2..vec.len() {
            if vec[i] == 0 {
                result += i;
                for j in ((i*2)..vec.len()).step_by(i) {
                    vec[j] = 1
                }
            }
        }
        result as u64
    }

    #[test]
    fn problem10_test() {
        assert_eq!(prime_summation(17), 41);
        assert_eq!(prime_summation(2001), 277050);
        assert_eq!(prime_summation(140759), 873608362);
        assert_eq!(prime_summation(2000000), 142913828922);
    }

}