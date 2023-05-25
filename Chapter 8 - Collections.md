> ... stored on the heap... can grow and shrink as the program runs

Three collections that are used very often: vector, String, hashmap.

### Vector
> Vector... puts all the values next to each other in memory

Initialize an empty vector:
```
let v: Vec<i32> = Vec::new();
```

Initialize a vector with values:
```
let v = vec![1, 2, 3];
```

Note: type inference works here, we don't have to write `v: i32` (unless we want another numerical type).

Question: why does it have to be a macro? Why can't we do something like `Vec::new([1, 2, 3])` for uniformity?

Two ways to get an element by index:
```
	let v = vec![1, 2, 3];
    let third: &i32 = &v[2];  // may panic on out-of-bounds
    let third: Option<&i32> = v.get(2);  // more safe; returns Option, requires match
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
```

Access with `[]` is
> best used when **you want your program to crash** if there is an attempt to access an element past the end of the vector

Question: why isn't it _always_ better to access via `Option` and handle an error gracefully? Like, if we really want to crash, we can do it in the `None => ` branch of pattern-match. Is it normal in general to _want_ your program to crash under some circumstance? (As opposed to catching the error and gracefully exiting with some error message or something.)

Ownership / borrowing for vectors applies to all elements: for example, **we can't push to a vector while reading from it**!

```
let mut v = vec![1,2,3,4,5];
let first = &v[0];
v.push(6); // won't compile
```

Explanation: push may require memory re-allocation:

> ...adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space... the reference to the first element would be pointing to deallocated memory.

Question: could it be possible to automatically update the reference, if any, to vector elements?

Iterating with for: `for i in &v {}`. Iterating mutably: `for i in &mut v {}`. Inside the loop, we must use `*i` explicitly for dereferencing.

Vectors store values of only one type. What if we want to store different types? Declare then an enum, and they become one type!

`pop` removes and returns the last element.

Question: so is a vector kinda like a stack? Can we insert / remove element into the middle? Answer: yes we can ([insert](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert)), as well as do many other things -- see documentation.

### String
Implemented as a vector of bytes with extra functionality and guarantees.

Create a `String` from a string literal: `String::from()` or `"blahblah".to_string()`.

Push a string slice to the end of a string:
```
s.push_str("bar");
```
Push a single character:
```
s.push('l');
```

Concatenating strings:
```
let s1 = String::from("Hello, ");
let s2 = "world!".to_string();  // the same as ::from
let s3 = s1 + &s2;
println!("{s3}");
```
Note: addition's first argument is consumed, but the second is not (it's provided by reference).

> The `+` operator uses the `add` method, whose signature looks something like this:

```
> fn add(self, s: &str) -> String {
```

This is very similar to [my implementation](https://github.com/s-tikhomirov/ln-jamming-simulator-rust/blob/2df21142cc7312eff397d973bdbffc5501670b52/src/fee.rs#L26) of applying a fee to an amount! Although for me `self` is not consumed either:
```
pub fn apply(&self, amount: &Satoshi) -> Satoshi {
```

> we can only add a `&str` to a `String`; we **can't add two `String` values together**.

OK, what do we do if we need to add two `String`s? Simply pass the second string by reference so that it becomes `&str`, why is this a big deal? Explanation:

> ...the compiler can _coerce_ the `&String` argument into a `&str`. ...Rust uses _deref coercion_, which here turns `&s2` into `&s2[..]`.

Still looks like no big deal from the developer's (my) point of view. "Here is some clever internal trick atop our type system that allows string concatenation to work _nearly_ as you'd expect it to work." Thank you very much!

Question: what it we want to keep both `s1` and `s2`, i.e., **not** consume `s1` while adding `s2` to it? Is this the way?
```
let s11 = s1.clone();
let s4 = s11 + &s2;
println!("{s1}"); // this works: s1 wasn't consumed
```

Concatenate multiple strings:
```
let s = format!("{s1}-{s2}-{s3}");
```
Question: what gets consumed here and what doesn't? Answer: nothing is consumed:
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
println!("{s}");
println!("{s1} {s2} {s3}"); // this compiles!
```
Indeed:
> `format!`... uses references so that this call doesn't take ownership of any of its parameters.

**Strings don't support indexing!**. We can't do `s[0]`. Why? Because Unicode! Latin letters take 1 byte each, Cyrillic take 2 bytes. What shall we return as `s[0]` of a Cyrillic string? Hell knows.

And it gets curiouser and curiouser! With bytes vs scalar values vs **grapheme clusters**. Thinking about this, isn't this a miracle that computers can handle more or less all human languages at all.

Now, we can't index into a String, but we can slice it! 
```
let hello = "Здравствуйте";
let s = hello[0..4];
```
The result is `Зд` because, as said earlier, each Cyrillic symbol occupies 2 bytes. I have to know this before I slice a Cyrillic string.

Iterating over a string: we must specify whether we want characters or bytes:
```
for c in s.chars() {}
```
or
```
for b in s.bytes() {}
```
Getting grapheme clusters is not provided by the standard library.

### Hash maps
```
let mut scores = HashMap::new();
// because they are inferred from here:
scores.insert(String::from("Blue"), 10);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
`copied()`: get an `Option<i32>` from `&Option<i32>` that `get()` returns.
`unwrap_or()`: a more compact form of pattern-matching: return value if `Some` or `0` if `None`.

Iterate over a hashmap (will print in **arbitrary order**):
```
for (key, value) in &scores {
	println!("{key}: {value}");
}
```

Ownership: types that implement `Copy` are copied, others are **moved** (the hashmap will be the owner of those values).

Updating a map: three cases:
- overwrite: `insert`;
- keep old value (add only if key didn't exist): `.entry(k).or_insert(new_v)`;
- update: see `hashmaps_counter` example in `projects/collections`.
