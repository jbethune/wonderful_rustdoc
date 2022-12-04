use wonderful_rustdoc::greetings::greet;
use wonderful_rustdoc::beings::Human;

/// Reimplement [Erlang the Movie](https://www.youtube.com/watch?v=BXmOlCy0oBM&t=205s) in Rust: 

fn main() {
    let joe = Human{name: "Joe".into(), age: 22};
    let mike = Human{name: "Mike".into(), age: 23};
    let robert = Human{name: "Robert".into(), age: 22};

    println!("Mike: Just let us make a normal call just to see that the system works");
    println!("Joe: {}", greet(&mike));
    println!("Mike: {}", greet(&joe));
    println!("Mike: System working? Okay fine");
    println!("Joe: We are going to do this again and this time we are going at some of the symbolic information that is available in the system while we are placing a call. I'm going to dial Mike again.");
    println!("[...]");
    println!("Mike: I like to show you another property of the language. And that is how we handle errors.");
    println!("Mike: And in order to show you this, I am first going to make a perfectly normal call to Joe on this telephone.");
    println!("Mike: {}", greet(&joe));
    println!("Joe: {}", greet(&mike));
    println!("[...]");
    println!("Mike: We've now made a call to Joe. I'm going to make a call to Robert.");
    println!("Mike: {}", greet(&robert));
    println!("Robert: {}", greet(&mike));
}
