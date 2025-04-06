// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let first_char = match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    };
    first_char + chars.as_str()
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn capitalize_words_string(words: &[&str]) -> String {
    // Instead of collect I previously used fold()
    // But apparently collect is very powerful!
    words.iter().map(|word| capitalize_first(word)).collect()
    // .fold(String::new(), |mut acc, elem| {
    //     acc.push_str(&elem);
    //     acc
    // })
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
