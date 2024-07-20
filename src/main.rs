mod control_algorithm;

use std::collections::HashMap;
use std::ffi::c_char;
use std::hash::Hash;

fn main(){

}

fn foo(left: &str, right: &str) -> bool{

    false
}

fn count(s: &str) -> HashMap<char, u8> {
    let mut counter: HashMap<char, u8> = HashMap::new();

    for c in s.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    counter
}

#[cfg(test)]
mod tests{
    use crate::foo;

    #[test]
    fn foo_ear_hear(){
        assert!(!foo("ear", "hear"))
    }
}