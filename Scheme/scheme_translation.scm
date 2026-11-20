#lang racket/base


; example inputs in the project description don't use strings as the input (since there are no quotation marks), so this translation is designed to accept non-strings
(define translate
    (lambda (word) ; take in the word as input
        (cond ; check if it matches any of the words from the translation table, and translate it
            ((or (eq? word 'ling) (eq? word 'zero)) 0)
            ((or (eq? word 'yi) (eq? word 'one)) 1)
            ((or (eq? word 'er) (eq? word 'two)) 2)
            ((or (eq? word 'san) (eq? word 'three)) 3)
            ((or (eq? word 'si) (eq? word 'four)) 4)
            ((or (eq? word 'wu) (eq? word 'five)) 5)
            ((or (eq? word 'liu) (eq? word 'six)) 6)
            ((or (eq? word 'qi) (eq? word 'seven)) 7)
            ((or (eq? word 'ba) (eq? word 'eight)) 8)
            ((or (eq? word 'jiu) (eq? word 'nine)) 9)
            ((or (eq? word 'shi) (eq? word 'ten)) 10)
            (else -1) ; otherwise, it's an unknown word, so translate to -1, which will be discarded later
        )
    )
)

; calculate the sum of a list
(define add
  (lambda (list) ; take in a list as input
    (if (null? list) ; if the list is null
        0 ; return 0
        (+ (add (cdr list)) (car list))))) ; otherwise, recurse with the head of the list + the rest of the list

; calculate the product of a list
(define multiply
  (lambda (list) ; take in a list as input
    (if (null? list) ; if the list is null
        1 ; return 1
        (* (multiply (cdr list)) (car list))))) ; otherwise, recurse with the head of the list * the rest of the list

; print a list, following the correct format depending on if we're printing the translation, addition, or multiplication
(define print
  (lambda (list type) ; take in a list and "type" of printing as input
    (if (not (= (length list) 1)) ; if it isn't the last element of the list
        (begin
          (display (car list)) ; print the current element/head of the list
          (cond ; check which type of printing to perform
            ((eq? type 'regular) (display " ")) ; translation printing, only include spaces between elements
            ((eq? type 'add) (display " + ")) ; addition printing, include + between elements
            (else (display " * ")) ; otherwise, multiplication printing. include * between elements
          )
          (print (cdr list) type)) ; recurse with the rest of the list
        (display (car list))))) ; otherwise just print the last element of the list with no + or * at the end

(define go
    (lambda (list)
        (set! list (map translate list)) ; translate the list
        (set! list (filter (lambda (num) (> num -1)) list)) ; filter the list to only include known words (unknown words are represented as -1)
        (let ((sum (add list))) ; calculate the sum of the elements in the list
        (let ((product (multiply list))) ; calculate the product of the elements in the list
        ; print the translation of the list
        (display "Translation: ")
        (print list 'regular) (newline)
        ; print the addition of the list
        (display "Addition: ")
        (print list 'add)
        (display " = ")
        (display sum) (newline)
        ; print the multiplication of the list
        (display "Multiplication: ")
        (print list 'multiply)
        (display " = ")
        (display product)
    )
)))

(go '(yi bill six ba two batman)) (newline) (newline)
(go '(yi josh three si)) (newline) (newline)
(go '(yi nine six ba))
       