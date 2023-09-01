fn main() {
    let s = String::from("abc def ghi jkl mno pqr stu");
    let x = take_words(&s, 2);

    assert_eq!(x, "abc def");
}

fn take_words(text: &str, number_of_words: u32) -> &str {
    let space: u8 = b' '.into();
    let mut words_taked = 0;
    let s = &text.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == space {
            words_taked += 1;
        }

        if words_taked == number_of_words {
            return &text[..i];
        }
    }

    return &text;
}