Last time, we made a (very) simple parser with LALRPOP that parses Calving & Hobbes style scientific names. 

It took me a little while to work out how to use Rust, Cargo, and LALRPOP to get this working, so this time 
we're going to take a brief digression to talk about getting Rust code to work. 

For contrast, I'm going to compare the setupu with a .csproj file, which a file that Visual Studio uses to keep 
track of C# projects. A .csproj file is basically a manifest. It has lines that say like "The project you've trying 
to build depends on this DLL, so go find that" and "Include this .cs file when you're building." When you ask Visual
Studio to add a dependency or include a source file, it modifies .csproj for you. 

Rust doesn't have a .rsproj file. At first I thought that Cargo.toml would fill the same role. I was partly right, but 
Cargo.toml declares build steps and dependencies. But Cargo.toml doesn't include a list of source files, and I spent 
a few minutes on the question of "how do you make a Rust project that has more than one file." Everything I found googling
seemed to imply that I could already do this, and wanted to do something more advanced.

Anyway, so here I am, I have three source files in my Rust executable project. Let's just call them main.rs, point.rs and 
rectangle.rs. main.rs has the entry point, that is, the function that is invoked when the program is first execute. point.rs 
has some code for defining points on a plane, and rectangle.rs has some code for defining a rectangle on a plane. 

point.rs has a struct called `Point`, and I remember from somewhere that the Rust compiler by default wraps stucts in a file
in a module named for that file, so I try `use points::Point` at the top of rectangle.rs. So far so good. I want to use rectangles
in my main function, so I type `mod rectangle;` at the beginnging of main.rs.

My project looks like this: 

https://gist.github.com/willmurphyscode/403440d84d327ef91987bee28af49948

Running `cargo build` fails with this message:

    src\rectangle.rs:1:5: 1:17 error: unresolved import `point::Point`. Maybe a missing `extern crate point`? [E0432]`

What's going on? I checked all the obvious things. Did I misspell `point`? Am I in the wrong directory? The full "am 
I missing something silly" checklist. 

Here's what I'm missing: The difference between `mod` and `use`. 

The `mod rectangle;` at the beginning of main.rs basically says "Hey go find a module named `rectangle` and *compile it here*. 
In other words, `mod rectangle;` is telling rustc that rectangle.rs is a file I care about. 

rectangle.rs wants a type called `Point`, but *nothing is telling rustc to include point.rs*. 

The answer was to include `mod point;` at the top of main.rs. 

I was a little frustrated here. I'm missing a `mod` statement in main.rs, so the compiler fails with an error in 
rectangle.rs that's complaining about `extern create` nonsense. 

So, to make a long story short: `use` just brings names into scope. `mod` is used in your main fail to tell the compiler about
other files you care about. This works fine:

https://gist.github.com/willmurphyscode/236837dd8edf61cf8070b28daa62339d

I'm sure that someone more experienced with Rust will be able to say more about `use` and `mod` and their difference, but for
now knowing that I need a `mod foo` statement in main.rs if I want the compiler to include foo.rs in the build is good enough.
Also, knowing that if one of my files is complaining that it can't find a type defined in another file, the probable 
explanation is that I failed to tell main.rs about the second file. Next time we'll get back to building something interesting. 

Also, special thanks to this [StackOverflow](http://stackoverflow.com/a/26225057/3896861) question, which helped me understand. 


Till next time, happy learning!
-Will
