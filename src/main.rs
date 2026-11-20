fn main() {
    // Warm up questions
    // Meaning of Life
    println!("~~~~~ Meaning of Life ~~~~~");
    println!("{:?}", hitchhiker(&[1, 2, 3, 4, 42, 5, 6, 3, 4]));
    println!("{:?}", hitchhiker(&[2, 4, 6]));
    println!("{:?}", hitchhiker(&[2, 4, 6, 42, 3, 24, 42]));

    // Every other element
    println!("~~~~~ Every Other ~~~~~");
    println!("{:?}", everyother(&[2, 4, 6, 42, 3, 24, 42]));
    println!("{:?}", everyother(&[1, 2]));
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));
    println!("{:?}", everyother(&[1, 2, 3, 4, 5, 6]));

    // Goldbach's Conjecture
    println!("~~~~~ Goldbach ~~~~~");
    goldbach(28);
    goldbach(14);
    goldbach(108);
    goldbach(420);

    // Binary Addition
    println!("~~~~~ Binary Addition ~~~~~");
    println!("{:?}", binaryAddition(&mut[1,0,1,1], &mut[1,0]));
    println!("{:?}", binaryAddition(&mut[1,0,0,1,1,0,1], &mut[0,0,1,0,0,1,0]));
    println!("{:?}", binaryAddition(&mut[1,0,0,1,0,0,1], &mut[0,0,1,1,0,0,1]));
    println!("{:?}", binaryAddition(&mut[1,0,0,0,1,1,1], &mut[1,0,1,1,0]));

    // Binary Subtraction
    // Assuming it's list1 - list2
    println!("~~~~~ Binary Subtraction ~~~~~");
    println!("{:?}", binarySubtraction(&mut[1,0,1,1], &mut[1,0]));
    println!("{:?}", binarySubtraction(&mut[0,1,0,1,0,0,0,0], &mut[1,1,0,0,0]));
    println!("{:?}", binarySubtraction(&mut[1,1,0,1,0], &mut[1,0, 1, 1, 1]));
    println!("{:?}", binarySubtraction(&mut[1,0,0,0,0,1,0,0], &mut[0,1,1,0,1,1,1,1]));

    // Translation
    let chinese: Vec<&str> = vec!["ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi",];
    go(&chinese);

    let input = vec!["yi", "nine", "six", "ba"];
    go(&input);

    let input2 = vec!["yi", "josh", "three", "si"];
    go(&input2);

    let input3 = vec!["er", "cat", "six", "cocoa", "si", "qi", "one"];
    go(&input3);
}

// Translation Functions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~``
pub fn go(list: &[&str]) {
    let mut translated_list: Vec<i32> = list.iter()
        .copied()
        .map(to_num)
        .collect();

    // Filter list to remove unknown words
    translated_list = translated_list.iter()
        .cloned()
        .filter(|x| *x >= 0)
        .collect();

    // Print out list
    // could i reuse the printing? because i need to do addition and multiplication and print them
    // can probably recurse
    println!("Translation: {}", print(&translated_list, "regular"));

    // Addition
    let sum = add(&translated_list);
    println!("Addition: {} = {}", print(&translated_list, "add"), sum);

    // Multiplication
    let product = multiply(&translated_list);
    println!("Multiplication: {} = {}", print(&translated_list, "multiply"), product);

    println!();
}

fn to_num(word: &str) -> i32 {
    match word {
        "ling" | "zero" => 0,
        "yi" | "one" => 1,
        "er" | "two" => 2,
        "san" | "three" => 3,
        "si" | "four" => 4,
        "wu" | "five" => 5,
        "liu" | "six" => 6,
        "qi" | "seven" => 7,
        "ba" | "eight" => 8,
        "jiu" | "nine" => 9,
        "shi" | "ten" => 10,
        _ => -1 // indicates unknown word, will be ignored later
    }
}

fn add(list: &[i32]) -> i32 {
    match list {
        [] => 0,
        [head, rest @ ..] => head + add(rest)
    }
}

fn multiply(list: &[i32]) -> i32 {
    match list {
        [] => 1,
        [head, rest @ ..] => head * multiply(rest)
    }
}

fn print(list: &[i32], print_type: &str) -> String {
    match (list, print_type) {
        ([], _any) => String::new(),
        ([single], _any) => single.to_string(),
        ([head, rest @ ..], "regular") => head.to_string() + " " + &print(rest, print_type),
        ([head, rest @ ..], "add") => head.to_string() + " + " + &print(rest, print_type),
        ([head, rest @ ..], "multiply") => head.to_string() + " * " + &print(rest, print_type),
        ([head, rest @ ..], _) => head.to_string() + " " + &print(rest, print_type),
    }
}

// Warm up questions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Meaning of life
// return true if the number 42 is in the list
pub fn hitchhiker(list: &[i32]) -> bool {
    let bool_list: Vec<i32> = list.iter() // iterate over the list
        .copied()
        .filter(|n| *n == 42) // filters the list to only include the number 42
        .collect();

    // if the number 42 was in the list, it won't be empty, so true will be returned
    return !bool_list.is_empty()
}

