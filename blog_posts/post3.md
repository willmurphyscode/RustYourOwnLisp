Last time, we built a simple REPL, the incredibly annoying echo. This program just echoed the user's input 
back to the user. Now, I'd like to actually do something simple with the input. We're going to need to 
*parse* the input into some sort of language. Now, languages can get pretty complicated. English, for example,
is an utter nightmare to parse, because we don't have that many rules, and we have so many symbols have multiple
meanings (e.g., "I may start work for my cousin May in May.). This makes it hard to write a simple parser for 
English.

Therefore, in keeping with the Calvin & Hobbes theme of the series, we're going to write a parser for a much simpler language:
"scientific names". There's a Calvin & Hobbes strip where Calvin opens a lemonade stand that sells "scientific names," offering
such samples as "the horrendous space kablooie" in place of the "Big Bang." 

So we need a grammar for these scientific names. I think the best grammar is as follows:

1. At least 1 "cool" adjective ("horrendous", "incredible", "monstrous", "ultimate" etc.)
2. A science-sounding intermediate noun ("space", "time", "matter", "energy")
3. An onomatopoeia that makes a cool sound ("boom", "kablooie", "swoosh")

Any expression of that form is a valid "scientific name", so now we can write a REPL that tells the user whether they have input a valid 
scientific name. 

This language is dead simple, so a function that validates it is equally simple:

https://gist.github.com/willmurphyscode/59e5800026b60b8d18cca5508469b034

We just typed up the whole rule for the language in a single regular expression. Now, some people have done truly heroic things 
with regular expressions, but maintaining the definition of a programming language in this form is not really feasible for languages
that have any complexity at all. Also, we don't know anything about the input string except that our regular expression matched it, 
and we'll need more information if we want to interpret a language. 

A first instinct might be to start splitting up strings and writing loops to go over the pieces and check whether the character at position 
7 is a left bracket and also whether the character at position twelve is a minus sign, but that's going to get miserable after about 32 seconds,
and it's going to be buggy and hard to read.

What we really need is a special set of functions that is built *for* describing languages. Given that set of functions, we can start to build up
a description of our language in a way thatt is easy to read and easy to update. Such a set of functions would be pretty hard to write. It would
be called a parser combinator and, thank God, someone already wrote one in Rust: 

So next time, we'll use this parser combinator, called `nom` to write a parser for our tiny "scientific names" language. After we've done that,
we can start writing a definition for Lisp, which is not quite as cool as scientific names, but still pretty neat. 

Till next time, happy learning!

-Will