fun finishBinaryAddition [] false = []
  | finishBinaryAddition [] true = [true]
  | finishBinaryAddition (false::rest) false = 
     false :: finishBinaryAddition rest false
  | finishBinaryAddition (true::rest) false =
     true :: finishBinaryAddition rest false
  | finishBinaryAddition (false::rest) true =
    true :: finishBinaryAddition rest false
  | finishBinaryAddition (true::rest) true =
    false :: finishBinaryAddition rest true;

fun doBinaryAddition [] [] false = []
  | doBinaryAddition [] [] true = [true]
  | doBinaryAddition (false::rest1) (false::rest2) false =
        false :: doBinaryAddition rest1 rest2 false
  | doBinaryAddition (false::rest1) (false::rest2) true =
        true :: doBinaryAddition rest1 rest2 false
  | doBinaryAddition (true::rest1) (false::rest2) false =
        true :: doBinaryAddition rest1 rest2 false
  | doBinaryAddition (true::rest1) (false::rest2) true =
        false :: doBinaryAddition rest1 rest2 true
  | doBinaryAddition (false::rest1) (true::rest2) false =
        true :: doBinaryAddition rest1 rest2 false
  | doBinaryAddition (false::rest1) (true::rest2) true =
        false :: doBinaryAddition rest1 rest2 true
  | doBinaryAddition (true::rest1) (true::rest2) false =
        false :: doBinaryAddition rest1 rest2 true
  | doBinaryAddition (true::rest1) (true::rest2) true =
        true :: doBinaryAddition rest1 rest2 true
  | doBinaryAddition longer_list [] carry =
        finishBinaryAddition longer_list carry
  | doBinaryAddition [] longer_list carry =
        finishBinaryAddition longer_list carry;

fun toBool [] = []
  | toBool (1 :: rest) = true :: toBool(rest)
  | toBool (_ :: rest) = false :: toBool(rest);

fun toBinary [] = []
  | toBinary (true :: rest) = 1 :: toBinary(rest)
  | toBinary (_ :: rest) = 0 :: toBinary(rest);

fun binaryAddition list1 list2 =
    let
        val bool1 = List.rev (toBool list1)
        val bool2 = List.rev (toBool list2)
        val sum = doBinaryAddition bool1 bool2 false
    in
        toBinary (List.rev sum)
    end;

binaryAddition [1,0,1,1] [1,0];
binaryAddition [1,0,0,1,1,0,1] [0,0,1,0,0,1,0];
binaryAddition [1,0,0,1,0,0,1] [0,0,1,1,0,0,1];
binaryAddition [1,0,0,0,1,1,1] [1,0,1,1,0];

fun matchLengths size1 size2 list =
    if size2 < size1 then
        matchLengths size1 (size2 + 1) (list @ [false])
    else
        list;

fun swap [] = []
  | swap (true :: rest) = false :: swap(rest)
  | swap (_ :: rest) = true :: swap(rest);

fun binarySubtraction list1 list2 =
    let
        
        val bool1 = List.rev (toBool list1)
        val bool2 = List.rev (toBool list2)

        
        val matched_bool2 = matchLengths (length bool1) (length bool2) bool2

      
        val inverted_bool2 = swap matched_bool2
        val complemented_bool2 = finishBinaryAddition inverted_bool2 true

  
        val truncated_bool2 = List.take (complemented_bool2, length bool1)

    
        val sum = doBinaryAddition bool1 truncated_bool2 false

        val difference = List.take (sum, length bool1)
    in
        toBinary (List.rev difference)
    end;

binarySubtraction [1,0,1,1] [1,0];
binarySubtraction [0,1,0,1,0,0,0,0] [1,1,0,0,0];
binarySubtraction [1,1,0,1,0] [1,0, 1, 1, 1];
binarySubtraction [1,0,0,0,0,1,0,0] [0,1,1,0,1,1,1,1];
        
