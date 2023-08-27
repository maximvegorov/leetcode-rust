pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if str1.is_empty() || str2.is_empty() {
        return String::new();
    }

    let len = gcd(str1.len(), str2.len());

    let f1 = &str1[..len];
    let f2 = &str2[..len];

    if !is_factor(&str1, f1)
        || !is_factor(&str2, f2)
        || f1 != f2 {
        return String::new()
    }

    f1.to_string()
}

fn is_factor(s: &str, r: &str) -> bool {
    let mut i = 0;
    while i < s.len() {
        let t = &s[i..i + r.len()];
        if t != r {
            return false;
        }
        i += r.len();
    }
    true
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
