fn main() {
    // Variable validation

    // _string_literal refert to a string literal
    // _string_literal is not valid here, it's not yet declared
    let _string_literal = "String Literal"; // _string_literal is valid from this point forward (Validity)
                                            // Do stuff with _string_literals

    /*
        Why can String be mutated but literals cannot? The difference is how these two types deal with memory.
        String literal is stored on the stack while,
        String type is store on the heap
        With String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap,
        unknown at compile time, to hole the contents. This means:
            * The memory must be requested from the memory allocator at runtime.
            * We need a way of returning this memory to the allocator when we're done with our String

        When it goes out of scope rust call a special f(x) drop automatically at the closing curly bracket.

        In C++, This pattern of deallocating is Resource Acquisition Is Initialization (RAII)
    */

    let mut s = String::from("Hello"); // :: Operator that allows us to namespace this particular from f(x)
    s.push_str(", Rustaceans"); // Appends a literal to a String
    println!("Our String is: {s}");

    /*
        Integers are simple values and stored
        in the stack so in this case y is the copy of 5.
    */
    let x = 5;
    let y = x;
    println!("{y}");
    // Immutable String
    /*
        A pointer holds the contents of the String,
        A length -> How much memory (In bytes)
        A capacity -> Total amount of memory (In bytes)
    */
    // We are not copying hello but
    // We are copying the data from the stack

    // This means s1 is no longer existing
    // This is an example of a "move"
    // let s1 = String::from("Hello");
    // let s2 = s1;

    // This is a deep copy and it mantain the value
    // This is an example of a "Clone"
    let c1 = String::from("Hello");
    let c2 = c1.clone();

    println!("c1: {c1}, c2: {c2}");

    let tup: (u32, bool) = (117, true);
    let tup_clone = tup;

    let (tup_x, _) = tup;
    let (tup_clone_x, _) = tup_clone;

    println!("tup_x = {tup_x}, tup_clone = {tup_clone_x}")
} // This scope is now over, and s is no longer valid
