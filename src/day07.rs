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

    /**
     * Problem 15 - Lattice paths
     */
    fn lattice_paths(grid_size: u64) -> u64 {
        let actually_size = (grid_size + 1) as usize;
        let mut vec: Vec<u64> = vec![0; actually_size * actually_size + 1];
        for i in 1..=actually_size {
            vec[i] = 1;
            vec[(i - 1) * actually_size + 1] = 1
        }
        for i in 2..=actually_size {
            for j in 2..=actually_size {
                vec[(i - 1) * actually_size + j] = vec[(i - 1) * actually_size + j - 1] + vec[(i - 2) * actually_size + j];
            }
        }
        *vec.get(actually_size * actually_size).unwrap()
    }

    /**
     * LeetCode 62 - Unique Paths (Similar to Problem 15)
     * https://leetcode.com/problems/unique-paths/
     */
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
            return 0;
        }
        
        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut vec: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    vec[i][j] = 1;
                } else {
                    vec[i][j] = vec[i - 1][j] + vec[i][j - 1];
                }
            }
        }
        vec[m - 1][n - 1]
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

    #[test]
    fn problem15_test() {
        let start = Instant::now();
        assert_eq!(lattice_paths(4), 70);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(lattice_paths(9), 48620);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(lattice_paths(20), 137846528820);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(lattice_paths(14), 40116600);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());

        assert_eq!(unique_paths(3, 2), 3);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(unique_paths(7, 3), 28);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(unique_paths(23, 12), 193536720);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(unique_paths(51, 9), 1916797311);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());
        assert_eq!(unique_paths(0, 0), 0);
        println!("{}.{:03} seconds elapsed.", start.elapsed().as_secs(), start.elapsed().subsec_millis());

    }

}