// Every other element
pub fn everyother(list: &[i32]) -> Vec<i32> {
    match list {
        [] => Vec::new(), // if the list is empty, return an empty vector
        [_x] => Vec::new(), // if there's only one element, also return an empty vector since every other includes every other element, starting with the second element in the list
        [_x, _y, rest @ ..] => { // x is the first element, which will always be skipped. Y is the second element, which we want to keep. The rest is the list we should extend with
            let mut next_elem = vec![list[1]]; // keep the second element in the list, following the every other behavior described
            next_elem.extend(everyother(rest)); // add the second element to the every other list that's under construction and recurse with the rest of the list
            next_elem
        }
    }
}

// Goldbach's Conjecture ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn goldbach(n: i32) {
    match n > 2 && n % 2 == 0 { // error checking
        true => {  
            let primes = goldbach_recurse(n, 2, n); // separate recursion function since error checking is done here, store the result in a list
            println!("{} + {} = {}", primes.0, primes.1, n); // print the primes that add up to the provided even number
        } 
        false => {
            println!("Error. The number is not even and greater than 2");
        }

}
}

fn is_prime(n: i32, divisor: i32) -> bool {
    match divisor * divisor <= n { // only continue until the square root of the number because once the square root has been reached, all possible divisors have been tested
        true => {
        match n % divisor { // is the number divisible by the current divisor?
            0 => false, // if it is, the number isn't prime
            _ => is_prime(n, divisor+1) // otherwise, recurse with a larger divisor
        }
    }
        false => true // if we've reached this point, the n didn't have any divisors and is therefore prime
    } 
}


fn goldbach_recurse(mut num1: i32, num2: i32, original: i32) -> (i32, i32) {
    match num2 < original / 2 { // only recurse until halfway to the number because once the half has been reached, all possible addends have been tested
        true => {
            num1 = original - num2; // subtract first addend from original num
            match (is_prime(num1, 2), is_prime(num2, 2)) { // check if the original number - the addend and the addend are both prime
                (true, true) => {
                    (num1, num2) // if they are, return both and exit as the nums satisfying the conjecture have been found
                }
                _ => { // otherwise, recurse, incrementing the second addend (which is what's subtracted from the original num) by 1
                    goldbach_recurse(num1, num2+1, original)
                }
            } 
        }
        false => { // this branch won't be reached since goldbach's theorem will find 2 primes, so return a placeholder. included so the compiler is satisfied
            (0, 0)
        }
    }
}


// Binary Addition and Subtraction ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn binaryAddition(list1: &mut[i32], list2: &mut[i32]) -> Vec<i32> {
    // reverse lists
    list1.reverse();
    list2.reverse();

    // convert to booleans
    let bool1 = to_bool(list1);

    let bool2 = to_bool(list2);

    // Call doBinaryAddition
    let mut sum = doBinaryAddition(&bool1, &bool2, false);

    // Reverse sum (so that it's in the correct order)
    sum.reverse();

    // Convert back to 0s and 1s
    to_binary(&sum)
}

// performs most of the work for binary addition
pub fn doBinaryAddition(list1: &[bool], list2: &[bool], carry: bool) -> Vec<bool> {
    // addition values are hard-coded
    // this match statement adds the heads of each list
    match (list1, list2, carry) {
        ([], [], false) => Vec::<bool>::new(), // if the list is empty and there's no carryover, return an empty vector

        ([], [], true) => vec![true], // if the list is empty and there is a carryover, return the carryover

        ([false, rest1 @ ..], [false, rest2 @ ..], carry) => { // 0 + 0
            let mut next_elem = vec![false ^ carry]; // sum bit is 0 + carry. ^ (XOR) accomplishes the addition for that (i.e. if carry is 0, sum bit remains 0, otherwise sum bit is 1)
            next_elem.extend(doBinaryAddition(rest1, rest2, false)); // no additional carry, recurse with the rest of both lists
            next_elem // return the sum bit
        },

        ([true, rest1 @ ..], [false, rest2 @ ..], carry) => { // 1 + 0
            let mut next_elem = vec![true ^ carry]; // sum bit is 1 + carry (XOR means 1+1 has a sum bit of 0 (with a carry of 1, as determined in the next line), while 1+0 has a sum bit of 1)
            let next_carry = true && carry; // does this generate a carry? sum bit is true, so if carry is true, then there is a carry bit brought to the next call
            next_elem.extend(doBinaryAddition(rest1, rest2, next_carry)); // recurse with the rest of both lists
            next_elem // return the sum bit
        },

        ([false, rest1 @ ..], [true, rest2 @ ..], carry) => { // 0 + 1 = 1
            let mut next_elem = vec![true ^ carry]; // sum bit is 1 with no carry, 0 with carry
            let next_carry = true && carry; // this addition generates a carry for the next call if this addition had a carry
            next_elem.extend(doBinaryAddition(rest1, rest2, next_carry)); // recurse with the rest of both lists
            next_elem // return the sum bit
        },

        ([true, rest1 @ ..], [true, rest2 @ ..], carry) => { // 1 + 1 = 0
            let mut next_elem = vec![false ^ carry]; // sum bit is 0 with no carry, 1 with carry
            next_elem.extend(doBinaryAddition(rest1, rest2, true)); // 1+1 always has a carry. recurse with the rest of both lists
            next_elem // return the sum bit
        },

        (x, [], carry) => finishBinaryAddition(x, carry), // second list has ended, carry the rest of the first list into another function

        ([], x, carry) => finishBinaryAddition(x, carry) // first list has ended, carry the rest of the second list into another function
    }

}

