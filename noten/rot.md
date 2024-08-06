I would like a tool called `rot` which just does the rot cipher.

Usage should be something like:
```sh
rot 13 "string"
fgevat

rot 4 "string"
wxvmrk

rot 6 "string"
yzxotm
```

0  a b c d e f g h i j k l m n o p q r s t u v w x y z
1  b c d e f g h i j k l m n o p q r s t u v w x y z a
2  c d e f g h i j k l m n o p q r s t u v w x y z a b
3  d e f g h i j k l m n o p q r s t u v w x y z a b c
4  e f g h i j k l m n o p q r s t u v w x y z a b c d
5  f g h i j k l m n o p q r s t u v w x y z a b c d e
6  g h i j k l m n o p q r s t u v w x y z a b c d e f
7  h i j k l m n o p q r s t u v w x y z a b c d e f g
8  i j k l m n o p q r s t u v w x y z a b c d e f g h
9  j k l m n o p q r s t u v w x y z a b c d e f g h i
10 k l m n o p q r s t u v w x y z a b c d e f g h i j
11 l m n o p q r s t u v w x y z a b c d e f g h i j k
12 m n o p q r s t u v w x y z a b c d e f g h i j k l
13 n o p q r s t u v w x y z a b c d e f g h i j k l m
14 o p q r s t u v w x y z a b c d e f g h i j k l m n
15 p q r s t u v w x y z a b c d e f g h i j k l m n o
16 q r s t u v w x y z a b c d e f g h i j k l m n o p
17 r s t u v w x y z a b c d e f g h i j k l m n o p q
18 s t u v w x y z a b c d e f g h i j k l m n o p q r
19 t u v w x y z a b c d e f g h i j k l m n o p q r s
20 u v w x y z a b c d e f g h i j k l m n o p q r s t
21 v w x y z a b c d e f g h i j k l m n o p q r s t u
22 w x y z a b c d e f g h i j k l m n o p q r s t u v
23 x y z a b c d e f g h i j k l m n o p q r s t u v w
24 y z a b c d e f g h i j k l m n o p q r s t u v w x
25 z a b c d e f g h i j k l m n o p q r s t u v w x y
26 a b c d e f g h i j k l m n o p q r s t u v w x y z

0   a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9
1   b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a
2   c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b
3   d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c
4   e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d
5   f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e
6   g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f
7   h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g
8   i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h
9   j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i
10  k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j
11  l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k
12  m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l
13  n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m
14  o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n
15  p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o
16  q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p
17  r s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q
18  s t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r
19  t u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s
20  u v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t
21  v w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u
22  w x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v
23  x y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w
24  y z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x
25  z 0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y
26  0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z
27  1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0
28  2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1
29  3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2
30  4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3
31  5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4
32  6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5
33  7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6
34  8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7
35  9 a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8
36  a b c d e f g h i j k l m n o p q r s t u v w x y z 0 1 2 3 4 5 6 7 8 9
