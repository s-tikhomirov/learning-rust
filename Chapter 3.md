What I don't completely understand about mutability is the notion of variable vs value. When I write `let x = 5;`, I bind the value `5` to a name `x`. Now, what exactly is immutable? If instead of `5` there were some complex type, like enum or something, that could be changed from some other place... that wouldn't work because the value (?) is immutable, or?

OK, this is that whole "there is at most one mutable reference" story, see later chapters.

My understanding is: _variables_ are immutable, which means that it's impossible to change the _value_ that had been initially bound to the _name_ of the variable. Mutable variables allow mutation of the corresponding value, but at most from one place at a time. (Clarify later.)

Constants are immutable always, not just by default (like variables). Type of constant **must** be annotated (type inference doesn't work here, why?).

Shadowing works in scopes: the second name takes over the first name until "it itself is shadowed or the scope ends".

Question: why use shadowing when we can declare variable as mutable? Answer: for a shadowed variable, the final variable **is immutable after all transformations**. So it fits better into the pattern like "convert user input from string to int, but keep the result immutable". Also, we can't change variable type (even for mutable variables!), so we'd have to come up with `result_str`, `result_int`, etc.

Numeric types can be written as `10_000` (separating for readibility). Base can be specified as `0x` (hex), `0o` (octal - who the heck uses octal anyway), `0b` (binary), `b` (a byte: "binary, `u8` only"). Type (signed-unsigned and bit width) can be given as `57u8` (unsigned 8-bit integer).

On overflow: Rust panics on overflow in debug mode **but quietly wraps around in release mode**. That's interesting, why such a design choice? Why not panic in release too?

> Integer division truncated towards zero.

Which means that `-5 / 3` is `-1`. Remainder is `%`. Boolean values are `true` and `false` (not capitalized).

Char is in single quotes, strings are in double quotes.

Compount types: tuple and array. Empty tuple is `unit` type `()`. Two ways to unpack a tuple:
```
let (x, y, z) = tup;
println!("{y}");
```
or indexing from zero: `let y = tup.1;`. Note the dot-indexing (no square brackets!).

Arrays: unlike some other languages, arrays in Rust **have a fixed length**. For unfixed size, use **vector**. Array is allocated on the stack, vector is allocated on the heap.

Initialization of an array:
```
let a: [i32, 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3, 3, 3, 3, 3]
```
The second line infers to `i32` because it's the default integer type.
Array access is done via square brackets: `a[3]`. (Unlike Python, where indexing in both tuples and arrays uses square brackets!)

### Out-of-bounds experiments (questions remain)
What happens on out-of-bounds array access? Rust panics (and exits). The Book says that it's better than, ghm, some other low-level languages we could mention (ghm ghm C ghm). For me it's like... sure, what else could you do? Why didn't you catch this at compile time - ah, because array size may be defined by the user or something, ok, then I guess we'll have to panic.

Question: how do I define a mutable variable but not assign any value to it yet? Answer: Rust catches uninitizlized-use (of course it does) but allows declare first and define later, although this may be considered bad style? I'm not sure why this doesn't compile:
```
let mut array_len: mut String;

io::stdin()
	.read_line(&array_len)
	.expect("Failed to read line");
```
How do I define a mutable string variable without value? (Of course I could use an empty string, but still).
This does not compile either:
```
let mut array_len: String;

io::stdin()
	.read_line(&array_len)
	.expect("Failed to read line");
```
How do I specify that `array_len` is of type "mutable String" without a value yet??

OK, the issue is that `&` is an _immutable reference_, and we need a mutable one. Next iteration:
```
let mut array_len: String;

io::stdin()
	.read_line(&mut array_len)
	.expect("Failed to read line");
```
...but this doesn't compile either, because:
> used binding `array_len` isn't initialized

Why is it necessary to initialize the variable that is used for a buffer we write into? What difference does it make if we initialize it with empty string, as it's going to be overwritten anyway? OK, looks like it contradicts Rust philosophy in ways I don't fully understand yet.

This is very interesting: what I wanted was to cause panic or not depending on user input, if the array is longer than 5, then the fifth element lookup should be fine, otherwise it panics. Or does it? Surprise:
```
use std::io;

fn main() {
    let mut array_len = String::new();
    io::stdin()
        .read_line(&mut array_len)
        .expect("Failed to read line");
    
    let array_len = array_len
        .trim()
        .parse()
        .expect("Index must be an integer");

    let a = [3, array_len];

    println!("{:?}", a);

    let element = a[5];   // this will panic due to out-of-bound access??
    println!("The 5th element is {}", element);
}
```
results in:
```
error: this operation will panic at runtime
  --> src/main.rs:36:19
   |
36 |     let element = a[5];   // this will panic due to out-of-bound access??
   |                   ^^^^ index out of bounds: the length is 2 but the index is 5
   |
   = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `variables` due to previous error
```
Where does it get **the length is 2** from?? I haven't specified anything yet! This is unclear.

But in general, it looks like it's better to always initialize variables. Back to the book.

### Functions
All parameter types must be defined in function definition. (no inference).

**Rust is an expression-based language**, what does that mean exactly? My current understanding: statement is something to be interpreted / executed, while an expression is a value to be passed around. (Both terms are confusing as hell if you ask me.) Indeed (the Book, page 46):
- statements are instuctions <...>
- expressions evaluate to a resultant value.

**Statements do not return values.** Therefore, things like Pythons `if a == b == c` are impossible (unless all are booleans, I guess). Indeed, `comparison operators cannot be chained` - compiler error for `if a == b == c` in Rust. One can however write `if a == (b == c)`, but this is semantically different: it evaluates to `false` if all are `false`.

> Calling a function is an expression. Calling a macro is an expression.

**Expressions do not include endins semicolons.** So ending a block or function body with `x + 1` returns `x + 1`; but `x + 1;` returns nothing (`unit`, or an empty tuple). (Similar to Scala, as far as I remember.)

Return: last expression is the return value; may return early with `return` keyword.

Comments: multi-line comments with `/**/` are _possible_ but considered outdated (I wonder why). The preferred (and only mentioned in the Book) way of doing multi-line comments is to add `//` on every line. There are also documentation comments (`///`).

There is no implicit conversion to boolean on control flow. (So no Python's `if arr` idiom evaluating to `True` iff `arr` is a non-empty array / collection.)

If-else in a `let` is possible (ternary operator):
```
let number = if condition { 5 } else { 6 };
```

Loops: a dedicated (infinite) `loop` (gotta `break` somewhere within) - more straightforward then Python's `while True`. Also: `for`, `while`.

Note: `loop` can return a value on `break`:
```
let mut counter = 0;
let result = loop {
	counter += 1;
	if condition {
		break counter;
	}
}
```
This looks less intuitive: we don't break the counter, we break the loop and _return_ the counter... Could we alternatively just say `result = counter` after we break? Checking... yes, this is totally possible:
```
let mut counter = 0;
loop {
	counter += 1;
	if counter == 10 {
		break;
	}
};
let result = counter;
println!("{result}");
```

Nested loops and labels:
```
`outer_loop: loop {
	//...
	loop {
		break `outer_loop;
	}
}
```

And, of course, the most widely used loop is a `for` through a collection:
```
let a = [1, 2, 3, 4, 5];
for element in a {
	// ...
}
```