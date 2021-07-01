fn main() {

    println!("Calculating the smallest positive difference between one value from each of two arrays.");

    let f = [1, 3, 5, 22];
    let s = [11, 16, 19, 26];

    println!("Array 1 is: {:?}", f);
    println!("Array 2 is: {:?}", s);

    let (first, second, result) = smallest_difference(&f, &s);
    println!("The answer is {} and {} because {}", first, second, result);
}

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
                    result = format!("{} - {} = {}", f, s, i);
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
