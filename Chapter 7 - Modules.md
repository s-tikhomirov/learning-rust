Packages, crates, and modules.

> ... you should organize code by splitting it into multiple **modules** and _then_ multiple **files**. A **package** can contain multiple **binary crates** and optionally one **library crate**. As a package grows, you can extract parts into separate crates that become external **dependencies**.

That's a lot to unpack. So we have the following notions here:
- module
- file
- package
- crate
	- binary crate
	- library crate

File contains modules. Package contains crates. What's in between? Let's read on. There are also _workspaces_ for very large projects.

Yet another dimension: public interface vs private implementation:
> ...other call can call your code via its public interface...

Question: what is the definition of "other code"? Is it code from another... module, file, package, crate?

> a related concept is **scope**

Understand exactly how it relates. I can ... import? a.. crate? and then its variables enter the scope of my code, right?

> Rust has a number of features...
> - packages
> - crates
> - modules and use
> - paths

Paths is something new here! 

### Crates
> A crate is **the smallest amount of code that the Rust compiler considers at a time**. 

Hm, what is "considers at a time"? I thought the smallest amount is a single expression or statement (or lexem even - depending on that "consider at a time" means).

> Crates **can** **contain modules**, and modules may be defined in other files that get compiled with the crate...

OK, so the smallest compilation... unit (?) is not a file, it's a crate, which may be spread across files. A module, in turn, is "sub-atomic" in the sense that you can't compile a module, only a crate that it belongs to. Does every module belong to a crate? To exactly one crate? Must a crate contain at least one module (or is it automatically always the case)?

Crates can be:
- **binary crates** - compiles to an executable that can be run; must contain `main` function (any requirements on its signature?);
- **library crates** - well, a library.

> when Rustaceans say "crate", they mean library crate, ... interchangeably with the general programming concept of a "library"

> The **crate root** is a source **file** that the compiler starts from... makes up the root module of your crate

