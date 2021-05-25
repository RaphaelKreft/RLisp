# RLisp

Is A Meta-Circular LISP Interpreter written in Rust. Meta-Circular means that RLisp is able to run a LISP Interpreter 
defined in RLisp. I think one can implement a LISP Interpreter in many ways in RLisp, but I tested it with an adapted
definition of Paul grahams Root of Lisp.
In Addition to that RLisp also has more functionality such as Integer arithmetics to make RLisp more powerful and
convenient to use in real-life applications.


## Usage

If you want to use precompiled executables, visit the Examples folder. The following Binaries are available: 

- Windows_amd64_64bit
- Ubuntu_amd64_64bit.


### Examples and Testing
RLisp was tested with Windows10 and Ubuntu_20.04. If you find a bug or issue please raise an issue.

**If you want to see some examples or perform tests**, visit `Examples/HowToTest.md` as a first starting point.

### Build

In case you can't find Binaries for your System you must build the Executable yourself:

1. Download and install Rust with Cargo as described here: https://www.rust-lang.org/tools/install
2. If you haven't downloaded the projects Directory, then now is the time for that.
3. Open a Commandline and change into the Project-Directory and run `cargo build` or `cargo build --release`
4. Find the executable
    1. `cargo build` Unoptimized executable with debug-information is created under target/debug
    2. `cargo build --release` Optimized executable will be created under target/release

### Run a file

You can run the Interpreter in this mode to execute a whole file. Similar to the famous python Interpreter
where you execute `python3 myfile.py` in your Shell, you can use RLisp in similar way:

1. Windows: `RLisp myfile.file` 
2. Linux: `./Rlisp myfile.file`


Please mention that when loading a file, you just see the output from the last expression!
This is because load internally uses the "do" expression to run all expressions in a sequence.
**If you want to See an output for every expression, use the "println" Command.**

### As Commandline Interpreter

When just executing Rlisp without any arguments, you start the Commandline Interpreter (similar to executing python3).

1. Windows: `Rlisp`
2. Linux: `./Rlisp `

Then you can enter your code expression for expression, line by line.

## Documentation

This part should give an overview over the data-types and language atoms, RLisp supports. The Interpreter has been built as
a mixture of other Lisp dialects: I included some elements of the Root of Lisp dialect 
(ATOM?, CAR, CDR, CONS, EQ?, QUOTE, LAMBDA) as well as elements from Scheme and MAL such as Integer Arithmetics, LET,
DEFINE, PRINTLN, DO and LOAD.

Please mention that the way each of the atoms work, can differ from what you expect, please consider the part about
"Language Atoms" for further information.

### Datatypes

**1. NIL**: 

represents "nothing" in RLisp, it is similar to null in other languages.
 
Code Representation: `#nil`

**2. Boolean**:

another essential data-type that represents truth values true and false.

Code Representation: True: `#t`, False: `#f`

**3. Integer**:

Represents Integer Numbers, is internally handled as Rust-Type i64, no Exponential notation supported.

Code Representation: `10`, `-20`, `0`

**4. Symbol**:

A symbol is a sequence of letters and numbers, Symbols can have multiple meanings and have expressions attached to them
or are dummies for arguments. 

Code Representation: `example`, `myfunction`, `bob`

**5. String**:

A String is a sequence of chars. In contrast to a symbol, a String will not be evaluated during the evaluation-process.
It can be used to print out text or give the load function a filename.

Code Representation: a String is surrounded by []- `[This is a String 123]`

**6. List**:

A List is another essential Lisp Datatype. A List consists of 0..n elements separated by commas and wrapped by 
round brackets. A List can have arbitrary elements, and an arbitrary number of elements.
Internally a List is represented as a Rust Vector of Language Elements(RLType).

Code Representation: empty list - `()`, `(1, 2, 5)`, `(1, (+ 1 2), [Raphael])`

**7. Function**:

A Function is a pure internal Datatype, that you don't see. It's the type of all operators except the special-forms.

**8. SelfDefinedFunction**:

In RLisp you can define own functions with the `lambda` operator. When you define such a function, the environment at the
point of creation, the arguments as well as the body of the function, is stored in a structure named SelfDefinedFunction. 

Code Representation: As a user you don't see this type explicitly, but it's the return type of the lambda operator:
`(define square (lambda (x) (* x x)))`

### Language Atoms

When RLisp evaluates an expression that is a list, it first evaluates the arguments and then applies a function with the arguments.
There are exceptions from this called **special forms**: When the evaluator finds a function that is a special form, the
evaluation behaviour can be different: `cond` for example doesn't evaluate all arguments, but checks conditions from left
to right and then just evaluates the first expression, that's predicate is evaluated to be true.

Atomic types are self evaluating. Example: `12` -> `12`, `#t` -> `#t`

**1. Arithmetics**:

For the Integer Datatype to be useful, we need Integer arithmetics! RLisp currently supports the following operators:

1. `+` is used to perform an addition of 2..n numbers: Example: `(+ 1 2 -3 4 5)`
2. `-` is used to subtract the sum of tailing numbers from the head. Example: `(- 10 1 2 3)` will output 4
3. `*` is used to multiply 2..n numbers. Example: `(* 2 3 -4)` or `(* 1 0)`
4. `/` is used to divide the heading number by the sum of the rest. Example: `(/ 20 2 2)` will output 5, please mention that
dividing by 0 leads to an error and that it's a full number division rounding down.

All operators currently need at least a number of 2 arguments.

**2. CAR / CDR / CONS / LIST**:

Are the standard List operators in LISP, The list representation  is not pairwise as in Root of Lisp, since this is not
needed in an implementation in Rust:

