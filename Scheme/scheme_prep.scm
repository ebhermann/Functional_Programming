#lang racket/base

; third, fourth, fifth ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

(define first (lambda (x) (car x)))
(define second (lambda (x) (cadr x)))
(define rest (lambda (x) (cdr x)))
; Takes the rest of the list and grabs the first element of the rest of the list
; So in this example, the list is josh, car, erin, sandy, jon
; The rest of the list is sara, erin, sandy, jon
; The rest of the rest of the list is erin, sandy, jon, and the head of that is erin
(define third (lambda (x) (car (cdr (rest x)))))
; Fourth and fifth follow the same logic: take the rest of the rest of the (you get the idea) of the list and grab the head of that
(define fourth (lambda (x) (car (cdr (cdr (rest x))))))
(define fifth (lambda (x) (car (cdr (cdr (cdr (rest x)))))))
(define family '(josh sara erin sandy jon))
(first family)
(second family)
(third family)
(fourth family)
(fifth family)


; truecount ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

(define truecount
  (lambda (list) ; takes in a list
    (if (null? list) ; if the list is null...
        0 ; return 0
        (if (eq? (car list) '#t) ; otherwise, the list isn't null, so identify if the head of the list is true
        (+ (truecount (cdr list)) 1) ; if it is, increase the count of trues by 1 and recurse with the rest of the list
        (truecount (cdr list)))))) ; otherwise, recurse with the rest of the list

(define boollist (quote(#t #f #t #t #f)))
(truecount boollist)

(define boollist2 (quote(#f #f #f #f #f)))
(truecount boollist2)

(define boollist3 (quote(#t #t #t #t)))
(truecount boollist3)

; squarelist ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
(define squarelist 
    (lambda (list) ; Take in a list as a parameter
    (map (lambda (x) (* x x)) list))) ; Take the list, and square it via a map: each element of the list is multiplied by itself

(squarelist '(1 2 3 4))
(squarelist '(-10 5 8 9))

; hundreds ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
; instructions specified "greater than 100", not "greater than or equal to 100"
(define hundreds?
    (lambda (list) ; take a list as a parameter
    ; filter based on whether each element (num) in the list is greater than 100
        (filter (lambda (num) (> num 100)) list)
    )
)
(hundreds? '(5 500 100 43))
(hundreds? '(450 500 101 321 5 4000 2))


; collatz ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

(define collatz
  (lambda (n) ; take an int parameter
    (if (not (= n 1)) ; if n isn't 1
    (begin
        (display n) (newline) ; print n
        (if (even? n) ; check if n is even
        ; if n is even, recurse with n/2
              (collatz (/ n 2))
              (collatz (+ (* 3 n) 1)))) ; otherwise, n is odd, so recurse with 3n+1
        (display 1)))) ; have reached the end of the sequence, so print 1

(collatz 3) (newline) (newline)
(collatz 7) (newline) (newline)
(collatz 12)