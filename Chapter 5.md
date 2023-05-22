> Structs and enums are the building blocks for creating new types...

Defining a struct:
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Initializing:
```
let user1 = User {
	active: true,
	username: String::from("Satoshi"),
	email: String::from("satoshin@gmx.de"),
	sign_in_count: 1,
};
```

Printing struct fields:
```
println!("{1} {0}", user1.username, user1.email);
```
This - `println!("{user1.username}");` - is not possible. Can't look up fields inside `{}`.

Structs have field names, so unlike tupes we don't rely on field order.

Question: are trailing commas necessary? Answer: apparently no, the code works without them. But are they useful for some reason? Like, avoiding the issue when copy-pasting new fields and forgetting to add a comma?

Questiion: can fields remain undefined, or None, or whatever it's called? Another question: what about default values?

Note: structs and tuples use dot notation for field access, and arrays and vectors use square brackets.

**To change a field, the enrite instance must be declared mutable!**
```
let mut user1 = User {
	// ...
}
user1.email = String::from("example@example.org");
```

We can construct a struct in a function. A naive example:
```
fn build_user (username: String, email:String) -> User {
    User {
        active: true,
        username: username,
        email: email,
    }
}
```

To avoid boilerplate, we can do:
```
fn build_user (username: String, email:String) -> User {
    User {
        active: true,
        username,
        email,
    }
}
```
This relies on the convention that function arguments have the same names as struct fields (`username`, `email`).

Building a struct from another struct changing some fields but not all:
```
let user2 = User {
	email: String::from("another@example.org"),
	..user1
}
```
Note that in this syntax the trailing comma is prohibited (!).

Also, `user1` becomes invalid if the struct contains some types that get moved (not copied). Like, `String`s. Only if the struct consists solely of copy-able ("primitive") types, then their values are copied, and the original variable remains valid.
```
    let username = String::from("Satoshi");
    let email = String::from("satoshi@gmx.de");
    let user1 = build_user(username, email);
    let user2 = User {
        email: String::from("another@example.org"),
        ..user1
    };
    println!("{} {} {}", user2.username, user2.email, user2.active);
```

Question: how is this useful at all? To "copy a user", I'd have to carefully `.clone()` all the fields that need cloneing?

Named tuple is a tuple that has a name (obviously) but no names for fields. Use case: define a... type (is it a correct term here) of a tuple to type-check it and not confuse with other tuples, but the fields remain unnamed. Example:
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
In other words, we list the field types right after the type name, unlike the prior `User` example where we define fields inside `{}` one by one.

Unit-like structs are structs without fields, like `()` Unit type, but may have _traits_ (discussed in Chapter 10).

Example: `struct AlwaysEqual;` - no fields in there; traits would define behavior like "always equal to any other thing" for testing.

General question: the Book often uses the word "valid" as in e.g. on page 90, gray box:
> ...we used the owned `String` type rather than the `&str` string slice type... because we want each instance of this struct to own all of its data and for that data **to be valid** for as long as the entire struct is valid.

What exactly does "valid" mean? Accessible? It could have meant "the value is still there, as in, not discarded", but I though this is what the compiler guarantees anyway.

### Example: calculate rectangle area
This is what I wrote:
```
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area is {}", area(rect1));
}

fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
```

This is kinda ok, but I gotta **use reference** instead of the Rectangle itself! Otherwise the Rectangle is unusable after the function call. Another way to think about it is that area calaulation is "read-only" and doesn't _need_ ownership of the Rectangle.

A better version:
```
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area is {}", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
```

> Accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs.

### Printing a struct
To print, we need `Display` trait that is not implemented for custom struct (obviously). On top of that, Rust can debug-print with `{:?}` inside `println!`, but for that to work, the struct must implement the trait `Debug`, which is done like this:
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

Then we can print like this:
```
println!("The area of {:?} is {}", rect1, area(&rect1));
```

And get:
```
The area of Rectangle { width: 30, height: 50 } is 1500
```

Another format option is `{:#?}` - same debug output but multi-line.

Yet another way is `dbg!` macro, which:
> ...**takes the ownership** of an expression, prints the line file and line number... along with the resultant value... and returns ownership.

Question: why is it important that `dbg!` takes ownership but `println!` doesn't? Answer: because we can continue working with the value afterwards. Example from the book: if we want to debug the Rectangle itself, we use the reference:
```
dbg!(&rect1);
```
This will simply print and continue (as one would expect from a debug print).
This - `dbg!(rect1)` - will also print, but further use of `rect1` is invalid. We could of course write with shadowing:
```
let rect1 = dbg!(rect1);
```
It works but is more verbose.

Why not use reference for println? Does println need to take ownership? [https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable](https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable) - printing macros do some magic with references, a "special case" although "every macro could implement this"

### Method syntax

Now lets turn the area function into the _method of_ a Rectangle struct (why isn't this called a class, by the way?). Instead of classes, Rust has structs + trait implementation. What are the key differences there? I guess we'll see later.

> Methods are defined in the context of a struct (or an enum or a trait object)... and their first parameter is always `self`.

To add methods, we have a **separate `impl` block**:
```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
So data fields are described within `struct`, and methods are within `impl` with the same name. Unlike classes in Python or Java.

> Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably.

What's the practical difference between taking ownership and mutable borrow here? The Book says that **taking ownership of `self` in methods is rare**, may be used if "the method transforms `self` into something else", and the caller must not use the original instance after the transformation.

TLDR: methods mostly borrow `self` either mutably or immutably.

On implicit borrowing for method: Rust does **automatic referencing and dereferencing** for methods (it can figure this out from types). Which eliminates the need for `->` operator like in C (both fields and methods are accessed via dot-notation: `rect.width` and `rect.width()`, if e.g. there is a `width()` - likely a getter).

Associated functions are functions within `impl` that don't take `&self` as a parameter (i.e., constructors). Associated functions are called with `::`, like `String::from()`:
```
impl Rectangle {
    // ...

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

Finally, there may be multiple `impl` blocks of the same name.
