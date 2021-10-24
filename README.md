# Rusty Brainfuck Compiler

## Notice
This compiler was initially built as I was learning Rust more in-depth, and as such, some of the code may look wrong or may not be quite as optimized as it should be, but the program should still work.

# Description
Rusty Brainfuck Compiler is a compiler for the programming language built entirely in Rust. The way it compiles brainfuck is to convert it to an intermediate layer known as BFM and then pass it to the script builder, which converts BFM to C, soon after compiling with GCC that makes the .exe executable for anyone to run and enjoy. This was a small project meant to see if I could even create a proper interpreter and compiler for BF since I had many issues doing it (with C++) in the past. But it turned out victorious, and now it's here for everyone to enjoy!

## Prerequisites
- GCC 8 or higher
