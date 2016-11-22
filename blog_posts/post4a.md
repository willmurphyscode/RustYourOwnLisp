Last time, we wrote a regular expression that recognizes "scientific names" (of the *Calvin & Hobbes* variety). But this only
worked because the simple grammar for scientific names is a [regular language](https://en.wikipedia.org/wiki/Regular_language).
If we want to do a more complicated language, or, for example, we want to have maintainable code and not be stuck 
in a regular expression nightmare, we're going to need a better parser. 

In this section of tutorial, I'm going to go *much* slower than *Build Your Own Lisp* goes, for a few reasons. First, I don't
know Rust nearly as well as [orangeduck](https://github.com/orangeduck) seems to know C, and second, I using a parser combinator
in Rust is proving a bit more difficult than I had anticipated. 

I found a [post](http://notes.willcrichton.net/parsing-strategies-in-rust/) about parsing text files in Rust. The post basically
concludes that [`nom`](https://github.com/Geal/nom), which I had originally planned to use, is better suited for binary formats,
and that [LALRPOP](https://github.com/nikomatsakis/lalrpop) might be a little easier to use for this purpose. Specifically, 
LALRPOP has a [helpful tutorial](https://github.com/nikomatsakis/lalrpop/blob/master/doc/tutorial.md#adding-lalrpop) that
shows how to write a program that does a simple four-function calculator, parsing input strings. I'm not going to copy that tutorial;
it's pretty good and I recommend you do it. But since I love Calvin & Hobbes, we're going to use LALRPOP to write a simple 
scientific name parser.

As a reminder, a scientific name, in the Calvin & Hobbes style, is:

1. One or more cool adjectives.
2. A "sciency" noun.
3. An onomatopoeia.

We're going to code our list of cool adjectives, sciency nouns, and onomatopoeias explicitly. In other words, rather than write a function that takes an 
adjective and assesses whether it's "cool," we're just going to type up a list of cool adjectives. Hey, get with the times. 

The cool thing about LALRPOP is that the definition of the language can be *declarative* rather than *imperative*. In other 
words, I get to type up a definition of the result that I want, and then let the computer figure out how to get there. I have 
to files, one is a .rs file that has the definitions of my types, and the other is the .lalrpop file, which lalrpop will turn
into a .rs that parses strings into the types I'm interested in. 

Here's a short sample of each:

https://gist.github.com/willmurphyscode/da2268ce07343d58e8820dd42e38060f

This will be much easier to maintain and more readable than the regex example from last time. 

Next time, we'll talk about how to handle the "One or more cool adjective" part of our language spec, and I'll talk 
a bit about organizing a rust project, and some of the confusion I had with the type system and project system. After that,
we should be ready to build parts of an actual Lisp interpreter. 

Till next time, happy learning.

-Will

