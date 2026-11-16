fn main() {
    // Warm up questions
    // Meaning of Life
    println!("{:?}", hitchhiker(&[1, 2, 3, 4, 42, 5, 6, 3, 4]));
    println!("{:?}", hitchhiker(&[2, 4, 6]));
    println!("{:?}", hitchhiker(&[2, 4, 6, 42, 3, 24, 42]));
    // Every other element


    // Goldbach's Conjecture
}

// Meaning of life
// return true if the number 42 is in the list
pub fn hitchhiker(list: &[i32]) -> bool {
    let bool_list: Vec<i32> = list.iter()
        .copied()
        .filter(|n| *n == 42)
        .collect();

    return !bool_list.is_empty()
}
