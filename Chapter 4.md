**Ownership enables memory safety without a garbage collector** - let's think of what this means exactly. A garbage collector track the number of references to a value and prunes it if that number drops to zero, right? What's the _fundamental_ difference to ownership?

Ways to manage memory:
- GC: runs **periodically** and cleans up memory. The Java way. (This means, that for _some time_ some memory is occupied by already useless stuff.)
- Explicit allocation and freeing: the C way. (Manual work, error prone, memory leaks, overflows etc).
- Ownership - the Rust way. The rules are checked **at compile-time**: non memory-safe code won't compile (unless it's marked as unsafe or something?).

OK, the fundamental difference to GC is that ownership is enforced at compile-time. And the difference to explicit allocation is that the rules are more... high-level? Elegant? The programmer thinks in terms "this variable owns that value", not "I allocate X bytes for this array, gotta remember to free it later but not too early either."

### The stack and the heap
Stack: LIFO.

> All data stored on a stack must have a known, fixed size.

This means that only "primitive" types can go on a stack?.. (For some definition of primitive.) Like, a complex data structure with linked lists inside can't go on a stack, or? Can I put a _reference_ to a floating-size list that is itself stored elsewhere (on the heap, I guess)?

Meta-question: why are the stack and the heap the only two memory structures? Is this defined by the processor architecture? Why don't we have a queue (FIFO), for example?

> Because a pointer to the heap is of known, fixed size, you can store the pointer on the stack

-- this answers my questions above.

**Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data.**

Relation between stack and function calls: function arguments and internal variables are pushed onto the stack.

> When the function is over, those values get popped off the stack.

My mental model is that a function takes over the stack, pops the arguments, does its job, and pushes the return value onto the stack for the caller to access. Is this the same thing? Where do local variables fit here? As I see it, they get pushed and popped within function execution, the caller (obviously) knows nothing about them. But the wording in the book is ambigious, in that: "popped off the stack" means "popped and thrown away", "popped and used elsewhere", or both depending on what the value represents?

### **Ownership rules**
0. ~~Never talk about ownership rules.~~
1. Each value has an owner.
2. There can only be one owner at a time.
3. (Last) owner goes out of scope => value is dropped.

OK, how could it be otherwise? An owner here is a variable, right? Then, _obviously_, each value has a name that refers to it, otherwise how could we use it at all? The third rule is also logical: what's the point of keeping a value around that can't be used anymore? If there was a single owner, and it no longer exists (out of scope), there is no way to resurrect it, AFAIU, so the value is useless. This is kinda what GC does in Java, but at runtime.

The key rule seems to be rule 2: **there can only be one owner**. What implications does it bring? Like, if I want to use a value elsewhere, the prior owner must... give it up? Go out of scope? Let's read on.

### Examples of ownership
```
let s = "hello"
```
This is a **string literal**, hardcoded into the binary and immutable. 

Question: why is the type `&str`, what does `&` mean? It's hardcoded, but it's still a... reference? Of `&` doesn't necessarily mean a reference?

Further examples use `String` type, which is heap-allocated.

```
let s = String::from("hello");
```

OK, this is a heap-allocated string, but... We were told that values are immutable by default, so can we append to this string? Possible answer: the _reference_ is immutable, but the _data that is stored there_ is not. Like, internally, `s` will always remain "chunk of memory starting with `oxdeadbeef`", but its size differs. But that likely isn't true ether: if we grow the string, it can grow past the size of that continious chunk of free memory, and the heap re-alloccates it to a larger chunk now starting at `0xcafebabe` - is this how it works? TLDR: I don't understand the relation between variable immutability and heap-allocated data types.

Now, the string above creates an **immutable** string. If we want to append to it, we do:
```
let mut s = String::from("hello");
s.push_str(", world!");
```
which means that without `mut` this could only be useful for, like, storing user input (size unknown at compile-time), but never modifying it later? Fair enough.

### Rust vs GC
I don't understant why what Rust does is not a GC. For me it sounds like an efficient GC, but a GC anyway. Like, either the programmer has to write `free()` or he doesn't. In the latter case, who frees up the memory? GC does. The difference is that e.g. JVM GC does it only when the memory runs low. Like "we have 4 GB, 1 GB is occupied with garbage, but we don't care, let's continue executing." And only if we hit 3.5 GB or something, then GC steps in and frees it up (at the cost of performance at that moment).

What Rust does differently is it frees the memory _exactly_ when it's no longer used, right? When the owner goes out of scope, Rust immediately "garbage-collects". The difference is that as long as there can be at most one owner, tracking it is simpler than reference-counting, so it can be done immediately. Which makes the memory footprint smaller. The trade-off is that the rules of ownership are stricter and enforced at compile-time, so the programmer can't be as sloppy.

**Is the above true at all? If so, why does Rust boast having no GC, when one could arguably say it has a better, more efficient GC (powered by stricter compile-time ownership rules)?**

The Book:
> When a variable goes out of scope, Rust calls a special function for us ... called `drop`.

This `drop` function... is the part of Rust? Can I call it manually? Answer seems to be: yes, I'd use it if I'd develop my own type like `String`. I make this conclusion from:

> (`drop`) is where **the author of `String`** can put the code to return the memory.

But what would that code look like? Some kind of inline assembly or C?

### I like to move it, move it
This copies a primitive type to another stack slot:
```
let x = 5;
let y = x;
```

This copies the **reference** but the data stays at the heap where it has been allocated (no copying!):
```
let s1 = String::from("hello");
let s2 = s1;
```

A `String` is a stack data structure with: name, pointer (to the heap where the actual contents are stored), length, and capacity. Capacity is how much has been allocated; length is how much is currently in use. Question: why not free up what is not used, if `len < capacity`? Possible answer: because we can only free up the whole thing, we can't free up just one (unused) field of a struct or something like that. If so, why?

