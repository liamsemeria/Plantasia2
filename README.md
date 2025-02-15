# Plantasia2
An interpreter in Rust for a more advanced version of my language Plantasia. It can be run with `cargo run (source file)`, and you can perform tests using `cargo test`

## How it works.
### The basics
There are no variables, only stacks of integers represented as lines that go down the program. As long as a line stays intact the values are kept. The stacks are modified by pushing values onto them or by modifying the top of the stack with an operation. The source of such instructions are popped from their source stacks.
```
|<<12
|  |<<8
|  |
|<+|
.
```
In the example above, at line 3 the stacks contain [12] and [8] respectively. Line 4 pops the 8 and adds it to 12, resulting in the stacks [20] and [].

Instructions can go in either direction, the direction is specified by using `<` or `>` to point  to the destination.\
For example `   |<-3` is the same as `3->|`, they both decrement the same dest stacks top value by 3. Note that here a constant is being used, you can imagine it as a temporary stack with size 1.

The operations in an instruction are as follows:\
`<` or `>`: pop from the source stack, it needs to point to the dest stack.\
`#`: propagate i.e copy from one stack to another.\
`+,-,*,%`: pretty self explanatory.\
`=`: in normal code it would look like `dest = dest == src`

Some other things:\
`.` is the return instruction that returns its entire stack.\
`$` is the input stack that can be specified while running the interpreter.\
`_` is a character that gets ignored by the lexer so you can use it for readability purposes.\
';' is for comments it looks bad but // and # felt weird

### Control flow

For control flow, its better to just jump straight into an example.
```
    $
    |
|<<6|
|   |<%2
|   ?[
|- >|   
|   ]
    .
```
The above program subtracts 6 from the rightmost stack if its value is even. It’s a dumb program but it gets my point across. The `?` instruction is an if statement that checks to see if the stacks value is greater than 0. If so then it executes the branch specified by the square brackets. The values from other stacks are brought through the branch and can be brought in from the left or right side.

One constraint is that the stacks that are alive before a branch must be the same as the branches alive after. This makes sure that there isn’t a mismatch during the merging of control and prevents programming some cases for nasty infinite loops.

The `:` instruction functions the same as `?` except its a while loop, it executes the branch as long as the top value is above 0. This is probably a big jump in complexity, but below is an example of using a while loop to output the first 5 values of the fibonacci sequence.
```
|<<3
|   |<<0
|   |<<1
:[  |
|<_<|
|   |#>|
|#_>|  |
|>_>|  |
|   |<+|
|<-1|
]   |
|   .
```
That's pretty much it, this language is tedious but fun to use since you need to keep track where all the values are and you need to think ahead when deciding how far apart stacks are from each other. A good way to approach the language is keeping more long term values on the left kind of like busses in computer architecture.


