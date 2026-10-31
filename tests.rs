# phical keyboard — the frozen index table of the 64 Operators (R6.2, R6.3).
#
# Generated from `phical::keyboard::index::render()`. Do not edit by hand.
#
# Re-record only through `phical::keyboard::index::record`: the previous
# table, the new table and the stated reason, all three. Any one of the three
# absent is recorded `unjudged` and fails (R11.10).
#
# index	name	family	arity	operands	result	idempotent	inverse	tolerance
0	identity	circle	1	word	word	yes	exact	-
1	reduce	circle	1	phasor	phasor	yes	none	-
2	centred	circle	2	phasor,phasor	phasor	no	none	-
3	double_centred	circle	4	phasor,phasor,phasor,phasor	phasor	no	none	-
4	delta	circle	2	phasor,phasor	phasor	no	none	-
5	distance	circle	2	phasor,phasor	phasor	no	none	-
6	delta_euclid	circle	2	phasor,phasor	phasor	no	none	-
7	step_toward	circle	3	phasor,phasor,phasor	phasor	no	none	-
8	analogy	circle	3	phasor,phasor,phasor	phasor	no	none	-
9	agreement	circle	2	phasor,phasor	phasor	no	none	-
10	nearest	circle	2	phasor,phasor	phasor	no	none	-
11	centroid	circle	2	phasor,phasor	phasor	no	none	-
12	shape_similarity	circle	2	phasor,phasor	phasor	no	none	-
13	twins	circle	2	phasor,phasor	phasor	no	none	-
14	quantise	q64	1	phasor	class	no	approximate	0.001204543795
15	lift	q64	1	class	phasor	no	approximate	0.001204543795
16	quantise_word	q64	3	phasor,phasor,phasor	variation	no	none	-
17	lift_word	q64	1	variation	phasor	no	none	-
18	hamming	q64	2	variation,variation	phasor	no	none	-
19	sector_distance	q64	2	class,class	phasor	no	none	-
20	sector_rotate	q64	2	class,class	class	no	none	-
21	sector_nearest	q64	3	class,class,class	class	no	none	-
22	codon_of	helix	1	class	word	no	none	-
23	codon	helix	1	word	word	no	none	-
24	hexagram	helix	1	word	word	no	none	-
25	degree	helix	1	word	phasor	no	none	-
26	order	helix	1	word	phasor	no	none	-
27	position	helix	1	word	word	no	none	-
28	likelihood	classes	2	phasor,phasor	phasor	no	none	-
29	move_gain	classes	3	phasor,phasor,class	phasor	no	none	-
30	apply_move	classes	3	phasor,phasor,class	phasor	no	none	-
31	split_cost	classes	2	class,phasor	phasor	no	none	-
32	halve	classes	2	class,level	class	no	none	-
33	cut	classes	3	class,level,level	class	no	none	-
34	mode	classes	2	class,level	class	no	none	-
35	inside	classes	3	class,class,level	phasor	no	none	-
36	sibling	classes	1	class	class	no	exact	-
37	parent	classes	1	class	class	no	none	-
38	pay	classes	3	phasor,phasor,class	phasor	no	none	-
39	size	classes	1	level	class	no	none	-
40	better	liskov	3	phasor,phasor,phasor	phasor	no	none	-
41	worse	liskov	3	phasor,phasor,phasor	phasor	no	none	-
42	loss	liskov	3	phasor,phasor,phasor	phasor	no	none	-
43	forcing_choice	liskov	3	phasor,phasor,phasor	phasor	no	none	-
44	staged_better	liskov	2	phasor,phasor	phasor	no	none	-
45	staged_worse	liskov	2	phasor,phasor	phasor	no	none	-
46	stage	liskov	1	phasor	class	no	none	-
47	measure	liskov	1	phasor	phasor	no	none	-
48	find_laws	relate	3	row,kind,row	phasor	no	none	-
49	find_inverses	relate	3	row,kind,row	phasor	no	none	-
50	implied	relate	3	row,kind,row	phasor	no	none	-
51	score	relate	3	row,kind,row	phasor	no	none	-
52	prior	relate	3	row,kind,row	phasor	no	none	-
53	two_hop	relate	3	row,kind,row	phasor	no	none	-
54	chance	relate	1	class	phasor	no	none	-
55	score_laws	relate	3	row,kind,row	phasor	no	none	-
56	laws_epoch	relate	3	row,kind,row	phasor	no	none	-
57	inherit_epoch	relate	3	row,kind,row	phasor	no	none	-
58	converse	relate	2	kind,kind	phasor	no	none	-
59	inherit	relate	3	row,kind,row	phasor	no	none	-
60	law_holds	relate	2	kind,kind	phasor	no	none	-
61	symmetric	relate	1	kind	phasor	no	none	-
62	confident	relate	3	row,kind,row	phasor	no	none	-
63	bar	relate	1	class	phasor	no	none	-
