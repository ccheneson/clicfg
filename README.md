# clicfg - a simple command line tool to manage config ( or any ) files located in AWS S3

I often need to change configuration files for several environments when deploying features and 
I always find such a pain to use AWS Web Console to browse bucket/folders to download a file, modify it, bump its version and re-upload it with the proper encryption/key. 

This cli is used to manage application configuration files stored in S3.

Configs are located 
* at a path in S3 like `bucket`/`project`/`environment`/`file`
* locally at `$HOME`/conf/`project`/`environment`/`file`

`file` could be like `1.5.12.conf` or `api-back.1.2.2.conf`

This is also my first application/tool in Rust

# How to use it ?

* List in S3 the existing conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging ls`

* Locally List the existing conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging lslo`

* Get a conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging get 1.20.9.conf`

* Put a conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging put 2.0.0.conf`

* Bump a conf to a new version for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging bump 2.0.0 2.1.0`

The above command will download `2.0.0.conf`, rename it the to version `2.1.0.conf`, and re-upload it to the `api/staging`

`$> clicfg api staging diff 2.0.0.conf 2.1.0.conf`

The above command highlights the difference between the file `2.0.0.conf` and the file `2.1.0.conf`

`$> clicfg api staging edit 2.1.0.conf`

The above command downloads the file `2.1.0.conf`, opens the file with the editor sets in `PMCFG_EDITOR` environment variable (default set to `vi`). When saving and closing `vi` (`:wq!`), it pushes the file back to the correct S3 path. 

It doesn't work with GUI-based editors like `gedit`



# Features learnt along the way

* ## strum_macros
Used to have a `to_string()` method that returns the name of the enum value

```rust
use strum_macros::{Display}; 

#[derive(Display)]
pub enum Project {
    Registration,
    Authentication,
    Authorization
}

Project::Authorization.to_string() => "Authorization"
```
https://crates.io/crates/strum_macros
https://stackoverflow.com/questions/32710187/how-do-i-get-an-enum-as-a-string

Or you can implement `Display` (could be useful to display errors from `enum`)

* ## varargs like Scala

Use slice

https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments

Usage: https://github.com/ccheneson/clicfg/blob/ab90110e64ab30ea83659a765541cfb56ed6bd42/src/commands.rs#L23



* ## const or static

const: 
> Constants live for the entire lifetime of a program. More specifically, constants in Rust have no fixed address in memory.

static: 
> Rust provides a ‘global variable’ sort of facility in static items.
> You can introduce mutability with the mut keyword:


```rust 
static mut N: i32 = 5;
```

> Almost always, if you can choose between the two, choose const

https://doc.rust-lang.org/1.29.2/book/first-edition/const-and-static.html


* ## Creating a Rust function that returns a &str or String

```use std::borrow::Cow; ```

https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html

* ## Iterators
> ```.iter()``` is a method that returns each element in a collection

> ```.enumerate()``` wraps the result of `iter` and returns each element as part of a tuple instead. The first element of the tuple returned from enumerate is the **index**, and the second element is a **reference to the element**. 

* ## The `?` operator for easier error handling

> The `?` is shorthand for the entire match statements we wrote earlier. In other words, `?` applies to a `Result` value, and if it was an `Ok`, it unwraps it and gives the inner value. If it was an `Err`, it returns from the function you're currently in.

https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html

* ## Convert `Option` to `Result`

> The `ok_or` and `ok_or_else` methods convert `Options` to `Results`, and the `?` operator automates the boilerplate associated with early `Err` returns. 

https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro

* ## Get executable name

```rust
let s = std::env::current_exe()
    .expect("Can't get the exec path")
    .file_name()
    .expect("Can't get the exec name")
    .to_string_lossy()
    .into_owned();
```
https://www.programming-idioms.org/idiom/105/current-executable-name/1887/rust

Usage: https://github.com/ccheneson/clicfg/blob/ab90110e64ab30ea83659a765541cfb56ed6bd42/src/help.rs#L2-L7

* ## Errors

    * Implements `From` trait on your custom `Error`

    https://doc.rust-lang.org/std/convert/trait.From.html

    * `map_err` to map the error returned to your custom error type
    
    https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

    * Error handling with `?`
    
    https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
    
    * General info:
    
    https://rust-cli.github.io/book/tutorial/errors.html

* ### Expect

> `expect`works very similarly to `unwrap` with the addition of a custom panic message. If you’re wanting a more friendly error message from an `Err` or a `None` value, you can use `expect` to get what you’re looking for:

```rust
let status = get_status("jakedawkins").expect("status fetching");
```

> If `get_status` returned an `Err`, then the program would `panic` with an error similar to the following:

```rust
thread 'main' panicked at 'status fetching', src/main.rs:5:27
```

https://jakedawkins.com/2020-04-16-unwrap-expect-rust/

* ### List files in a directory

```rust
use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
```
The above snippet will print out the path and the file name


To only print the file name and sorted:

```rust
let mut paths = fs::read_dir("./").unwrap()
                                        .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
                                        .collect::<Result<Vec<_>, io::Error>>().unwrap();
paths.sort();
for file in paths {
    println!("{}", file)
}
            