// Appending the rest of the longer list to the sum
// However, if there are carries, addition still needs to be performed, so this function accomplishes that
pub fn finishBinaryAddition(longer_list: &[bool], carry: bool) -> Vec<bool> {

    match (longer_list, carry) {
        ([], false) => Vec::<bool>::new(), // empty list, no carry. Return empty vector

        ([], true) => vec![true], // Empty list, carry, return bit of 1 (true)

        ([false, rest @ ..], false) => { // 0 + carry of 0 = 0. There are still more elements in the list
            let mut next_elem = vec![false]; // sum bit is 0
            next_elem.extend(finishBinaryAddition(rest, false)); // recurse with the rest of the list, and there's no carry
            next_elem // return the sum bit
        },

        ([true, rest @ ..], false) => { // 1 + carry of 0 = 1
            let mut next_elem = vec![true]; // sum bit is 1
            next_elem.extend(finishBinaryAddition(rest, false)); // recurse with the rest of the list, and there is no carry
            next_elem // return the sum bit
        },

        ([false, rest @ ..], true) => { // 0 + carry = 1
            let mut next_elem = vec![true]; // sum bit is 1
            next_elem.extend(finishBinaryAddition(rest, false)); // recurse with the rest of the list, and there is no carry
            next_elem // return the sum bit
        },

        ([true, rest @ ..], true) => { // 1 + carry = 0 with a carry of 1
            let mut next_elem = vec![false]; // sum bit is 0 (1 is carried)
            next_elem.extend(finishBinaryAddition(rest, true)); // recurse with the rest of the list and a carry
            next_elem // return the sum bit
        }
    }
}

// Binary subtraction, which operates the same as addition after taking the two's complement
pub fn binarySubtraction(list1: &mut[i32], list2: &mut[i32]) -> Vec<i32> {
    // reverse lists
    list1.reverse();
    list2.reverse();

    // convert to booleans
    let bool1 = to_bool(list1);

    let mut bool2 = to_bool(list2);

    // Lists have to be the same length for two's complement
    // Base case: lengths are equal
    match_lengths(bool1.len(), bool2.len(), &mut bool2);

    // Two's complement
    // Swap values (true to false/1 to 0, false to true/0 to 1)
    bool2 = swap(&bool2);

    // add 1, treating it as a carry bit (true)
    bool2 = finishBinaryAddition(&bool2, true);
    bool2.truncate(bool1.len()); // carry for the most significant bit is dropped

    // Subtraction can now be handled as addition
    // Call doBinaryAddition
    let mut difference = doBinaryAddition(&bool1, &bool2, false);

    // drop carry for most significant bit
    difference.truncate(bool1.len());

    // Reverse sum (so that it's in the correct order)
    difference.reverse();

    // Convert back to 0s and 1s
    to_binary(&difference)
}

// Convert binary bits to booleans
fn to_bool(list: &[i32]) -> Vec<bool> {
    list.iter() // iterate over the list
        .map(|&int| match int {
            1 => true, // bit of 1 = true
            _ => false, // since we are assuming that all inputs are valid, bit must be 0, which is false
        })
        .collect()
}

// Convert booleans to binary bits
fn to_binary(list: &[bool]) -> Vec<i32> {
        list.iter() // iterate over the list
        .map(|&int| match int {
            true => 1, // true = bit of 1
            _ => 0, // since we are assuming that all inputs are valid, boolean must be false, which is 0
        })
        .collect()
}

// swap the boolean values of a list, which is used for taking the two's complement
fn swap(list: &[bool]) -> Vec<bool> {
        list.iter() // iterate over the list
        .map(|&int| match int {
            true => false, // true becomes false
            _ => true, // since we are assuming that all inputs are valid, boolean must be false, which becomes true
        })
        .collect()
}

// lists/binary numbers need to be the same length for two's complement
// since i'm assuming the first list is the one being subtracted from, it will be longer than the second, so recurse until the second list is the same length as the first
fn match_lengths(size1: usize, size2: usize, list: &mut Vec<bool>) {
    if size2 < size1 {
        list.push(false); // just add 0 (false) to the list/number so the value isn't changed but the lengths match
        match_lengths(size1, size2+1, list);
    }
}