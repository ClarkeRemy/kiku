Much of this needs revisiting


# Move Semantics
The Language supports move semantics and used deterministic destruction
If a type has only a destructor it has move only semantics.
(A compiler query can show where are duplicates made for performance review)

# Functions
Functions take one argument and return one value
```
f x = double x
```
Functions suport currying notation for multiple arguments
```
f x y = add x y
```
This can be simplified, but to keep it clear a type signature can be added
```
f : fn num (fn_c num num)
f = add
```
Functions can be used in type signatures
```
curried = reduce fn_c
f : curried [num ; num ; num]
f = add
```
This is powerful, but curried values can complicate the function type.
if we have things like mutable references and movable values, they require different types
```
f : fn int (fn_c (ref _ int) (fn_m resource (fn_o num unit))
f _ _ _ = []
```
As such the ordering of curries is important.
fn   = const     closure (value erasure -> function pointer)
fn_d = AutoClone closure (it can be called freely at runtime)
fn_m = mut       closure (multiple calls can result in different values)
fn_o = once      closure (can only be called once)
all versions can have avriants that can be Cloned

as such idiomatic ordering is like this

const -> copy -> mut -> move



# A Declarative language
All lines written declare truths.
pattern matching is possible
as such compile time assertions are as simple as a function call on a new line

This program fails to compile
```
one : fn byte bool
one
| 1 = true
| _ = false
 
one 5
```

# Types and values
Types are composed of primitives.
They include :
  Intrinsic
    word   (Architecture machine word)
    mem    (contiguous fixed size memory of bytes)

  Built from intrinsic
    record (contiguous memory that maps tokens to fields)
    union  (a type tag with enough memory to hold any type variants)

Possible Extention
  struct (a C repr struct for ffi and allocation)
  enum   (Similar to a Rust Enum, variants are values, would exist for performance/ffi)

When a value is parsed, if the type has not yet been infered, The type needs assertion.
If the assertion passes, the type is bound.

## Declaring your own type alias
Declarations of type alias creates compile time identity functions that assert a type
```
ascii = byte
a = ascii 43
```
Note that the type aliases of primitives are transparent and structural.
This mild form of weak typing is deemed acceptable, and even useful for row polymorphism.
Stronger typng is availible with the module system
```
dinner = mem 4 byte

ascii a
byte  a
```

  Record (takes tokens as static keys, declaration order does not matter)
```
co-ords = record [f64 _.x ; f64 _.y]
```
  Tuples (a record where tokens must be numerals between 0..elements, declaration order matters)
```
co-ords_classic = record [ posit64 _.0 ; posit64 _.1]
co-ords         = tuple [posit64 ; posit64]

eq co-ords_classic co-ords

x : co-ords_classic
x = [5.5 ; 6.6]

y : co-ords
y = [5.5 ; 6.6]
```
because of structural typing
```
eq x y
```

  TypeSet    (contiguous memory that maps types to fields of that type)


# Defining Signatures
Signatures define the public interface of a type.
Conversely, omitted implementations are excluded from that interface.
Once a type is signed, the defining module becomes the owner of that type.

# Implementing Signatures
Only the owner of a type can implement a signature.
The Owner of a type can implement as many signatures as they like,
from as many modules as they like.
(unlike Rust you __cannot__ extend another module's signed types)

# Overloaded Functions 
Generic function using

todo improve example
```
h : [Iterator T; G ; H ] -> T -> G -> H
h | [vector, ..] t g = h [1]
  | [..]         t g = h []
```

# Semantics vs syntax
Semantics are based on module system
Some semantics can be configured at the module level (eg. Auto-Clone)

Users can always overide syntax (eg. operator precedence)
Alias types or basically any name
Maybe even things like the `=` for definitions

# What can you ask the compiler?
The compiler has a top level similar to Prolog where you can make queries
  Asserting a type can return true
  The query can fail and return false
  If the query includes variables, if it cannot return true, it will iterate over possible answers of things that could work.

You can ask for the different levels of ir/assembly

You can call other functions while in the middle of debugging

The module system can be queried for 
- visibility of names
- dependancy DAG

Execute compile time functions in the repl/top level

# What can the compiler do?
You can modify the dependacy DAG, while the original is preserved

You can refactor code while in the repl, and it will save to your files (structured refactoring)

# Define the compiler Interface to the user
Effectively The compler is type with a signature, and every module starts with one instace of the compiler.
2 Passes
  - The meta-compiler reads for compiler directives/extentions to create a new compiler instance that satisfies the compiler signature.
  - Then the new compiler instance compiles the code with new behavior (but maintains the signature)

## The Compiler Signature
The Compiler Signature incudes
```rust
trait Lexer { // iterator
  type Token : Copy;
  type Source
  fn next(&mut self) -> Option<Self::Token>
}

trait Parser {
  type Token : Copy;
  type Ast;

  fn new(lexer: impl Lexer<Token = Self::Token>)
  fn parse(&mut self, )
}

trait TypeChecker {
  type Ast;
  type Tir;
  fn new(parser : Parser<Ast = Self::Ast>) -> Self;
  fn check(&mut self, ast: Self::Ast) -> Result<Self::Tir, TypeError>
}
```