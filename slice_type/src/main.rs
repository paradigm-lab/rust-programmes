fn main() {
    let s = String::from("What's up Rustaceans");
    let word = first_word(&s);
    println!("The first word in {} is {}", s, word);

    // We use the destructure pattern because we return a tuple
    let (start, end) = second_word(&s);
    println!("The second word starts at {} and ends at {}", start, end);

    // chunck of word from the String
    let chunck_word = chunck_first_word(&s);
    println!("The first word in {} is {}", s, chunck_word);
}

// Passing a String reference in the first_word f(x)
// It's because we don't want the ownership
fn first_word(s: &String) -> usize {
    // We'll convert our String to an array of bytes using as_bytes method
    let bytes = s.as_bytes();

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

fn second_word(s: &String) -> (usize, usize) {
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

fn chunck_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
