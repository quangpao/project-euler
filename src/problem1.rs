/**
 * Problem 1 - Multiples of 3 and 5
 */
#[cfg(test)]
mod day01 {
    fn multiples_of_3_and_5(number: u32) -> u32 {
        let mut result = 0;
        for i in 1..number {
            if i % 3 == 0 || i % 5 == 0 {
                result += i;
            }
        }
        result
    }
    
    #[test]
    fn main() {
        assert_eq!(multiples_of_3_and_5(10), 23);
        assert_eq!(multiples_of_3_and_5(49),543);
        assert_eq!(multiples_of_3_and_5(1000),233168);
        assert_eq!(multiples_of_3_and_5(8456),16687353);
        assert_eq!(multiples_of_3_and_5(19564),89301183);
    }
}