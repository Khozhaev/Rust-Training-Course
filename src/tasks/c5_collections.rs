// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }
    let n = vec.len();
    let mut max_idx = n;
    let mut second_max_idx = n;
    for (i, &x) in vec.iter().enumerate() {
        if max_idx == n || x > vec[max_idx] {
            second_max_idx = max_idx;
            max_idx = i;
        } else if (second_max_idx == n || x > vec[second_max_idx]) && (x != vec[max_idx]) {
            second_max_idx = i;
        }
    }
    if second_max_idx == n {
        return None;
    }
    return Some(vec[second_max_idx]);
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    let n = init_sequence.len();
    let mut dp = vec![1; n];
    let mut prev = vec![0; n];
    for i in 0..n {
        prev[i] = n;
        dp[i] = 1;
        for j in 0..i {
            if init_sequence[j] < init_sequence[i] {
                if dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
        }
    }
    let mut result = vec![];
    let mut max_len:usize = 1;
    let mut max_idx:usize = 0;
    for i in 0..n {
        if dp[i] > max_len {
            max_len = dp[i];
            max_idx = i;
        }
    }
    while max_idx != n {
        result.push(init_sequence[max_idx]);
        max_idx = prev[max_idx];
    }
    result.reverse();
    return result;
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let mut words = sentence.split_whitespace().collect::<Vec<&str>>();
    words.reverse();
    return words.join(" ");
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    let mut result_vec= Vec::new();
    for word in words {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            result_vec.push(first_char.to_uppercase().to_string() + &chars.collect::<String>().to_lowercase());
        }
    }
    return result_vec.join(" ");
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut count_map = HashMap::new();
    for char in s.to_lowercase().chars() {
        count_map.entry(char).and_modify(|count| *count += 1).or_insert(1);
    }
    for (_, count) in count_map {
        if count > 1 {
            return false;
        }
    }
    return true;
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut count_map = HashMap::new();
    for num in nums {
        count_map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut count_vec = count_map.into_iter().collect::<Vec<(i32, usize)>>();
    count_vec.sort_by_key(|(_, count)| *count);
    count_vec.reverse();
    return count_vec.into_iter().take(k).map(|(num, _)| num).collect();
}