1. `list` creates a list of the arguments given to it. Example: `(list 1 'abc', (1,2), #f)` creates a list 
    `(1, [abc], (1,2), #f)`
2. `car` returns the first element of a given list, raises an error on empty lists. Example: `(car (5,6))` -> `5`
3. `cdr` returns the rest of the list, excluding the first element, raises an error on empty lists. \n
    Example: `(cdr (1,2,3))` -> `(2,3)`
4. `cons` takes exactly 2 arguments. The second one must be a list! cons then prepends the first argument to the list.
    Example: `(cons 12 ())` -> `(12, ())`
    
**3. ATOM? / LIST? / NIL? / NUMBER?**:

Are the type-checks included in RLisp. All type-checks have arity 1.

1. `atom?` inspired by the operator in Root of Lisp. Returns true if argument is a Symbol, an Integer or the empty list,
    false otherwise.
2. `list?` returns true if the argument is a list, false otherwise
3. `nil?` returns true if the argument is nil, false otherwise
4. `number?` returns true if the argument is an Integer, false otherwise

**4. EQ?**:

This operator takes 2 arguments and checks for equality. Two elements are equal if they have the same type and if the
value is the same. The arguments are evaluated before the comparison.

Example: `(eq? 5 (+ 2 3))` -> `#t`

**5. QUOTE**:

is a specialform, that is essential for the Homoiconicity principle(In Principle no difference between Code and Data). 
To be able to treat Code as data the `quote` operator suppress the evaluation of its argument. 

Example: Quoted: `(quote (+ 1 2))` -> `(+ 1 2)` vs. not quoted: `(+ 1 2)` -> `3`

The short-form of a quote also works in RLisp (quotes start with a '): `'(+ 1 2)`

**6. EVAL**:

the second specialform that is essential for Homoiconicity. It just takes one argument and evaluates it. Useful when
you have a function that returns a quoted value, and you especially want to evaluate it .

Example: `(eval (quote (+ 1 2)))` returns `3` vs. `(quote (+ 1 2))` returns `(+ 1 2)`

**7. PRINTLN**:

Is very useful when you want to print something out in a calculation or when you load a file. it takes one argument which
can be of any valid RLisp Type and prints it out on the commandline. It returns the value of the Expression it printed,
so that values can be printed out and passed to other functions for further calculation.

Example: `(println (lambda (x) (x)))` will print out "#function" or `(println (cons 1 2 3 4))` prints out "(1,2,3,4)"

**8. LAMBDA**:

is a special form that allows creating functions. It takes two lists as arguments: The first list contains the formal
parameters whereas the second one contains the body of the function(= Expression that is evaluated when the function 
is applied). When defining a function, the variables and symbols are determined by the environment at the point of creation.

1. Example: define function and bind it to a symbol: `(define identity (lambda (x) (x)))`
2. Example: inLine use: `((lambda (x, y) (+ x y)) 2 2)` will return `4`

**9. DEFINE**:

is a special form that allows creating and overwriting values in the environment. It takes two arguments, a symbolname
and a target. When the symbolname doesn't exist in the current environment, a new binding is inserted in the environment.
In following calculations with this environment, the symbolname will be mapped to the (evaluated) target.
Furthermore `define` expressions return the target value.

Example: `(define a (+ 3 3))` -> a will have value 6 from now on.

If the symbolname already exists, the target is overwritten. This makes a `set!` operation obsolete.

**10. DO**:

is a special form that executes/evaluates an arbitrary number of expressions given in sequence. It returns the 
return value of the last expression evaluated.

Example: `(do (define a 6) (define b 7) (+ a b))` will return `13`

**11. COND**:

is a special form and inspred by the Root of Lisp dialect, it takes 1..n pairs of the form `(predicate branch)`.
`cond` checks the predicates for it's truth value and evaluates the branch for the first predicate to be true.
The following pairs are ignored. If no predicate turns out to be true, then `#nil` is returned.

Example: `(cond ((eq? 3 (+ 1 2)) #t) (#t #f))` -> first predicate is true so return is `#t`

**12. LET**:

is a special form that lets you bind symbols ina specific context. It is similar to the "where" clause in Haskell.
It takes two arguments:

1. First argument is a list of symbol-target pairs.
2. The second argument is the context of the let statement. Its the body evaluated with the new environment.

`let` creates a new environment with the bindings as defined by the pairs. Outer symbolnames are shadowed by the inner ones.

**Closures:** With `let` you create a new environment. An environment has an outer environment (except the golbal environment).
When RLisp resolves a symbolname it looks for the name in the inner environment first and then explores the outer ones until
the symbol is found or certain to be undefined.

Example: `(let ((x 12) (y 5) (name [Raphael])) (do (println name) (+ x y)))`

**13. LOAD**:

is a special form that allows to evaluate whole files. It takes one String as argument, tries to open the according file
and executes the expressions in it in sequence (internally using the `do` expression). Please mention that you need to use
`println` when you want to see the results of expressions that are not the final one. The result of the final expression is
being returned.

Example: `(load [test.txt])`

### Language Elements defined in RLisp itself
With this already pretty sweet selection of atoms RLisp is Turing-Complete and we have the opportunity to expand the
language in RLisp itself. One can for example use a file to store definitions of new functions or constants 
using `define` and `lambda`. I also included a fucntion in `src/main.rs`, that takes expressions that will be executed,
every time RLisp is started. There I already added `cadr`, `caar` etc for convinience. 


## Disclaimer

The Interpreter as it is, is not perfect. There is a lot more I could implement or optimize:

- Relational Arithmetics for Integers
- All atoms and special forms can deal with empty lists and arbitrary count of arguments (proper error handling)
- optimize performance with TCO
- Add more atoms like `map` or string conversion to dynamically create strings.