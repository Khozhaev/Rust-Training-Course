// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    if text.is_empty() {
        return Err("Empty string".to_string());
    }
    Ok(text.chars().next().unwrap())
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();
    for word in line.split_whitespace() {
        let number = word.parse::<i32>();
        if number.is_err() {
            return Err("Invalid number".to_string());
        }
        numbers.push(number.unwrap());
    }
    return Ok(numbers)
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        if self.email.is_none() {
            return None;
        }
        let email = self.email.as_ref().unwrap();
        let domain = email.split('@').nth(1)?;
        Some(domain.to_string())
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::factorial;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(10), 3628800);
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::is_prime;
    #[test]
    fn test_is_prime_0() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(36), false);
        assert_eq!(is_prime(143), false);
        assert_eq!(is_prime(239), true);
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// `TemperatureLog` store city name and list of daily temperature readings.
///
/// It allows to add new temperature and calc avg temperature.
///
/// # Examples
///
/// ```
/// let mut log = TemperatureLog::new("New York");
///
/// log.add_reading(20.5);
/// log.add_reading(25.5);
///
/// assert_eq!(log.average(), Some(23.0));
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// Name of the city
    pub city: String,
    // List collecting all added temperature values
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new `TemperatureLog` for the city with an empty list of values.
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }
    /// Adds new temperature to the list
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }
    /// Calculates and returns average value of temperature
    ///
    /// Returns `None` if list is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("London");
    ///
    /// assert_eq!(log.average(), None);
    ///
    /// log.add_reading(10.0);
    /// log.add_reading(20.0);
    /// log.add_reading(30.0);
    ///
    /// assert_eq!(log.average(), Some(20.0));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}


pub fn tmp() {
    let mut log = TemperatureLog::new("New York");

    log.add_reading(20.5);
    log.add_reading(25.5);

    assert_eq!(log.average(), Some(23.0));
}