On `s2 = s1`, **the pointer is copied, but the data is not**. (Illustration on page 65.)

The next question the Book discusses: how to avoid double-free problem? If `s1` goes out of scope, and then `s2` goes out of scope, when do we free the memory? (My note: this is exactly what a GC does, isnt' it? Counting references until the counter hits zero.)

Rust's answer is:
> ...after the line `let s2 = s1;`, Rust **considers `s1` as no longer valid**.

In other words, "reference counting" is done in such a way that the counter never exceeds one.

**Move** = shallow copy + invalidate the old reference.

> Rust will never automatically create deep copies of your data.

Question: does it mean the same as "if your custom data structure includes even one field that is stored on the heap, not on the stack, its contents will be moved, not copied"? Is it true that if, say, all fields in my struct are "primitive types" stored on a stack, it will get copied?

Deep copy is done with `.clone()`.

Note: `String::from()` is a... class-level (?) method, hence `::`. Method on a concrete object is called with `.`: `s1.clone()`.

Indeed (page 68): primitive types are copied.

`Copy` and `Drop` are **traits**. Types that implement `Copy` are, well, copied. However, a type can't implement `Copy` and `Drop` at the same time. `Drop` is required for heap-allocated types. (Is it true that "a type implements `Drop`" <=> "some data is stored on the heap"?)

> ...any group of simple scalar types **can** implement `Copy`, and and nothing that requires allocation or _is some form of resource_ can implement `Copy`.

Question: so a group of scalar types doesn't _have to_ implement `Copy`? If it doesn't, then how is it copied?

A resource in this context, as I understand, it e.g. a web connection, a printer handle, something of that nature.

### Ownership and functions
Functions take ownership of their arguments.
```
    // initialize an immutable string allocated on heap
    let s = String::from("hello");
    take_ownership(s);
    // does function return ownership?? no!
    //println!("{s}");  // this doesn't work

    let x = 5;
    make_copy(x);
    println!("{x}"); // this is OK
```

OK, now it's time to think what ownership really means. Does it mean that the owner is the only one who can read the value? Or write? Or both?

In the above example, the function took ownership of the string and printed it. Question: why doesn't ownership return automatically after the function terminates?

A function can return ownership via... `return`. (Or returning without the keyword, by last-line expression without `;`).

```
fn take_and_give_back(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

let s = String::from("hello");
let s = take_and_give_back(s);
println!("{s}");
```
Note: we can't do `s = take_and_give_back(s)` because `s` is immutable! But we can use shadowing.

**Assigning a value to another variable moves it.** It would be too much ceremony to do this every time, that's why we have references.

### References and borrowing
Example: a function calculates a string length. With ownership, it must return the length **and** the string itself, otherwise the ownership won't be transferred back. References allow doing it more elegantly. Using the value without obtaining ownership, that is.

> A reference... is an address we can follow to access the data... Unlike a pointer, a reference is **guaranteed to point to a valid value of a particular type** for the life of that reference.

So what it says is basically: references are always typed. There isn't any C-style `*`-pointers with crazy type-casting and shit.

Question: what would be an "invalid value of a particular type"? Does it mean that while there is at least one reference in the scope, the value won't get freed? What if the original owner goes out of scope, but a reference doesn't? Is it impossible?

Calculate string length without giving ownership (aka borrowing):
```
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of {s} is {len}");
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}
```

The opposite of referencing (`&`) is dereferencing (`*`). I'd guess that as long as all references are typed, the dereferenced value has the corresponding type automatically, no issues there.

> Because (the reference) doesn't own (the value), the value ... **will not be dropped when the reference stops being used**.

This is the key distinction: a reference may go out of scope, but the value remains in memory (until the owner goes out of scope).

References are immutable by default.

To modify a borrowed variable, use a mutable reference `&mut` (the variable must be mutable too!).
```
fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of {s} is {len}");
    change(&mut s);
    println!("The length of {s} is now {}", calculate_length(&s));
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

**There can only be one mutable reference to a value.**

So the value can only be modified either by the owner, or by the (single) mutable reference. (...to which control flow has been transferred? Gotta understand why "two places" that can change a value are not a problem, namely, a `&mut` and the original owner.)

> We _also_ cannot have a mutable reference while we have an immutable one to the same value.

This is important! **While someone somewhere can potentially read the value, there is no way to modify it!** (What about the original owner, can it still mutate? Probably not but I don't understand why exactly.)

This indeed doesn't compile:
```
let r1 = &mut s;
s.push_str("!!");
println!("{}", *r1);
```
with
```
cannot borrow `s` as mutable more than once at a time
```
OK, so `push_str` is a... function that tried to borrow s (given not as an argument, but as an object the function is called on, but I guess it doesn't matter in this context...)

Question: if a function is called on an object, is it true that the object is borrowed by that function?

**The scope of a reference ends after its last usage.** This means that we don't have to wait until the `}` to mutable-borrow something that had been immutably-borrowed earlier in the scope, if the immutable-borrow is no longer used:
```
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}"); // note the absence of *
let r3 = &mut s;  // this is OK: r1 and r2 no longer used
println!("{r3}");
```

Question: why `println!("{r1}")` and not `println!("{*r1}")`?

Rust doesn't allow dangling references (related to lifetimes, Chapter 10).

References recap:

> At any given time, you can **either have one mutable reference, or any number of immutable references**.

The second rule I don't understand:
> References must always be valid.

What does "valid" mean here? Doesn't the compiler ensure that references are valid, that is, not dangling? The compiler also ensures that the types check out. So what do _I_ have to do to follow the above rule? Or is it descriptive, not prescriptive, like, "this is how Rust behaves"?