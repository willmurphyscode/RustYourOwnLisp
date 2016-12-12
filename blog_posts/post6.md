This is another detour post.

Last time, we convincer LALRPOP to parse out some scientific names. We 
even have a parser that handles 1 or more cool adjectives at the begining. The
parser seems pretty maintainable, and I like LALRPOP. It's easy to read and
write. Now, it's doing a great deal of code generation under the hood to 
make loops that split up strings and do heroic things with regular expressions
or something, and I'm happy to let it. 

I sat down to write an actual parser that would parse the [s-expressions](https://en.wikipedia.org/wiki/S-expression)
that make up Lisp and at least give  us a four function calculator that used
prefix notation. But I realized halfway through that I don't understand
Rust's memory model well enough to make a tree, and if I want to play 
with Lisp I've gotta make a tree. 

I thought I'd start simple and just make a linked list at first, since that 
will give me a way to represent a value that may or may not have a reference
to the next value. And thank God that I decided to Google "linked list in 
Rust" because I found [this wonderful tutorial](http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html).
The author will teach us to write Rust in 
