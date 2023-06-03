# Generics
Relation between generics and types (one of, I guess?) is that traits can limit the possible types that a generic accepts. In fact, I alrady faced this use case, when for example a Hashmap demands that the key type is hashable (i.e., implement the corresponding trait).

Lifetimes are a variety of generics that give the compileer information about how rfeerences relate to each other**. Hmm.

Signature of a generic function that finds the maximal element in a list:
```
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
```
Note: we use `<T>` in function **name** as well as argument types. Moreover, we specify that type `T` here must implement the ordering trait (items are comparable).

Question: are there other ways to implement comparison? Like, can we order a list of integers by their number of prime divisors or something like that?

In struct definitions:
```
struct Point<T> {
	x: T,
	y: T,
}
```
Note: this enforces that `x` and `y` are of the **same** type.

A struct with different types:
```
struct Point<T, U> {
	x: T,
	y: U,
}
```
Question: does this prohibit same types? Let's check:
```
let s = Point{x: 1, y: 2};
println!("{} {}", s.x, s.y);
```
This works!

In `impl`, we **must** add `<T>` after `impl`:
```
impl<T> Point<T> {
	//fn ...
}
```

To implement functions only on some **concrete** type, we use a regular `impl` block with normal type definitions:
```
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		// ...
	}
}
```

Similarly, for a function within an implementation that takes a generic over some _other_ types: we need to specify `mixup<X2, Y2>` so that the compiler understands that `X2` and `Y2` are too generics (Listing 10-11, page 190).

On compilation, Rust expands generics into concrete implementations, as if copied by hand => no runtime overhead.

# Traits
> A traits defines the functionality a particular type has and **can share with other types**.

> Trait definitions are a way to **group method signatures together**

example: a tweet and a news article can both be summarized via this trait:
```
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Question: we don't have to specify `pub` here? Implementations of functions from a trait can be private?

Implementation for `NewsArticle` type:
```

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
        "{}, by {} ({})",
        self.headline,
        self.author,
        self.location
        )
    }
}
```

Question: if a trait contains many functions, must they all be implemented in one `impl` block? Answer: probably not ("conflicting implementations" error).

> ...we can implement a trait on a type only if **either the trait or the type, or both, are local to our crate**.

Indeed, otherwise we could have conflicting implementations.

Question: why is this restriction sufficient for preventing this? Let's say we have crates A and B. Crate A has trait A, and crate B has type B. we implement an implementation in both crates, as condition holds: either the trait or the type are local to both crates. What happens?

we can define a default implementation inside the trait definition:
```
pub trait Summary {
    fn summarize(&self) -> String {
	    String::from("(Read more...)")
    }
}
```
And specify an empty `impl` block:
```
impl Summary for NewsArticle {}
```

> Note that it isn't possible to call the default implementation from an overriding implementation of the same method.

## Traits as parameters
We can implement a function that takes as an argument _some_ type that implements a trait:
```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
A longer form aka **trait bound**:
```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
Trait bound is useful for e.g. two arguments of the _same_ generic type (or more complex conditions on types of arguments):
```
pub fn notify<T: Summaty>(item1: &T, item2: &T) {
	// ...
}
```

Require multiple traits:
```
pub fn notify(item: &(impl Summary + Display)) {
	// ...
}
```
Or:
```
pub fn notify<T: Summary + Display>(item: &T) {
	// ...
}
```

If trait bounds are too long, use `where` (see page 198).

Return type that implements a trait:
```
fn returns_summarizable() -> impl Summary {
	// ...
}
```
Note: this doesn't allow for returning _multiple_ types, only one type that implements the trait.

Using trait bounds to conditionally implement methods:
```
impl<T: Display + PartialOrd> Pair<T> {
	// ...
}
```
Works only for pairs with display and partial order (see page 200).

Blanket implementations: implementation for a type that implements another trait (example: `to_string()` for any type with trait `Display` in the standard library).

# Lifetimes
> Lifetimes are another kind of generics ... ensure that references are valid as long as we need them to be.

> The main aim of lifetimes it to prevent dangling references

Wait, I thought the compiler takes care of this automatically... Indeed, with a borrow checker (example pp. 201-203).

> Lifetime annotations **don't change how long any of the references live**. Rather, they **describe the relationship of the lifetimes** of multiple references to each other without affecting the lifetimes.

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
```
-- this means that the function takes too parameters that live for at least as long as `'a`. Again, this is just an annotation for the compiler!

> The lifetime parameter of the return value needs to match the lifetime parameter for one of the parameters.

Otherwise, it would have been a dangling reference!

> Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

Lifetimes in structs can allow structs to hold references (so far we've only seen structs that own their fields) - see p. 209.

Elision rules: some common lifetime heuristics are hard-coded into the compiler, where explicitly annotating would be tedious. (Kinda like blockchain pre-compiles.) Three elision rules - see pp. 210-211.

Static lifetime (`'static`): the reference **can** live for the duration of the program. Again, whether it does or not is determined by the program, but the borrow checked will assume that it does.




