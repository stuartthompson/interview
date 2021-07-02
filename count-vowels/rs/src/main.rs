fn main() {
   println!("Run cargo test to verify the answers.");
}

/// Returns the number of vowels within a string.
/// 
/// # Arguments
/// 
/// `s` - The string containing the vowels to count.
#[allow(dead_code)]
fn count_vowels(s: String) -> u32 {
    let mut count: u32 = 0;
    for c in s.to_lowercase().chars() {
        if is_vowel(c) {
            count += 1;
        }
    }

    return count;
}

/// Returns a value indicating if a supplied character is a vowel.
/// Returns true if a character is a vowel, false otherwise.
/// 
/// # Arguments
/// 
/// * `c` - The character to test.
#[allow(dead_code)]
fn is_vowel(c: char) -> bool {
    return
        c == 'a' ||
        c == 'e' ||
        c == 'i' ||
        c == 'o' ||
        c == 'u';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_vowels_lowercase() {
        let str = String::from("This string contains 7 vowels.");
        let count = count_vowels(str);

        assert_eq!(7, count);
    }

    #[test]
    fn count_vowels_uppercase() {
        let str = String::from("THIS STRING CONTAINS 7 VOWELS.");
        let count = count_vowels(str);

        assert_eq!(7, count);
    }

    #[test]
    fn count_vowels_mixed_case() {
        let str = String::from("tHiS sTrInG cOnTaInS 7 vOwElS.");
        let count = count_vowels(str);

        assert_eq!(7, count);
    }
}