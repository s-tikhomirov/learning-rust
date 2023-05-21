`println!` is a macro, not a function (unclear what it means exactly)





- think about the relationship between: ownership; references; stack and heap. Important idea I don't internalize yet: references are a way to use a value without transferring ownership.
-   We need to distinguish between two "pointer-like" aspects:
    -   1. Values of complex types are on the heap; variables themselves _point_ to them; assigning transfers ownership (so does passing into a function)
    -   2. We can use _references_ to variables.
-   Questions: can there be references to primitive types??
-   On mutability
    -   variables are immutable by default
    -   declare with mut => can modify in the same scope
    -   can modify in a different scope if you transfer ownership
    -   can pass by reference, but references don't transfer ownership (borrowing), hence can't modify
    -   you can use a mutable reference &mut to modify without giving ownership, but the variable ALSO needs to be mutable!
    -   Question: what's the point of "no ownership" for a mutable reference if the value can be modified anyway??
        -   In other words: what's the difference between passing ownership and passing a mutable reference? [https://stackoverflow.com/questions/39534712/should-i-pass-a-mutable-reference-or-transfer-ownership-of-a-variable-in-the-con](https://stackoverflow.com/questions/39534712/should-i-pass-a-mutable-reference-or-transfer-ownership-of-a-variable-in-the-con)
            -   "One key way to think about ownership is: who is **responsible for destroying** the value when you are done with it."
        -   " Mutability is only a property of the binding:"
-   Move = shallow copy + invalidate the previous reference
-   On various types of "references"
    -   primitive types are simply on the stack and get copied around
    -   "complex" types (String) are on the heap; variable is a "fat pointer"
    -   variable gets _moved_ when assigned
    -   also there are _references_
        -   mutable references
        -   immutable references
        -   (yet unclear what's the difference between mutating a variable and assigning a new one)
    -   borrow = reference without transferring ownership
-   Copy trait: is is only _possible_ for stack-only types? "Because a String owns a pointer to data on the heap, a shallow-copy would cause multiple strings to have ownership over the same heap data." - seems that assignment is by definition a shallow copy. If we want a deep copy, we must use clone().
-   On ownership
-   On references
-   TLDR: each value has:
-   Slice: a reference to a _contigious_ sequence in a collection. Unclear: if a slice is at the "same level of indirection" than a variable, than.... what is wrong? keyword: DEREF COERCIONS
-   Apparently, we can pass a &String to a function that expects &str, although a slice and a reference-to-a-string are on different "levels of indirection"

