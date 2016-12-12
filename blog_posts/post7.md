I saw a Christmas tree the other day, and it reminded me that I should be working on this tutorial.
And speaking of trees, how do we build a tree in Rust? Trees are easily my favorite data type 
(and holiday decoration), so I am pretty excited to write this post.

Why are we building a tree? The first sentence of the [Wikipedia article](https://en.wikipedia.org/wiki/S-expression) 
on s-expressions, the building blocks of Lisp, tells us to, but let's think about why. 
Here's the definition of an s-expression from the same article: 

1. an atom, or
2. an expression of the form (x . y) where x and y are s-expressions.

This definition reminds me of another definition, also from [Wikipedia](https://en.wikipedia.org/wiki/Binary_tree#Recursive_definition). A binary tree is: 

1. A single vertex.
2. A graph formed by taking two (full) binary trees, adding a vertex, and adding an edge directed from the new vertex to the root of each binary tree.

Those definitions are basically the same. We have the base case (atom or single vertex), and a way of nesting the structure,
and we're off and running. 

Algorithms for processing trees are pretty cool. You can have a program like, "Are we in the base case? If so, emit a 
value / do a thing, otherwise, process each sub tree." 

So now we know that trees are cool. Let's gain a little intuition about why trees are good for mathematical computation.

Here's a made up equation: `x = a + b`. What are `a` and `b`? In high school they're numbers, but they could be any
mathematical expression. We could say `a = 12 + 32c + n/5`. Then we could say `c = 12n + 4` etc. At every level of 
our math expression, each term could be either a value (atom/single vertex) or a set of terms that can be combined in 
some way. The whole "every item is either a basic item, or else a combination of items" definition, wherever we find it,
strongly suggests that we can represent the problem with a tree. Algebraic expressions can be represented as trees. 

Ok, enough philosophizing. Let's make a tree in Rust. We're going to build a syntax tree that can represent basic 
arithmetic operations. We have the rule "every vertex is either an atom (in this case, a number or an operation) or a 
pair of vertices."

//s_expression_types.rs goes Here


That all looks fine. The other thing I'd like is to be able to invoke "evaluate" on the root of a tree and have 
that tree do all the math and return a number. This part will be a little tricky because Rust is so concerned with 
memory safety, we'll have to persuade the compiler that we're allowed to do this. So here it goes. 