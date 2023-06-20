#[cfg(test)]
mod day01 {
    /**
     * Problem 1 - Multiples of 3 and 5
     */
    fn multiples_of_3_and_5(number: u32) -> u32 {
        let mut result = 0;
        for i in 1..number {
            if i % 3 == 0 || i % 5 == 0 {
                result += i;
            }
        }
        result
    }
    
    /**
     * Problem 2 - Even Fibonacci numbers
     */
    fn fibo_even_sum(number: u32) -> u32 {
        let mut first_num: u32;
        let mut second_num: u32 = 1;
        let mut current = 2;
        let mut result = 0;
        loop {
            if current > number {
                return result;
            }
            if current % 2 == 0 {
                result += current;
            }
            first_num = second_num;
            second_num = current;
            current = first_num + second_num;

        }
    }


    #[test]
    fn problem01_test() {
        assert_eq!(multiples_of_3_and_5(10), 23);
        assert_eq!(multiples_of_3_and_5(49),543);
        assert_eq!(multiples_of_3_and_5(1000),233168);
        assert_eq!(multiples_of_3_and_5(8456),16687353);
        assert_eq!(multiples_of_3_and_5(19564),89301183);
    }

    #[test]
    fn problem02_test() {
        assert_eq!(fibo_even_sum(8), 10);
        assert!(fibo_even_sum(7) % 2 == 0);
        assert_eq!(fibo_even_sum(10), 10);
        assert_eq!(fibo_even_sum(34), 44);
        assert_eq!(fibo_even_sum(60), 44);
        assert_eq!(fibo_even_sum(1000), 798);
        assert_eq!(fibo_even_sum(100000), 60696);
        assert_eq!(fibo_even_sum(4000000), 4613732);
    }
}