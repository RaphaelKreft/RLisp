(println [-->defining prime-sum-pair: try "(prime-sum-pair (list 1 3 5 8) (list 20 35 110))"<--])

(define prime-sum-pair (lambda (list1 list2)
        (let ((a (an-element-of list1))
              (b (an-element-of list2)))
            (cond ((prime? (+ a b)) (list a b))
                  (#t (amb))))))
(define square (lambda (n) (* n n)))

(define prime? (lambda (n)
    (eq? n (smallest-divisor n))))

(define smallest-divisor (lambda (n) (find-divisor n 2)))

(define find-divisor (lambda (n test-divisor)
    (cond ((> (square test-divisor) n) n)
          ((divides? test-divisor n) test-divisor)
          (#t (find-divisor n (+ test-divisor 1))))))

(define divides? (lambda (a b) (eq? (% b a) 0)))

(println [-->defining multiple-dwelling from sicp book: try "(multiple-dwelling)"<--])

(define distinct?(lambda (items)
    (cond ((nil? items) #t)
          ((nil? (cdr items)) #t)
          ((member (car items) (cdr items)) #f)
          (#t (distinct? (cdr items))))))

(define member (lambda (item x)
    (cond ((nil? x) #f)
          ((eq? item (car x)) #t)
          (#t (member item (cdr x))))))

(define multiple-dwelling (lambda ()
    (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
          (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
          (smith (amb 1 2 3 4 5)))
        (do (require (distinct? (list baker cooper fletcher miller smith)))
            (require (not (eq? baker 5)))
            (require (not (eq? cooper 1)))
            (require (not (eq? fletcher 5)))
            (require (not (eq? fletcher 1)))
            (require (> miller cooper))
            (require (not (eq? (abs (- smith fletcher)) 1)))
            (require (not (eq? (abs (- fletcher cooper)) 1)))
            (list (list 'baker baker) (list 'cooper cooper)
            (list 'fletcher fletcher) (list 'miller miller)
            (list 'smith smith))))))

(define append (lambda (x y)
  (cond ((nil? x) y)
        ((list? x) (cons (car x) (append (cdr x) y)))
        (#t (cons x y)))))

(define subsetsum (lambda (target list)
    (subsetsum-proc target list 0 #nil)))


(define subsetsum-proc (lambda (target list sum solution)

    (cond ((nil? list) (subsetsum-check target sum solution))
           (#t (let ((include (amb #t #f)))
                    (cond (include  (subsetsum-proc target (cdr list) (+ sum (car list)) (append solution (car list))))
                          (#t (subsetsum-proc target (cdr list) sum solution))))))))

(define subsetsum-check (lambda (target sum solution)
    (do
       (require (not (nil? solution)))
       (require (eq? target sum))
       (println [Elements in solution:])
       solution)))

(define partition (lambda (list)
    (let ((s (sum list)))
    (cond ((eq? (% s 2) 0) (subsetsum (/ s 2) list))
          (#t (amb))))))


(define sum (lambda (list)
    (cond ((nil? list) 0)
          (#t (+ (car list) (sum (cdr list)))))))