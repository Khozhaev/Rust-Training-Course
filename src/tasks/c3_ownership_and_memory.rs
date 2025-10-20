// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.

pub fn longest_owned(s1: String, s2: String) -> String {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    let s1 = String::from("A");
    let s2 = String::from("BB");
    let result = longest_owned(s1, s2);
    //println!("s1: {}", s1);// error
    //println!("s2: {}", s2);// error
    println!("The longer string is: {}", result);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.

pub fn print_length(s: &str) {
    println!("The length of the string is: {}", s.len());
}

// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    let s = String::from("Hello");
    print_length(&s);
    println!("s: {}", s);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.

pub fn append_and_return_length(s: &mut String, suffix: &str) -> usize {
    s.push_str(suffix);
    return s.len();
}

// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut s = String::from("Hello");
    let length = append_and_return_length(&mut s, " World");
    println!("s: {}", s);
    println!("The length of the string is: {}", length);
    let length = append_and_return_length(&mut s, "!");
    println!("s: {}", s);
    println!("The length of the string is: {}", length);
}

// SLICES
// =======================================================S=========================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    let bytes = slice.as_bytes();
    let mut len = bytes.len();
    while len > 0 && bytes[len - 1] == b' ' {
        len -= 1;
    }
    let mut pos = 0;
    for i in 0..len {
        if bytes[i] == b' ' {
            pos = i + 1;
        }
    }
    return &slice[pos..len];
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();
    let mut l = 0;
    let mut r = 0;
    let mut mxlen = 0;
    let mut mxidx = 0;
    while r < bytes.len() {
        if bytes[r] == b' ' {
            l = r + 1;
        } else if r - l + 1 >= mxlen {
            mxlen = r - l + 1;
            mxidx = l;
        }
        r += 1;
    }
    return &sentence[mxidx..mxidx + mxlen];
}
