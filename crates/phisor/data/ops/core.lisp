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
