(*Hitchhiker function*)
fun hitchhiker [] = false (*Base case: list is empty, so 42 wasn't found*)
  | hitchhiker (head::rest) = (*Checks to see if the head element is 42*)
      if head = 42 then true (*42 found, return true*)
      else hitchhiker rest; (*If not, recurse with the rest of the list*)
hitchhiker [1, 2, 3, 4, 42, 5, 6, 3, 4];
hitchhiker [2, 4, 6];
hitchhiker [2, 4, 6, 42, 3, 24, 42];

(*Every Other Function*)
fun everyother [] = [] (*Base case*)
      | everyother [x] = [] (*If the list is only one element, return an empty list since every other takes every other element, starting at the second element of the list*)
  | everyother(x::y::rest) = y::everyother rest; (*Otherwise, the list has 3+ elements. Discard the head, take the second element (which creates the every other behavior) and recurse with the rest of the list*)
everyother [2, 4, 6, 42, 3, 24, 42];
everyother [1, 2];
everyother [3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];
everyother [1, 2, 3, 4, 5, 6];


(*Goldbach Functions*)
fun is_prime (n, divisor) =
    if divisor * divisor <= n then (*Only go up to the square root of the number because once the square root has been reached, all possible divisors have been covered already*)
        case n mod divisor of (*Is the number divisible by the divisor?*)
            0 => false (*No remainder, so it's divisible and therefore not prime*)
           | _ => is_prime(n, divisor+1) (*Remainder, recurse, incrementing divisor by 1*)
    else 
      true; (*A divisor wasn't found, so return true*)


fun goldbach_recurse (num1, num2, original) =
    if num2 < original div 2 then (*Only continue up to half the num because after half has been reached, all possible addends (in 25+3=28, 25 and 3 are the addends) have been evaluated*)
        let
            val num1 = original - num2 (*Subtract first addend from original num*)
        in
            case (is_prime(num1, 2), is_prime(num2, 2)) of (*Are both numbers prime?*)
                (true, true) => (num1, num2)    (*Both are prime, so return them*)
                | _ => goldbach_recurse(num1, num2+1, original) (*At least one isn't prime so recurse, incrementing smaller addend*)
        end
    else
       (0, 0); (*Only here for syntax: this else will never be reached according to Goldbach's conjecture*)

fun goldbach n =
    if n > 2 andalso n mod 2 = 0 then (*Error checking: confirms the number is even and greater than 2*)
        let
            val primes = goldbach_recurse (n, 2, n) (*Call recursive function*)
        in (*Print result*)
            print (Int.toString (#1 primes) ^ " + " ^
                   Int.toString (#2 primes) ^ " = " ^
                   Int.toString n ^ "\n")
        end
    else
        print "Error. The number is not even and greater than 2\n";

goldbach(28);
goldbach(14);
goldbach(108);
goldbach(420);
