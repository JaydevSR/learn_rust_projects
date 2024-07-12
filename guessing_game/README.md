A number guessing game from the Rust book.

## Notes

### VARIABLES

* Rust variables are immutable by default.
* `let` statement creates a new variable.
* `let mut` statement is used to define mutable variables.
* `=` creates a binding for the variable defined using `let`.

* Example: `let mut apples = 5;`


### STRINGS
* A string can be defined using "..."
* A new string instance is bound to a variable using the 'associated function' `String::new()`.
* `String::new()` creates a new empty string instance.


### STANDARD IO
* The method `read_line` of `std::io::Stdin()` is used for input.
* This method appends whatever the user types in to the string argument.
* In addition this method also returns a `Result` type.
* `Result` type has values `Ok` or `Err` telling the status of the method.
* `expect` method is defined for `Result` type for displaying a message if value is `Err`.

* `&` passes the variable by reference, without copying the contents anywhere.
