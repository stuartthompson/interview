fn main() {
    println!("Run cargo test to verify the answers.")
}

/// Returns the smallest difference between pairs of values from two arrays, as
///  well as a sentence describing why the pair was picked.
/// 
/// # Arguments
/// 
/// * `first` The first array to select from.
/// * `second` The second array to select from.
#[allow(dead_code)]
fn smallest_difference(first: &[u8], second: &[u8]) -> (u8, u8, String) {
    let mut smallest: u8 = 255;
    let mut f_res: u8 = 0;
    let mut s_res: u8 = 0;
    let mut result: String = String::from("");
    for f in first {
        for s in second {
            if f >= s {
                let i = f - s;
                if i < smallest {
                    smallest = i;
                    f_res = *f;
                    s_res = *s;
                    result = format!(
                        "The answer is {0} and {1} because {0} - {1} = {2}", 
                        f, s, i);
                }
            } else if s >= f {
                let i = s - f;
                if i < smallest {
                    smallest = i;
                    f_res = *f;
                    s_res = *s;
                    result = format!("{} - {} = {}", s, f, i);
                }
            } 
        }
    }

    return (f_res, s_res, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_vowels_lowercase() {
        let first = [1, 3, 5, 22];
        let second = [11, 16, 19, 26];
        
        let (f, s, result) = smallest_difference(&first, &second);
        assert_eq!(f, 22);
        assert_eq!(s, 19);
        assert_eq!(result, "The answer is 22 and 19 because 22 - 19 = 3");
    }
}