use std::arch::x86_64::__cpuid;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
//this check u8 for being over u8::MAX becasue --release flag  has no runtime checks


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
