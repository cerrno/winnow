;;; *Oh My Gawd*: It's Full of Stars

(load "../prereqs.scm")

(define removemember*
  (lambda (a lat)
    (cond
     ((null? lat) '())
     ((atom? (car lat))
      (cond
       ((eq? (car lat) a) (removemember* a (cdr lat)))
       (else (cons (car lat) (removemember* a (cdr lat))))))
     (else (cons (removemember* a (car lat)) (removemember* a (cdr lat)))))))

(define l '((coffee) cup ((tea) cup) (and (hick)) cup))
(removemember* 'cup l)

(define occur*
  (lambda (a l)
    (cond 
      ((null? l) 0)
      ((atom? (car l))
       (cond 
         ((eq? (car l) a) (+ 1 (occur* a (cdr l))))
         (else (occur* a (cdr l)))))
      (else (+ (occur* a (car l)) (occur* a (cdr l)))))))

(define ll '((banana) (split ((((banana ice))) (cream (banana)) sherbert)) (banana) (bread) (banana brandy)))
(occur* 'banana ll)

(define subst*
  (lambda (new old l)
    (cond
     ((null? l) '())
     ((atom? (car l))
      (cond
       ((eq? old (car l)) (cons new (subst* new old (cdr l))))
       (else (cons (car l) (subst* new old (cdr l))))))
     (else (cons (subst* new old (car l)) (subst* new old (cdr l)))))))

(subst* 'strawberry 'banana ll)

(define insertL*
  (lambda (new old l)
    (cond
     ((null? l) '())
     ((atom? (car l))
      (if (eq? (car l) old)
          (cons new (cons old (insertL* new old (cdr l))))
          (cons (car l) (insertL* new old (cdr l)))))
     (else (cons (insertL* new old (car l)) (insertL* new old (cdr l)))))))

(insertL* 'roast 'chuck li)

(define member*
  (lambda (a l)
    (cond
     ((null? l) #f)
     ((atom? (car l))
      (cond
       ((eq? (car l) a) #t)
       (else (member* a (cdr l)))))
     (else (or (member* a (car l)) (member* a (cdr l)))))))

(member* 'cream ll)
(insertL* 'roast 'chuck li)
