pub fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn main() {
    println!("4 + 6 ={}", add(4,6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1,3), 4);
    }
    #[test]
    fn test_add_bad() {
        assert_eq!(add(1,5), 4);
    }   
}
