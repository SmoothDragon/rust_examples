use std::collections::HashMap;

pub fn counter_char(letters: &str) -> HashMap::<char, u64> {
    // Implementation similar to Python Collections.Counter
    let mut char_counts = HashMap::<char, u64>::new();
    for ch in letters.chars() {
        char_counts.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    char_counts
}

pub fn max_over_counters(arr_letters: Vec<&str>) -> HashMap::<char, u64> {
    // Find the maximum value for each key over multiple HashMaps
    let mut max_counts = HashMap::<char, u64>::new();
    for letters in arr_letters {
        for (key, value) in counter_char(letters).into_iter() {
            let _ = *max_counts.entry(key).and_modify(|max| *max = std::cmp::max(*max, value)).or_insert(value);
        }
    }

    max_counts
}

#[cfg(test)]
pub mod test {
    use super::counter_char;
    use super::max_over_counters;

    #[test]
    fn counter() {
        let counter = counter_char(&"abracadabra");
        println!("{:?}", counter);
        assert_eq!(counter[&'a'], 5);
        assert_eq!(counter[&'b'], 2);
        assert_eq!(counter[&'c'], 1);
        assert_eq!(counter[&'d'], 1);
        assert_eq!(counter[&'r'], 2);
    }

    #[test]
    fn max_of_counters() {
        let max_counter = max_over_counters(vec!(&"abracadabra", &"jackbox"));
        println!("{:?}", max_counter);
        assert_eq!(max_counter[&'x'], 1);
        assert_eq!(max_counter[&'a'], 5);
    }
}

