# Move Semantics
The Language supports move semantics
If a type has only a destructor it has move only semantics.
If a type has a duplicator it defaults to auto-clone semantics.
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
fn   = const closure (value erasure -> function pointer)
fn_d = dup   closure (it can be called freely at runtime)
fn_m = mut   closure (multiple calls can result in different values)
fn_o = once  closure (can only be called once)
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
  byte   (fixed sized adressable)
  word   (Architecture machine word)
  mem    (contiguous fixed size memory)
  record (contiguous memory that maps tokens to fields)
  union  (a type tag with enough memory to hold any type variants)

Possible Extention
  struct (a C repr struct for ffi and allocation)
  enum   (Similar to a Rust Enum, variants are values, would exist for performance/ffi)

When a value is parsed, if the type has not yet been infered, The type will be asserted.
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
f64  co-ords.x _
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
  Symbol