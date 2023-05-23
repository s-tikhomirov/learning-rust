Note and questions while (re-)implementing my LN jamming simulator in Rust (originally written in Python).

Comparison of custom `enum`s: is `#[derive(PartialEq)]` enough? What exactly does it do? When do I have to implement my own comparison operators and how do I do it?

How to write tests?

How to structure a project, how to import files, namespaces something something.

Apply constraints to scalar types? https://www.reddit.com/r/rust/comments/swiqw7/whats_the_right_way_to_apply_constraints_to/

Related: constraint the range of values of a primitive type (that's not so obvious from the first glance - no dependent types in Rust?): https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
(I wanted to use it for a probability type that would only accept values from 0 to 1; I could also import it from some probability crate, but I don't yet know how to do that either. Leaving as is, this probability stuff isn't used much in my code anyway.)