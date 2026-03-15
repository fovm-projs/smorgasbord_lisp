# Pre-standard Smorgasbord specification

This file includes the most up-to-date specification of Smorgasbord Lisp.

It contains the following chapters:
- syntax
- unique variables
- metaprogramming
- type system
- built-in functions

## Syntax

The syntax is... Plain S-expressions.

### Calling a function

```
(function_name arg1 arg2 argn)
```

### Defining constants

```
(defconstant (type name) value)
```

### Defining unique variables

```
(defunique (type name) value)
```

### Defining functions

```
(defun purity (type name) ((type arg1) (type arg2)) (f1 a1 a2 (f2 b1)))
```

Note that unlike others, the ranged number type uses parentheses:

```
(defun pure (number a))
(defun pure (string a))

(defun pure ((ranged 0 8) a))
```

(Note that these functions are invalid: arguments & body should be explicitly written; it's an example for types)

Here are some examples:

```
(defun pure (number factorial) ((number n))
  (if (= n 0)
    1
    (* n (factorial (- n 1)))))
```

No arguments can be provided at all. This function just returns "hey":

```
(defun pure (string greeting) () "hey")
```

## Unique variables

This is a special variable type, inherited from the Clean & Rust programming languages.

This type allows:
- in-place mutations
- deterministic freeing with no garbage collection
- safe use in parallel computations

Constants are a very important and useful concept in pure functional programming, because they allow safe data manipulation: no side effects can affect data, nothing in parallel computations can go wrong, etc. That's why Smorgasbord Lisp uses them everywhere it can.

The cost, however, is pretty high. Long story short: we write programs to communicate with a changing world. That's why we need to represent mutability in one way or another. And it had better be safe.

Unique type is the way to do that.

#### How does it help?

Mutable languages allow you to shoot yourself in the foot, because they don't have any guarantees that a mutable variable is safe. The main problem is **aliasing**: a situation where a mutable variable has more than one reference to it at a time.

```
// Lua-like pseudocode

local age = 10

function grow_older()
    age = age + 1
end

// Consider we call "grow_older" somewhere here
// ...

// You're expecting it to be true,
// but this function returns false,
// because you didn't notice the "grow_older" call somewhere
function are_we_ten()
    return age == 10
end
```

Both functions ```grow_older``` and ```are_we_ten``` use the ```age``` variable. And it may cause unpredictable results and therefore unpredictable behavior.

Unique type solves this with a guarantee that aliasing is semantically impossible:

```
(defun pure (number loop_exm) () (
    (defunique ((ranged 0 10) counter) 0)
    (while (< counter 10) (set counter (+ counter 1)))
    counter))
```

You can safely define it with the ```defunique``` instruction & change it with ```set```.

#### Global unique variable?

No, it isn't allowed. Unique variables are specifically reserved for local use, so the only way you can use them is inside functions.

The closest you can get to a global variable is passing it around between functions.

Note that in the case of recursion (e.g. circular recursion) this rule doesn't change:

```
(defun pure (void just-define) () 
    (defunique (number x) 0)
    (first-func (move x)))

(defun pure (void first-func) ((unique number num))
    (set num (+ num 1))
    (second-func (move num)))

(defun pure (void second-func) ((unique number num))
    (set num (+ num 2))
    (first-func (move num)))
```

The ```num``` variable is just moved around.

This is an eternal recursion, though. Don't use it in production.

#### How do unique variables influence the purity of a function?

Unique variables inside a function don't change its purity.

They were created for local usage. Unless effectful and/or nondeterministic functions are called in the process of initialization, a function can be pure.

Considering that global unique variables don't exist, they can be fairly predictable.

#### Copying and moving

So you have a special type modifier — ```unique```. This type is specifically created to make unique variable management explicit.

Consider these 2 functions:

```
(defun pure ((ranged 0 16) x) (((ranged 0 8) first) ((ranged 0 8) second)) (+ first second))
(defun pure ((ranged 0 16) y) ((unique (ranged 0 8) first) ((ranged 0 8) second)) (+ first second))

(defun pure (void main) () 
    (defunique ((ranged 0 8) value) 4)
    (defconstant ((ranged 0 16) first-sum) (x (copy value) 2))
    (defconstant ((ranged 0 16) second-sum) (y (move value) 2)))
```

In this example, ```value``` is a unique variable.
- it's forced to be copied if the argument's type is not ```unique```
- it's forced to be moved if the argument's type is ```unique```

```copy``` passes a unique variable by value. After this, the unique variable is dropped in the same scope and the next function treats it as a constant.

```move``` passes a unique variable by reference (transfers ownership of the unique variable to the next function). After this, the unique variable is dropped in the next function (which treats it as a unique variable).

Both ```copy``` and ```move``` semantics are special to unique variables. Constants are always passed by value. Using ```copy``` or ```move``` on constants will cause a compilation error, as will not using them for unique variables.

#### What happens after moving?

The variable is automatically freed if it isn't moved further. The scope ends — the lifetime does too.

## Metaprogramming

[TODO] 22.03.2026

## Type system

The type system is a static, explicit type system.

These types are provided:
- number
- ranged number
- string
- a list with a fixed/dynamic size
- void
- unique

### Number

This is a numeric type. Theoretically it can be (-inf, inf), while the backends will provide different implementations for it.

This type includes unsigned/signed integers & floats.

Without optimizations, backends will put this data on the heap for dynamic-size behavior. However, numeric range analysis can provide additional information, which can be used by the backends for optimizations.

Use this type if you can't tell with certainty the maximum possible input (or the minimum one). For example, it can be used in IO (you can't tell with certainty which number a user will provide), etc.

