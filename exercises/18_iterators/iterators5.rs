// Let's define a simple model to track Rustlings' exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try to not use imperative loops (for/while).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // My first attempt was to resort to fold (as always)
    map.iter().fold(0, |mut acc, (_, progress)| {
        if *progress == value {
            acc += 1;
        }
        acc
    })
}

fn count_iterator_with_filter(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // But iterators have a handy count() method which we can combine with filter()
    map.iter()
        .filter(|(_, progress)| **progress == value)
        .count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // My first attempt here was a double-fold. Kinda gnarly
    collection.iter().fold(0, |mut acc, elem| {
        acc += elem.iter().fold(0, |mut acc, (_, progress)| {
            if *progress == value {
                acc += 1
            }
            acc
        });
        acc
    })
}

fn count_collection_iterator_with_reuse(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    // A more reasonable approach would reuse our count_iterator() function
    // instead of the inner fold - this is already much better!
    collection
        .iter()
        .fold(0, |acc, elem| acc + count_iterator(elem, value))
}

fn count_collection_iterator_with_sum(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    // But we can do even better using the map() and sum() methods on iterators!
    // It's one extra-line over the previous solution, but map() + sum() is arguably
    // less mental overhead than using fold()
    collection
        .iter()
        .map(|elem| count_iterator(elem, value))
        .sum()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator_with_filter(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator_with_reuse(&collection, progress_state)
            );
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator_with_sum(&collection, progress_state),
            );
        }
    }
}
