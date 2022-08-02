fn main() {
    let s = String::from("What's up Rustaceans");

    // s.clear();

    let word = first_word(&s[..]);
    println!("The first word in {} is {}", s, word);

    // We use the destructure pattern because we return a tuple
    let (start, end) = second_word(&s[..]);
    println!("The second word starts at {} and ends at {}", start, end);

    // chunck of word from the String
    let chunck_word = chunck_first_word(&s[..]);
    println!("The first word in {} is {}", s, chunck_word);

    // chunck of second word from the String
    let word_two = chunck_second_word(&s[..]);
    println!("The second word s {word_two}");
}

// Passing a String reference in the first_word f(x)
// It's because we don't want the ownership
fn first_word(s: &str) -> usize {
    // usize is the type used for the size of an array or String or Index
    // We'll convert our String to an array of bytes using as_bytes method
    let bytes = s.as_bytes(); // Returns a byte slice of this String's content(The inverse of this method is from_utf8)

    // Created an iterator over the array of bytes using the iter method
    // iter method returns each element in a collection and
    // Enumerate wraps the result of Iter and
    // Return each element as a tuple instead.

    // The first element of the tuple returned from enumerate is the index,
    // The second element is a reference to the element

    // Because enumerate() will return a tuple
    // We then use a pattern to destructure that tuple.
    for (i, &item) in bytes.iter().enumerate() {
        // We search for the byte that represents the space by using the byte literal syntax.
        // if we find a space we return the position.
        if item == b' ' {
            return i;
        }
    }
    // Otherwise we return the length of the String
    s.len()
}

fn second_word(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut first_index = 0;
    let mut found_first: bool = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first {
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return (first_index, (i - 1));
        }
    }

    (s.len(), s.len())
}

fn chunck_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn chunck_second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index = 0;
    let mut found_first: bool = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first {
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return &s[first_index..i];
        }
    }

    // Adding the optimization
    if found_first {
        &s[first_index..s.len()]
    } else {
        ""
    }
}

/*
    Notes:
        String slices is a reference to part of a String.
        The slice data structure stores the starting position and the length of the slice,
            Which corresponds to ending_index minux starting_index

        let s = String::from("hello world");
        let world = &s[6..11];
        "world" would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5

        If index starts at 0 we can use [..2] (Short hand notation) likely with the end [5..]
        The entire string is [..]

        String slice range indices must occur at valid UTF-8 character boundaries.

        Type that signifies "String slice" is written as (&str) String type slice
        &str is a type that represent chank or a String or the entire string

        String literal are slices.
        Pointing to specific point of the binary. String literals are immutable; 
        &str is a immutable reference

        Other Slices
        String slices, you might imagine, are specific to strings. But there's a more general slice type, too.
        Just as we might want to refer to a part of a string, We might want to refer to part of an array. 
        eg: let a = [1, 2, 3, 4, 5];
            let slice = &a[1..3];

*/