(unclear exactly what it means but the general idea is that this is something like a `main`, or an entry point - libraries don't have `main`, but there still is the place where the public interface is defined)

> A **package** is a bundle of **one or more crates**... contains a `Cargo.toml` file that describes how to build those crates.

> A package can contain **as many binary crates as you like** but **at most one library crate**

Why? [SO](https://stackoverflow.com/questions/54843118/why-can-a-cargo-package-only-have-one-library-target):
>When we use a crate as a dependency, we only specify the package name in our `Cargo.toml`. Since there can be at most one library, Cargo doesn't need you to specify which one to use. If it were allowed to define multiple libraries in the same package, then we'd need to specify a way to define dependencies between them...
>...there is no reason to limit the other types of targets (binaries, examples, tests, etc.) to one each.

So the use case seems to be the following. I want to develop my library for others to use. In Cargo terms, that would be a package. Users would import it by adding the **package** name in `Cargo.toml`, as I did in [[Chapter 2 - Guessing Game]] in the guessing game example. So I create a **library crate** that defines my core functionality. It may be implemented in one file / in one module, or some modules (?) may be in separate files, - the important thing is that it all compiles to one library binary(?). As I'm developing the library, I'd need auxiliary stuff like tests and examples. These are not library crates, but rather binary crates, meaning, they can be executed on their own (and they likely import my library crate of course).

Is it true that: `Cargo.toml` <--> package?

There is no mention of `src/main.rs` in `Cargo.toml` because
> Cargo follows a convention that `src/main.rs` is the **crate root of a binary crate with the same name as the package**.

(Similar convention for `src/lib.rs` for a library crate with the same name as the package - there can be at most one per package!)

One can declare **submodules** within a module with keyword `mod`. Where the compiler looks for the code of a (sub-)module:
- inline: `mod garden { //... };`
- `src/garden.rs`
- `src/garden/mod.rs`

Paths to code in modules: `crate::garden::vegetables::Asparagus`. Question: what is `crate` here, is it the name of my crate?

**Code within a module is private from its parent modules by default.** To make it public: `pub mod` _and_ `pub` before declaration of items within. Question: why does it make sense to declare module as `pub` but leave all items (i.e., functions, structs etc?) within it public? Probably it makes no sense. A sane use case could be: _some_ of module functions are public, so I declare them _and_ the module public to indicate that _some_ stuff within is public. Some functions however remain private (by default) - implementation details or whatnot.

As a test, I tried to separate some of my code to a module, with the following structure:
```
src
	main.rs
	channels
		direction.rs
```

Inside `direction.rs`, I define a module:
```
mod direction {
	struct Direction {
		//...
	}
	impl Direction {
		// ...
	}
}
```
How to I use it in `main.rs`? `use crate::channels::direction;` doesn't work. The issue is that I don't really understand what I'm trying to do.

I understand how my code can be logically separated into kinda independent chunks that do as follows:
- the structure of a channel, channel directions, fee structure
- the structure of the whole network: parse JSON or some inline graph description, assign channels to edges, find routes
- the schedule: a priority queue with scheduled payments
- the structure of a payment: a nested structure specifying fees on every hop so that each node on the route gets paid enough
- the "main" logic: defining the high-level properties of an experiment, running it, and collecting the results (may also be separated into sub-chunks, like, for exporting the results into some JSON file, drawing graphs, etc.).

What I don't understand is what level these "chunks" should be separated at? Should these be:
- sub-modules inside on module?
- modules inside one crate?
- crates inside one package?
- multiple packages?

I tried to go "module inside one crate" route but it's non-intuitive how to import (see above), reading on. The example in the gray box on page 122 with `use crate::garden::vegetables::Asparagus` didn't translate to my use case, I wonder why. Defining it `pub` didn't help. Reading on.

### Defining modules to control scope and privacy
> Modules let us organize code... control the privacy... items are private by default... not available for **outside use**.

By "outside", I gather, they mean "outside of the module they are defined in". And "items" are probably functions, structs, enums, etc. So far so good.

Example: a **library crate** implementing a restaurant. Front of house and back of house.

> `src/main.rs` and `src/lib.rs` are called **crate roots**. ... the contents of **either** of these two files **form a module** named `crate` **at the root of the crate's ... module tree**.

In other words. The module tree is defined by our own use of `mod` keyword, except for that the outer-most module is created automatically and is called `crate`. What goes into the `crate` module? - The contents of the root file, which is (must be?) called `main.rs` for binary crates and `lib.rs` for library crates.

Question: should my project be a library? I kinda can see where it can be separated into "let's define the way we model the network" and "using this model, let's run some simulations on the network model". The former part looks like a library, and one could imagine other projects using it. If so, how would I structure my code? Should it be one package with two crates inside: the library crate defining the model, and the binary crate using it?

Question: what is / should be the relationship between modules and files? On the one hand, it's convenient when the file structure reflects the semantic structure of the code (like in Java the class name must equal the filename?). On the other hand, at least in my case, modules are... "smaller" than files? Creating a file per module (or even per highest-level module) can look clunky.

### Paths
Paths (in the module tree) can be absolute or relative:
- an absolute path starts with `crate::` for code from internal modules, and from the crate name for external code.
- a relative path starts from the current module "and uses `self`, `super`, or an **identifier in the current module**".

> Both absolute and relative paths are **followed by** one or more identifiers separated by double colons (`::`).

What does it mean "followed by"? Are not identifiers part of path? AFAICT, the (full) path is something like `crate::module::submodule::subsubmodule::function`. What follows what in this chain of things? `function` follows all that precede it? For me this whole thing is a path, am I wrong?

And how would _multiple_ identifiers "follow" the path, if I want to use `function1` and `function2` from the same module?

> Our preference in general is **to specify absolute paths**

> All items are private **to parent modules** by default.

> Items in child modules **can use the items in their anscestor modules**.

Note: modules defined on the same level of the module tree (**siblings**) are visible to each other (but not their inner items, unless they are public):
```
mod A {
	pub mod B {}
	mod C {}
	fn bar() {}
}

fn foo () {
	A::B::// works
	A::C::// doesn't work: C is private
}
```

The gray box on  page 129 gives this piece of advice:
> The module tree should be defined in `src/lib.rs`

The same idea I mentioned earlier: write a library that models stuff, and then use it for concrete simulations.

Question: how is the file system structure related to the module structure? I want to structure not only module-wise, but file-wise. Can I _define_ the module tree in `src/lib.rs`, but _implement_ the bulk of it in other files? If so, how close should the file hierarchy be to the module hierarchy? I want to avoid two things here:
1. a complex module structure defined in one file - too much code to consider at once;
2. a module structure spread over a _similar but somewhat different_ file structure - this is also hard to reason about.

On the other hand, maintaining the exact match between modules and files requires manual effort (or does it)? Is it worth it?

On the `super::` thing: so from inside a module, we can access:
- all items in this module
- all (public and private!) items in the parent module, to which we get access with `super::`
- only public items in the sibling modules.

Structs and enums are private; to make a field public, **both the field and the struct** must be marked public.

Insight: this private / public discussion is **about types, not objects**. For example, to access a field in a struct defined in another module, **all** of the following must hold:
- the **struct** (~ the data type) is declared public;
- the **field** we want to access is declared public.
If we want not only to access, but to **modify** the field, the corresponding **instance of a struct** must be marked mutable.

In short: public / private relates to types of objects, while mutable / immutable relates to concrete objects.

Enums are not the same:
> if we make an enum public, all of its variants are then public.

On the `use` keyword:
```
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Why do we repeat `hosting::` both in `use` and when calling the function? Looks a bit repetitive. Possible answer: we bring the `hosting` **module** into the scope, not its inner items. We can _then_ refer to the items (like, functions, structs, etc) specifying their module and name. But still. Is it for stricter import control? Probably. Like in Python I could write `from module import *` but this if discouraged, better `import module` or `from module import foo`.

### Idiomatic use paths
This section answers my question above. When do we `use` the module, and when a concrete function from the module?

> Listing ... is the idiomatic way

referring to this style, i.e., not going all the way to the function name in `use`:
```
use crate::module;
//...
module::foo();
```

On the other hand, for **structs, enums, and other items** (not functions) it's **idiomatic to specify the full path**.

The exception is two items with the same name (`fmt::Result` and `io::Result`). Although re-naming with `as` is possible:
```
use std::io::Result as IoResult;
```

Re-exporting: if we don't use `pub` here, the imported item won't be available to code that `use`s our code:
```
pub use crate::front_of_house::hosting;
```
Although the caller's code could have imported the same thing independently from its original location... Seems not very relevant for me now.

Returning to the example from [[Chapter 2 - Guessing Game]], where we did:
```
use rand::Rng;

let secret_number = rand::thread_rng().get_range(1..100);
```

Here we use the `rand` **crate**, which was downloaded from crates.io. In particular, from that crate, we use the `Rng` **trait**, which is part of the `rand` crate. We then call a function `thread_rng()` that is part of that trait. (Is this correct?)

Question: why do we write  `rand::thread_rng()` and not `Rng::thread_rng()`? Or even  `rand::Rng::thread_rng()`? OK, `rand` is a crate here. What is a module? Is it also named `crate`? Or `rand`? The hierarchy needs to be:
1. crate: `rand`
2. module: ???
3. item: `Rng` (a trait)
4. function we are calling: `thread_rng`

Unclear: what the module is; part of what (trait?) is the function?

Nested paths:
```
use std::{cmp::Ordering, io};
```
Or include the top-most ancestor as well:
```
use std::io::{self, Write};
```

The glob operator (careful with this one):
```
use std::collections::*;
```

### Separating modules into different files (finally!)

In the restaurant example, in the end, we have this file structure:
```
src
	front_of_house
		hosting.rs
	front_of_house.rs
	lib.rs
```

So the file structure **must** correspond to the module structure (I'm not sure myself what correspond means exactly, but it should be pretty close anyway). In particular:
- in the root file (`lib.rs` or `main.rs`) we `mod front_of_house;`
- in `front_of_house.rs` we define the module `front_of_house` and also do `pub mod hosting;`
- in `front_of_house/hosting.rs` we define the module `hosting`, which is a sub-module of `front_of_house`.

Note: we write `pub mod hosting` **in the parent module** (`front_of_house.rs`). In the file `hosting.rs`, we simply define the function: `pub fn add_to_waitlist() {}`.

Think about the difference between `mod my_module;` (this defines a module and **tells the compiler to look for its code somewhere else**, namely, in the file named `my_module.rs`) and `use cargo::my_module` - we can do it only when the module is defined. So, assuming each module is defined in its own file, we repeat `mod my_module` twice for every module; in the root file (??) and in the file where the module is actually defined. On the other hand, we may repeat `use` as many times as we use this module, also from other crates / packages. Finally: do we have to `use cargo::my_module` in `lib.rs` when we also have `mod my_module`? Not necessarily, this works:
```
mod front_of_house;
//pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
```
Note that if we uncomment the `use` line, we can refer to the function shorter:
```
mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Wait a sec, what's the difference between `mod` and `use`? Apparently that's where many novices stumble :) 

https://panaetius.io/post/2020/11/the-difference-between-mod-and-use-in-rust/
> When you use mod in this way and compile, Cargo brings the contents of utilities.rs and inserts it into the current file.

(this way referring to `pub mod utilities;`)

> using `use` simply brings an item into the current namespace so you can access it more easily. Whereas `mod` (without a body block {}) literally brings the contents of a file and inserts in its place.

OK, so the question is: does the compiler _know_ where to look for the code of a module we want to use...

Made the first attempt at establishing the module structure, still have no intuition what I'm doing and why (see [this commit](https://github.com/s-tikhomirov/ln-jamming-simulator-rust/commit/2df2114)). One of the questions: it feels like in the root there should be `main.rs`, `lib.rs`, and everything else in its own dedicated directory. It looks weird where the two root files are mixed with regular source code files. Indeed, [this example](https://doc.rust-lang.org/cargo/guide/project-layout.html) of a typical project layout shows that all source files apart from the two root ones are in the `bin` directory (which is confusing: for me `bin` means binary, i.e., what the compiler produces; and here we have Rust source files in `bin` referred to as "executables"; maybe I misunderstand something?). Anyway, this structure looks logical, disregarding the `bin` name, but I'm not sure how to initialize modules in `lib.rs` then.






































































