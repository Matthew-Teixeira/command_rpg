// Lifetimes are another kind of generic that we’ve already been using. 
// Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.


// Dangling References Example
fn test() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// Lifetime Annotations in Function Definitions
/* 
Lifetime annotations don’t change how long any of the references live. Rather, 
they describe the relationships of the lifetimes of multiple references to each 
other without affecting the lifetimes. Just as functions can accept any type 
when the signature specifies a generic type parameter, functions can accept references 
with any lifetime by specifying a generic lifetime parameter.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters 
must start with an apostrophe (') and are usually all lowercase and very short, like generic types. 

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

/* 
The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, 
both of which are string slices that live at least as long as lifetime 'a. The function signature 
also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. 
In practice, it means that the lifetime of the reference returned by the longest function is the same as 
the smaller of the lifetimes of the values referred to by the function arguments. These relationships are 
what we want Rust to use when analyzing this code.

In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the 
lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, 
the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
*/

fn pseudo_main() {
    let my_str = "Hello";
    let another_str = "Hello Again";

    let longest = longest_str(&my_str, &another_str);

    println!("Longest str is: {}", longest);
}

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime Annotations in Struct Definitions