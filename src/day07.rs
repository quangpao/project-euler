#[cfg(test)]

mod day07 {
    use std::{collections::HashMap, time::Instant};


    /**
     * Problem 14 - Longest Collatz sequence
     */
    fn longest_collatz_sequence(limit: u64) -> u64 {
        let mut map: HashMap<u64, u64> = HashMap::from([(1, 1), (2, 2)]);
        let mut result = 1;
        let mut longest = 1;
        for i in 3..limit {
            if map.get(&i).is_none() {
                let mut vec: Vec<u64> = Vec::new();
                let mut curr = i;
                let mut count: u64 = 1;
                while map.get(&curr).is_none() {
                    if curr % 2 == 1 {
                        curr = 3 * curr + 1;
                    } else {
                        curr /= 2;
                    }
                    vec.push(curr);
                    count += 1;
                    if map.get(&curr).is_some() {
                        map.insert(i, map.get(&curr).unwrap() + count - 1);
                        for j in 0..(vec.len()-1) as u64 {
                            map.insert(vec[j as usize], map.get(&i).unwrap() - 1 - j);
                        }
                    }
                }
                if longest < *map.get(&i).unwrap() {
                    longest = *map.get(&i).unwrap();
                    result = i;
                }
            }
        }
        
        result
    }

    #[test]
    fn problem14_test() {
        let start = Instant::now();
        assert_eq!(longest_collatz_sequence(14), 9);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(longest_collatz_sequence(5847), 3711);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(longest_collatz_sequence(46500), 35655);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(longest_collatz_sequence(54512), 52527);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(longest_collatz_sequence(100000), 77031);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(longest_collatz_sequence(1000000), 837799);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
    }

}