```

https://stackoverflow.com/questions/26076005/how-can-i-list-files-of-a-directory-in-rust

https://doc.rust-lang.org/std/fs/fn.read_dir.html

Usage: https://github.com/ccheneson/clicfg/blob/ab90110e64ab30ea83659a765541cfb56ed6bd42/src/main.rs#L42

* ### `Result<T>` and `Result<T,E>`

> It's common to specialize `std::result::Result<T, E>` to ease the use with specific libraries. `std::io::Result<T>` for example is actually an alias to `std::result::Result<T, std::io::Error>`, and it's obvious that you will frequently want the `std::io::Error` for I/O tasks.

https://www.reddit.com/r/rust/comments/6b8iuy/resultt_e_vs_resultt_vs_result/


* ### Currying in Rust

You can use the `move` keyword

```rust
let foo = |x| move |y| x + y;
let bar = foo(3);
dbg!(bar(10));
```

> `move` converts any variables captured by reference or mutable reference to owned by value variables. The three `Fn` trait's mirror the ways to capture variables, when move is used, the closures is represented by the `FnOnce` trait.

https://doc.rust-lang.org/beta/std/keyword.move.html

https://internals.rust-lang.org/t/currying-in-rust/10326

* ### String

https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str


* ### Errors I came across

> value borrowed here after partial move

https://stackoverflow.com/questions/62346833/borrowing-a-structure-which-reference-strings-value-borrowed-here-after-partia

https://stackoverflow.com/questions/59215169/does-non-matching-arm-take-the-owner-of-a-variable-in-a-match-statement-in-rus

    > when you `match` something, it is moved (or copied, if possible).

* ### Testing

##### Unit Test

>  You’ll put unit tests in the `src` directory in each file with the code that they’re testing. <ins>The **convention** is to create a module named `tests` in each file to contain the test functions</ins> and to annotate the module with `cfg(test)`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

https://doc.rust-lang.org/book/ch11-03-test-organization.html

* ### Derive only for test

> You can use the cfg_attr(a, b) attribute:

```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct MyStruct;
```

Example for unit test:
```rust
//PartialEq : used for unit test
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CliConfigError {
    RusotoError(String),
    FileError(String),
    HttpClientError(TlsError),
    CliArgError(String),
}
```

https://stackoverflow.com/questions/42551113/is-it-possible-to-conditionally-enable-an-attribute-like-derive


* ### await

> Unlike a regular function, calling an `async fn` doesn't have any immediate effect. Instead, it returns a `Future`. This is a suspended computation that is waiting to be executed. To actually execute the future, use the `.await` operator:

> In contrast, in Rust, calling an `async` function does not do any scheduling in and of itself, which means that we can compose a complex nest of futures without incurring a per-future cost. As an end-user, though, the main thing you'll notice is that **futures feel lazy**: they don't do anything until you `await` them.

https://blog.rust-lang.org/2019/11/07/Async-await-stable.html


* ### Enum

Values in enum are called ` enum's variants`. 

Enum variants **are not types**.

https://stackoverflow.com/questions/51567350/can-traits-be-used-on-enum-types

* ### `map`and `and_then`

https://stackoverflow.com/questions/62745681/how-to-flatten-two-result

* ### Some links about `String`, `str`

https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html

https://blog.guillaume-gomez.fr/Rust/2/9

* ### I32 vs isize, u32 vs usize

> Use usize and isize when it’s related to memory size – the size of an object, or indexing a vector, for instance. It will be a 32-bit number on 32-bit > > platforms, as that’s the limit of memory they can address, and likewise for 64-bit.

> Use u32 and i32 when you just want numbers.

https://users.rust-lang.org/t/i32-vs-isize-u32-vs-usize/22657

* ### `into()`

https://doc.rust-lang.org/rust-by-example/conversion/from_into.html

The `Into` trait is simply the reciprocal of the `From` trait. That is, if you have implemented the `From` trait for your type, `Into` will call it when necessary.

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

```
Similar to `From` and `Into`, `TryFrom` and `TryInto` are generic traits for converting between types. Unlike `From`/`Into`, the `TryFrom`/`TryInto` traits are used for fallible conversions, and as such, return `Result`s.



* ### for and iterators

The `for in` construct is able to interact with an Iterator in several ways. As discussed in the section on the `Iterator` trait, by default the `for` loop will apply the `into_iter` function to the collection. However, this is not the only means of converting collections into iterators.

`into_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.

* `iter` - This `borrows` each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

* `into_iter` - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been `moved` within the loop.

* `iter_mut` - This mutably borrows each element of the collection, allowing for the collection to be modified in place.




* ### pointers/ref


For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts which are used differently from a language like C.

* Dereferencing uses `*`
    
    ```rust
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    ```
    
* Destructuring uses `&`, `ref`, and `ref mut`
```rust
    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
```

https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html


* ### Closures - As Input parameters


*    `Fn`: the closure captures by reference (`&T`)
*    `FnMut`: the closure captures by mutable reference (`&mut T`)
*    `FnOnce`: the closure captures by value (`T`)


https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html



* ### Closures - As Output parameters

```rust

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
```
https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html



