; Note G: the Bernoulli numbers, as Lovelace's cards (1843). Her table computes
; B(2n-1), in her numbering B1 = 1/6, B3 = -1/30, B5 = 1/42, B7 = -1/30, from
;   0 = A0 + A1·B1 + A3·B3 + … + B(2n-1)
; where A0 = -1/2 · (2n-1)/(2n+1), A1 = 2n/2, and each next coefficient is the
; last times (2n-2k+3)(2n-2k+2) / ((2k-1)(2k)). The cards below are that
; recurrence: a cycle (her backed cards) over k, and the numbers already found
; read back - the first published program, run on the mill.

; A0, for the n of B(2n-1)
(define note-g-a0 (lambda (n) (* -0.5 (/ (- (* 2 n) 1) (+ (* 2 n) 1)))))

; the k-th coefficient A(2k-1): A1 = n, then the cycle
(define note-g-coef (lambda (n k) (if (= k 1) n (* (note-g-coef n (- k 1)) (/ (* (- (* 2 n) (- (* 2 k) 3)) (- (* 2 n) (- (* 2 k) 2))) (* (- (* 2 k) 1) (* 2 k)))))))

; the sum A1·B1 + A3·B3 + … up to k = n-1
(define note-g-sum (lambda (n k) (if (= k n) 0 (+ (* (note-g-coef n k) (bernoulli (- (* 2 k) 1))) (note-g-sum n (+ k 1))))))

; B(m) for odd m, her numbering: B(2n-1) = -(A0 + the sum)
(define bernoulli (lambda (m) (let ((n (/ (+ m 1) 2))) (- 0 (+ (note-g-a0 n) (note-g-sum n 1))))))
