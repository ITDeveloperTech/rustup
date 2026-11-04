fn main() {
    println!("Привет мир");

    let s1 = String::from("Something string");
    let (s2, len) = calculate_length(s1);
    println!("length string = {len}");

    let len = calculate_length_single(&s2);
    println!("len {len} string {s2}");

    let mut s = String::from("Now");
    change_str(&mut s);
    println!("{s}");

    let s1 = String::from("It's cold");
    let s2 = String::from("NoSpace");
    let len1 = search_space(&s1);
    let len2 = search_space(&s2);
    println!("Results: {len1}-{len2}");

    let s = String::from("It's a multi words");
    let mut i: usize = 0;
    let mut word: &str = &s[..1];
    while word.len() != s.len() {
        word = get_word(&s, i);
        i += 1;
        println!("{word}");
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_single(s: &String) -> usize {
    s.len()
}

fn change_str(s: &mut String)
{
    s.push_str(" Take this");
}

fn search_space(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_word(s: &String, word_i: usize) -> &str {
    let bytes = s.as_bytes();
    let mut word_index = 0;
    let mut last_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if word_index == word_i {
                return &s[last_index..i];
            }
            last_index = i + 1;
            word_index += 1;
        }
    }

    if word_index == word_i {
        return &s[last_index..];
    }

    &s[..]
}
