fn main() {
    // Warm up questions
    // Meaning of Life
    println!("{:?}", hitchhiker(&[1, 2, 3, 4, 42, 5, 6, 3, 4]));
    println!("{:?}", hitchhiker(&[2, 4, 6]));
    println!("{:?}", hitchhiker(&[2, 4, 6, 42, 3, 24, 42]));

    // Every other element
    println!("{:?}", everyother(&[2, 4, 6, 42, 3, 24, 42]));
    println!("{:?}", everyother(&[1, 2]));
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));
    println!("{:?}", everyother(&[1, 2, 3, 4, 5, 6]));

    // Goldbach's Conjecture


    // Binary Addition
    println!("{:?}", binaryAddition(&mut[1,0,1,1], &mut[1,0]));
}

// Meaning of life
// return true if the number 42 is in the list
pub fn hitchhiker(list: &[i32]) -> bool {
    let bool_list: Vec<i32> = list.iter()
        .copied()
        .filter(|n| *n == 42) // filters the list to only include the number 42
        .collect();

    // if the number 42 was in the list, it won't be empty, so true will be returned
    return !bool_list.is_empty()
}

// Every other element
pub fn everyother(list: &[i32]) -> Vec<i32> {
    match list {
        [] => Vec::new(),
        [_x] => Vec::new(),
        [_x, _y, rest @ ..] => { // x is the first element, which will always be skipped. Y is the second element, which we want to keep. The rest is the list we should extend with
            let mut next_elem = vec![list[1]];
            next_elem.extend(everyother(rest));
            next_elem
        }
    }
}

// Goldbach's Conjecture
pub fn goldbach(n: i32) {
    // error checking could be done with pattern matching...check that it's greater than 2 and even?
    // base case might be 2? I'm not sure....need to see some examples
    // every even number greater than 2 
    // 28 = 5 + 23
    /*
        30 = 13 + 17
        46 = 23 + 23
     */
}

// Binary Addition and Subtraction
pub fn binaryAddition(list1: &mut[i32], list2: &mut[i32]) -> Vec<i32> {
    // reverse lists
    list1.reverse();
    list2.reverse();

    // convert to booleans
    let bool1 = toBool(list1);

    let bool2 = toBool(list2);

    // Call doBinaryAddition
    let mut sum = doBinaryAddition(&bool1, &bool2, false);

    // Reverse sum (so that it's in the correct order)
    sum.reverse();

    // Convert back to 0s and 1s
    toBinary(&sum)
}


pub fn doBinaryAddition(list1: &[bool], list2: &[bool], carry: bool) -> Vec<bool> {
    match (list1, list2, carry) {
        ([], [], false) => Vec::<bool>::new(),
        
        ([], [], true) => vec![true],

        ([false, rest1 @ ..], [false, rest2 @ ..], carry) => {
            let mut next_elem = vec![false ^ carry];
            next_elem.extend(doBinaryAddition(rest1, rest2, false)); // no additional carry
            next_elem
        },

        ([true, rest1 @ ..], [false, rest2 @ ..], carry) => {
            let mut next_elem = vec![true ^ carry];
            let next_carry = true && carry;
            next_elem.extend(doBinaryAddition(rest1, rest2, next_carry));
            next_elem
        },

        ([false, rest1 @ ..], [true, rest2 @ ..], carry) => {
            let mut next_elem = vec![true ^ carry];
            let next_carry = true && carry;
            next_elem.extend(doBinaryAddition(rest1, rest2, next_carry));
            next_elem
        },

        ([true, rest1 @ ..], [true, rest2 @ ..], carry) => {
            let mut next_elem = vec![false ^ carry];
            next_elem.extend(doBinaryAddition(rest1, rest2, true));
            next_elem
        },

        (x, [], carry) => finishBinaryAddition(x, carry),

        ([], x, carry) => finishBinaryAddition(x, carry)
    }

}


pub fn finishBinaryAddition(longer_list: &[bool], carry: bool) -> Vec<bool> {

    match (longer_list, carry) {
        ([], false) => Vec::<bool>::new(),

        ([], true) => vec![true],

        ([false, rest @ ..], false) => {
            let mut next_elem = vec![false];
            next_elem.extend(finishBinaryAddition(rest, false));
            next_elem
        },

        ([true, rest @ ..], false) => {
            let mut next_elem = vec![true];
            next_elem.extend(finishBinaryAddition(rest, false));
            next_elem
        },

        ([false, rest @ ..], true) => {
            let mut next_elem = vec![true];
            next_elem.extend(finishBinaryAddition(rest, false));
            next_elem
        },

        ([true, rest @ ..], true) => {
            let mut next_elem = vec![false];
            next_elem.extend(finishBinaryAddition(rest, true));
            next_elem
        }
    }
}

fn toBool(list: &[i32]) -> Vec<bool> {
    list.iter()
        .map(|&int| match int {
            1 => true,
            _ => false, // since we are assuming that all inputs are valid
        })
        .collect()
}

fn toBinary(list: &[bool]) -> Vec<i32> {
        list.iter()
        .map(|&int| match int {
            true => 1,
            _ => 0, // since we are assuming that all inputs are valid
        })
        .collect()
}
