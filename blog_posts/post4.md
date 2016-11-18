Last time, we invented a super-simple language. ("Scientific Names" in a nod to Calvin & Hobbes). We 
used a simple regular expression, which is a little more compact than just hard coding the list of 
valid expressions in our language, but basically does the same thing. I mentioned that maintaining 
the language would be hard, since editing regular expressions is notoriously error prone anyway. I 
also failed to mention the detail that for more complicated languages, it's possible to exceed
categorically the sort of language that could be matched with a regular expression. English, for 
example, or Java, are both too complicated to make a regular expression that matches only valid
phrases in the language. Wikipedia has a more [complicated explanation](https://en.wikipedia.org/wiki/Regular_language).
Occasionally people ask StackOverflow how to write a regular expression to parse HTML, and they 
are basically given this answer: "You're using the wrong tool, HTML is inherently more complex
than a regular expression can match. There's probably a library that just parses HTML in your 
language so use that."

(Quick etymological aside, since I used to teach Latin and I can't resist: A *regulum* in Latin is a 
rule, so a "regular expression" is a way of writing down rules. A regular expression is shorthand for a 
bunch of rules like "expressions can contain digits" or "expressions must not contain the letter A". A 
regular language is a language whose grammar can be specified by this sort of simple rule. So regular
expressions express rules. End aside.)

And that's a very long-winded introduction to this post, which should start: "Help! I need a tool 
more versatile than regular expressions for defining a language." Enter the parser-combinator library.

I asked twitter what parser combinator library I should use in Rust. I got some feedback, but in particular
the author of `nom` answered me and said he was interested in my project. I thought that was very kind of him,
and `nom` seems to be a healthy library (regular commits on GitHub), so we're using `nom`. 

So how to get started using a new library? I like the following method: Find the unit test project for that 
library, copy a test file, and try to modify it. So that's exactly what I did for this post. `nom` has a unit 
test for parsing floats out of strings. That seemed suitably simple, so I decided to modify it for parsing ints. 
  





