; phisor's operation cards: the operators the crate ships, as lambdas over
; phiclm's 39 primitives and the keyboard's 64 (phisor-op-<index>-<name>).
; Read into every machine at its birth; a card punched at run time goes to
; learned.lisp beside this file. One form per card.

; the meet of a list of relations, and their union
(define meet-all (lambda (rs) (if (= (length rs) 1) (car rs) (meet (car rs) (meet-all (cdr rs))))))
(define union-all (lambda (rs) (if (= (length rs) 1) (car rs) (union (car rs) (union-all (cdr rs))))))

; the events the fixed roles name: the domain of the meet of the roles' identities (docs/211)
(define events (lambda (rels) (domain (meet-all rels))))

; a question over what was told: the hole's role(s), read off the events the fixed roles name
(define told-answer (lambda (roles rels) (image (union-all roles) (events rels))))

; a yes/no question over what was told: whether the events the fixed roles name are more than none
; (docs/phisor/runs/mill-usage-2026-11-08: the one operation the count found missing). `no` is
; "not among what was told": the store is all the machine knows.
(define told-whether (lambda (rels) (if (< 0 (count (events rels))) (quote yes) (quote no))))

; a hole inside a clause (told::nested): of the events `evs`, the sub-events their `role-rows` name
; that meet `inner-rels` (the clause's predicate and its own fixed roles); then the hole's roles
; read off the bottom, or whether the bottom holds any event
(define told-under (lambda (evs role-rows inner-rels) (events (cons (identity (range (restrict-domain (union-all role-rows) evs))) inner-rels))))
(define told-answer-of (lambda (roles evs) (image (union-all roles) evs)))
(define told-whether-of (lambda (evs) (if (< 0 (count evs)) (quote yes) (quote no))))
