Strongly typed      // linear or affine types?
functions           // take 1 argument return 1 argument
(
  Tag               // unit type
  Tag union         // enum
  tuple types       // can now take or return 1 or more arguments; nested pairs like Idris?
  pattern matching  // bool
  Dependant types
)
  type    inference  // can be limited, enough for variables
  tagged  types      // wrapper types
  tagged  unions

  record  types



Generics
  const/copy currying (auto currying for const and copy types, but not mut types)
  SMl / OCaml modules  ?  // what does this mean with dependant types?
  Typeclass    (typed) ?
  overloadable literals
  overloadable syntax (list syntax) // tuple and vector use same syntax?
Macros are implemented after ?


tokenizing identifiers requires whitespace

punt on Concrete syntax (can be discussed, but we avoid implementing it)
Start from Abstract syntax
(f a b)       = ((f a) b)
(f [a b])     = (f (cons a b))            // note this a tuple argument
(f [a b])     = (f (cons a (cons b nil))) // note f now takes a list
[(f [a b]) 5]



RAII
Tail Call Elimination ABI


When you ask the compiler the definition you get it
The language IR is first class (everything expands to intrinsics)




interesting...
```
tuple A&B:  (A -> B -> T) -> T
union A|B:  (A -> T) -> (B -> T) -> T
```