# Minilang

A simple implementation of a programming language interpreter, built to learn Rust.

### The language

Here is a program showing the istruction set:

```
# this is a comment
# only one istruction per line
# here is an assignment:
@1 := 42
# it puts the number 42 in the memory cell 1. 0 is the first memory cell.
# This prints to stdout the value in the cell 1 (42)
write(42)
# This reads a line from stdin, converts it to an integer and
# puts into in the "4" cell
read(4)
# This language only understands integers, no real numbers allowed for now
*1 := 15
# This above is a pointer: it doesn't put "15" in the "1" cell, it stores "15"
# inside the cell pointed by the value in cell "1". So since cell "1" contains
# "42", "15" will be stored in cell "42".

# This below is a label. It allows you to jump to this point
start:
# You can place a label on its own line or before an istruction, like this:
useless: pass
# the "pass" istruction does nothing.
# You can get back to a label by using the goto istruction
goto start

# All cells contain value 0 unless another value is assigned.
# There is no true expression parsing, but you can do this:
@2 := @1 + 10
# Only one operation allowed per assignment. These operators are implemented
# + - * / % ^
# ^ is the power operator. 2^3 is 8.

# Conditions
if @1 > 5 then goto start
# You have these operators: > < >= <= = ==
# The = operator and == are the same

# You can freely indent your code, like this:
          indented: pass

# The halt istruction immediately stops program execution
halt
# The execution also stops at EOF and jumps targeted at non existing labels.
```

The program doesn't work because of infinite loops, but it shows the istruction set just fine.

### Running the interpreter

Just clone this repo, then run `cargo build` to compile the interpreter. `cargo run <arguments>` runs it.

The interpreter takes exactly one argument which is the filepath to the program to execute.

The interpreter is very slow, badly written and full of bugs but it's not really intended to be useful,
however it did help in debugging a simple program full of gotos written in similar pseudocode.

The few programs I built are not included because I changed the istruction set and I only have one program that works.
