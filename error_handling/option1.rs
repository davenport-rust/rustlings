// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)

fn main() {
    let mut list = vec![3];

    print_opt("last item".to_string(), list.pop());
    print_opt("second to last item".to_string(), list.pop());
}


fn print_opt(item_name: String, opt: Option<i32>) -> () {
    match opt {
        Some(v) => println!("The {:?} in the list is {:?}", item_name, v ),
        None => (),
    }
}

























// Try using a `match` statement where the arms are `Some(thing)` and `None`.
// Or set a default value to print out if you get `None` by using the
// function `unwrap_or`.
// Or use an `if let` statement on the result of `pop()` to both destructure
// a `Some` value and only print out something if we have a value!
