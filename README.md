# music stuff

## Building
With Rust installed, `git clone` this repository; then `cargo r` to build and run the REPL.

## Language documentation

### Using the REPL
Use `(load "module_path")` to load a file; use `(quit)` to terminate the REPL.

### Primitive types
There are several primitive types:
| Type | Description | Examples |
| ---- | ----------- | -------- |
| `Number` | a 32-bit floating point number | `10.4` |
| `Boolean` | a boolean value | `TRUE`, `FALSE` |
| `Symbol` | an identifier for a variable or function | `add`, `find_least_element` |
| `String` | a string of characters | `"hello world"` |
| `List` | a list of objects | `(1 2 3)`, `("hello" 17.8 FALSE)` |

### Functions
Everything that isn't a primitive value is a function. Functions are defined with the following syntax:
```
(def <function_name> (<arg_1> <arg_2> <...>) <eval_expression>)
```
and are called with the following syntax:
```
(<function_name> <arg_1> <arg_2> <...>)
```

For example, a function that computes `(a * b) + (a / b)` would be defined as follows:
```
(def add_product_and_quotient (a b)
    (+ (* a b) (/ a b)))
```
Functions with no arguments are effectively just variables, and the argument list can be omitted entirely in their definition:
```
(def three (+ 1 2))
```
The following functions are built into the language:
| Function | Description | Examples |
| -------- | ----------- | -------- |
| `+` | Adds all arguments | `(+ 1 2 3 4)` evaluates to `10` |
| `-` | Subtracts all other arguments from the first argument | `(- 1 2 3 4)` evaluates to `-8` |
| `*` | Multiplies all arguments | `(* 1 2 3 4)` evaluates to `24` |
| `/` | Divides the first argument by all other arguments | `(/ 144 6 3)` evaluates to `8` |
| `sin` | Computes the sine of the given radian value | `(sin 3)` evaluates to 0.14112 |
| `cos` | Computes the cosine of the given radian value | `(cos 3)` evaluates to -0.9899925 |
| `sin` | Computes the tangent of the given radian value | `(tan 3)` evaluates to -0.14254655 |
| `equals` | Checks if two values are equal | `(equals 3 2)` evaluates to `FALSE` |
| `if` | If the first argument is `TRUE`, evaluates to the second argument; otherwise, evaluates to the third argument | `(if (equals 3 (+ 1 2)) "equal!" "not equal!")` evaluates to `"equal!"` |
| `def` | As described above | |