#### Different types of a number

These types are the inner compiler representation. It also works with ranged numbers.

[TODO] 22.03.2026

### Ranged number

This is also a numeric type. It's strictly bounded by user-defined bounds.

For example: `(defun ((ranged 0 14) sum) (((ranged 0 7) a) ((ranged 0 7) b)) (+ a b))`

This example will guarantee that the function called `sum` will return a number from 0 to 14 (which will be a sum of 2 numbers from 0 to 7).

This type has backend optimization guarantees:
- ranged from 0 to inf — represented as an unsigned integer
- ranged from -inf to inf — represented as an integer
- ranged from -inf to inf (with a floating point) — represented as a float

Generally speaking, most targets can provide 64-bit numbers. So this means that you can expect memory optimizations for up-to-64-bit ranges, because backends will put them on the stack. Otherwise they will generally be put on the heap.

Use this type if you can tell with certainty the maximum possible input (and the minimum one). It's advisable to use it basically everywhere you can, but even with no specified boundaries the compiler will try to determine them, and only if it's not possible will it give up.

### String

This type is a representation of a subsequence of bytes. It typically represents UTF-8 encoded bytes.

`(write 0 "Hello, World!")`

### Fixed/Dynamic lists

[TODO] 22.03.2026

### Void

This is a special function type, which signifies that a function returns nothing.

It is not possible to assign this type to a constant, a unique variable, or an argument.

### Unique

This is a special argument modifier, which signifies that the unique variable must be moved.

It is not possible to assign this modifier to a function, a constant, or a unique variable.

## Built-in functions

The functions available in Smorgasbord strictly depend on the selected backend. In contrast, syntax, type system, etc. are available for any backend.

And the backends... They are very different: from vanilla JS for browsers to MISRA-C with CompCert.

Below is a table with separate articles explaining the functionality of each backend in detail:
- server JS
- vanilla JS
- C11
- C99
- MISRA-C
- JVM
- virtual machine

### Functions purity

In computer programming, a pure function is a function that has the following properties:

1. the function return values are identical for identical arguments (no variation with local static variables, non-local variables, mutable reference arguments or input streams, i.e., referential transparency) (determinism/nondeterminism)
2. the function has no side effects (no mutation of non-local variables, mutable reference arguments or input/output streams)

Smorgasbord Lisp provides function purity annotations, consisting of 4 types:
- pure function — it has no side effects & is deterministic
- effected function — it has side effects, yet is deterministic
- nondeterministic function — it has no side effects, but is nondeterministic
- impure function — it has side effects and is nondeterministic

The compiler guarantees that a certain function will have no authority to call some of the other functions:

| Can/Can't call?  | Pure    | Effected | Nondeterministic | Impure  |
|------------------|---------|----------|------------------|---------|
| Pure             | **Can** | Can't    | Can't            | Can't   |
| Effected         | **Can** | **Can**  | Can't            | Can't   |
| Nondeterministic | **Can** | Can't    | **Can**          | Can't   |
| Impure           | **Can** | **Can**  | **Can**          | **Can** |

- pure functions can be called by any kind, but they can't call any functions except pure ones
- effected & nondeterministic functions can call pure & their own type functions, but can't call impure functions
- impure functions can use any kind of function, but can only be called by other impure functions

#### But... Why?

Let's simplify this academic lexicon with a simpler explanation:
- side effects within a function change something outside of the function
- nondeterminism means that a function doesn't always produce the same results, typically depending on external conditions

For simplicity, let's just say that nondeterminism means inner change. It isn't quite right, but many cases show nondeterminism as dependent on the side effects produced by other functions (e.g. mutating some variable) or on some specific conditions (e.g. the function ```get-unix-time``` will return the Unix time, which depends on time), so it's practical enough to be a rule of thumb.

Pure functions change nothing *and* don't change themselves. Impure functions change something *and* change themselves. Effected functions change something *but* don't change themselves, while nondeterministic functions change themselves *but* don't change anything outside of them.

These simple guarantees make many headaches with security, performance, formal verification, and so on disappear.
