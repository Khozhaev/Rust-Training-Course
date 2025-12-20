//#[cfg(test)]
mod tasks;

#[cfg(test)]
mod tests;

use tasks::c3_ownership_and_memory;

fn main() {
    c3_ownership_and_memory::string_ownership();
    c3_ownership_and_memory::simple_borrowing();
    c3_ownership_and_memory::hard_borrowing();
}
