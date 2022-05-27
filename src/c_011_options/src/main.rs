use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let value = map.get("four");

    // Method 1
    let incr_value = match value{
        Some(val) => val +1,
        None => 0,
    };
    println!("Method 1: {:?}", incr_value);

    // Method 2
    let incr_value2 = if let Some(val) = value{
        val+1
    } else{
        0
    };
    println!("Method 2: {:?}", incr_value2);

    // Method 3
    let incr_value3 = value.unwrap_or_else(|| &0);
    println!("Method 2: {:?}", incr_value3);

}
