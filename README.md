# Advent of code: Rust

This is the a solution to the [advent of code](www.adventofcode.com) implemented in Rust.

## Why rust

Advent of code is a great opportunity to learn a new programing language. My first intention was to become proficient using Haskell, so I started implementing the solutions with it. However, because it relies a lot on interpreting Intcode, being unable to use mutability becomes annoying very fast. Of course you can find some workarounds using hashmaps and whatnot, but in the end instead of the terse beautiful code Haskell promises you end up with a convoluted mess. 

- But what if it looks hard to read because you are not good enough?

Yes of course I lack the experience, but Haskell does feel too... academic. Everything feels too hard and ugly until it starts looking better, and then you realize that all your beautiful code does is simply desugar a `while` loop...

All in all, i think Haskell is a beautiful language that has a very sexy type system with a decent syntax and style, but it just feels painful. Getting started with it is hard, explanations are cryptic, records are annoying and compiler (GHC) errors are even worse than C++ template errors. There is a wonderful language inside Haskell held back by purists.

Rust on the other hand is a language that promises C++ efficiency and power, with the beauty of \*gasp\* Haskell. But oh boy, if Haskell feels complicated at first, get ready to receive a beating from rustc. The compiler will complain for every. single. thing. But it teaches you *so much*. The messages are incredibly helpful and you can pretty much learn the language and fix the mistakes just by following the indications (most of the time).

 Even though it allows mutability by using lifetimes and borrow mechanics (which **will** drain your will to live) it is still a extraordinarily secure language (take that C). Add in its wonderful type system (traits, ADTs, structs, tuple types, etc) and you get an iron-clad practical language. And even if the syntax is not the most beautiful (particularly coming from Haskell), for a lower-level language the syntax it is quite clear, and well thought. Pattern matching is fantastic (take that C++, again) and after using it a while you really miss it with languages that lack it, function definitions (even with generics!) are alright and code blocks feel just right.

I will keep updating this while I learn more about the language.

## Completion
- [x] day 1
- [x] day 2
- [ ] day 3
- [ ] day 4
- [x] day 5
- [x] day 6
- [x] day 7
- [X] day 8
- [ ] day 9
- [ ] day 10
- [ ] day 11
- [ ] day 12
- [ ] day 13
- [ ] day 14
- [ ] day 15
- [ ] day 16
- [ ] day 17
- [ ] day 18
- [ ] day 19
- [ ] day 20
- [ ] day 21
- [ ] day 22
- [ ] day 23
- [ ] day 24