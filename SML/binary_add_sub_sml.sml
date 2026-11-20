(*add the rest of the longer list to the sum. still need to perform addition in case of a carry*)
fun finishBinaryAddition [] false = [] (*empty list with no carry: return an empty list*)
  | finishBinaryAddition [] true = [true] (*empty list with carry: return carry*)
  | finishBinaryAddition (false::rest) false = 
     false :: finishBinaryAddition rest false (*head is false/0 and there is no carry: add false to the sum list and recurse with the rest of the list and no carry*)
  | finishBinaryAddition (true::rest) false =
     true :: finishBinaryAddition rest false (*head is true/1 and there is no carry: add true to the sum list and recurse with the rest of the list and no carry*)
  | finishBinaryAddition (false::rest) true =
    true :: finishBinaryAddition rest false (*head is false/0 and there is carry: add true (1) to the sum list and recurse with the rest of the list and no carry*)
  | finishBinaryAddition (true::rest) true =
    false :: finishBinaryAddition rest true; (*head is true and there is carry: add false to the sum list and recurse with a carry*)

(*accomplishes most of the binary addition*)
(*adds the heads of each list together and then recurses with the rest of both lists*)
fun doBinaryAddition [] [] false = [] (*empty list*)
  | doBinaryAddition [] [] true = [true] (*empty list with carry: return carry/add cary to the sum list*)
  | doBinaryAddition (false::rest1) (false::rest2) false =
        false :: doBinaryAddition rest1 rest2 false (*0+0=0 and there is no carry: add false to the sum list and recurse with no carry*)
  | doBinaryAddition (false::rest1) (false::rest2) true =
        true :: doBinaryAddition rest1 rest2 false (*0+0=0 and there is carry: add true to the sum list and recurse with no carry*)
  | doBinaryAddition (true::rest1) (false::rest2) false =
        true :: doBinaryAddition rest1 rest2 false (*1+0=1 and there is no carry: add true to the sum list and recurse with no carry*)
  | doBinaryAddition (true::rest1) (false::rest2) true =
        false :: doBinaryAddition rest1 rest2 true (*1+0=1 with carry = 0 with a carry of one: add false to the sum list and recurse with carry*)
  | doBinaryAddition (false::rest1) (true::rest2) false =
        true :: doBinaryAddition rest1 rest2 false (*0+1 with no carry = 1: add true to the sum list and recurse with no carry*)
  | doBinaryAddition (false::rest1) (true::rest2) true =
        false :: doBinaryAddition rest1 rest2 true (*0+1=1 with carry = 0 with a carry of one: add false to the sum list and recurse with carry*)
  | doBinaryAddition (true::rest1) (true::rest2) false =
        false :: doBinaryAddition rest1 rest2 true (*1+1=0 with a carry of one: add false to the sum list and recurse with carry*)
  | doBinaryAddition (true::rest1) (true::rest2) true =
        true :: doBinaryAddition rest1 rest2 true (*1+1=0 with carry = 1 with a carry of one: add true to the sum list and recurse with carry*)
  | doBinaryAddition longer_list [] carry =
        finishBinaryAddition longer_list carry (*finish binary addition by adding the longer list and carry to the sum list*)
  | doBinaryAddition [] longer_list carry =
        finishBinaryAddition longer_list carry; (*finish binary addition by adding the longer list and carry to the sum list*)

(*convert binary digits to bools*)
fun toBool [] = [] (*empty list: append nothing*)
  | toBool (1 :: rest) = true :: toBool(rest) (*head is 1: convert to true by adding true to the boolean list. recurse with the rest*)
  | toBool (_ :: rest) = false :: toBool(rest); (*head is 0: convert to false by adding false to the boolean list. recurse with the rest*)

(*convert boolean vals to binary digits*)
fun toBinary [] = [] (*append nothing when there's an empty list*)
  | toBinary (true :: rest) = 1 :: toBinary(rest) (*head is true: convert to 1 by adding 1 to the binary list*)
  | toBinary (_ :: rest) = 0 :: toBinary(rest); (*head is false: convert to 0 by adding 0 to the binary list*)


fun binaryAddition list1 list2 =
    let (*reverse the converted lists*)
        val bool1 = List.rev (toBool list1)
        val bool2 = List.rev (toBool list2)
        val sum = doBinaryAddition bool1 bool2 false (*calculate the sum*)
    in
        toBinary (List.rev sum) (*reverse lists back so they're in the correct order*)
    end;

binaryAddition [1,0,1,1] [1,0];
binaryAddition [1,0,0,1,1,0,1] [0,0,1,0,0,1,0];
binaryAddition [1,0,0,1,0,0,1] [0,0,1,1,0,0,1];
binaryAddition [1,0,0,0,1,1,1] [1,0,1,1,0];

(*lists need to be the same length for two's complement*)
(*assuming the second length is the one being subtracted from the first*)
fun matchLengths size1 size2 list =
    if size2 < size1 then
        matchLengths size1 (size2 + 1) (list @ [false]) (*add false (therefore not changing the number) to the shorter list and recurse with the new length*)
    else
        list; (*just return the list since the length's are equal*)

(*boolean values need to be swapped for two's complement*)
fun swap [] = []
  | swap (true :: rest) = false :: swap(rest) (*true becomes false by adding false to the list and recursing with the rest*)
  | swap (_ :: rest) = true :: swap(rest); (*false becomes true by adding true to the list and recursing with the rest*)

(*binary subtraction operates the same as binary addition once the two's complement is taken*)
fun binarySubtraction list1 list2 =
    let
        (*reverse the converted lists*)
        val bool1 = List.rev (toBool list1)
        val bool2 = List.rev (toBool list2)

        (*ensure the lists are the same length, which is required for two's complement to work correctly*)
        val matched_bool2 = matchLengths (length bool1) (length bool2) bool2

        (*swap the values of the second num (so we're assuming it's bool1-bool2) to perform two's complement*)
        val inverted_bool2 = swap matched_bool2
        (*add one to the inverted list to complete two's complement*)
        val complemented_bool2 = finishBinaryAddition inverted_bool2 true

        (*discard the carry that may have resulted by adding 1 for the most significant bit*)
        val truncated_bool2 = List.take (complemented_bool2, length bool1)

        (*with two's complement now complete, perform addition*)
        val sum = doBinaryAddition bool1 truncated_bool2 false

        (*discard the carry for the most significant bit*)
        val difference = List.take (sum, length bool1)
    in
        toBinary (List.rev difference) (*reverse the list so it displays in the correct order*)
    end;

binarySubtraction [1,0,1,1] [1,0];
binarySubtraction [0,1,0,1,0,0,0,0] [1,1,0,0,0];
binarySubtraction [1,1,0,1,0] [1,0, 1, 1, 1];
binarySubtraction [1,0,0,0,0,1,0,0] [0,1,1,0,1,1,1,1];
        
