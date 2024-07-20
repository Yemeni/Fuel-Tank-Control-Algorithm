use std::collections::HashMap;
use std::ffi::c_char;
use std::hash::Hash;

fn main(){

}

fn foo(left: &str, right: &str) -> bool{

    false
}

fn count(s: &str) -> HashMap<char, u8>{
    let mut counter: () = HashMap::new();

}

#[cfg(test)]
mod tests{
    use crate::foo;

    #[test]
    fn foo_ear_hear(){
        assert!(!foo("ear", "hear"))
    }
}