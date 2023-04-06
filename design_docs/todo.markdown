Strongly typed      // linear or affine types?
Types are always copyable values ?
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
f a b         = (f a b)        = ((f a) b)
(f [a b])     = (f (cons a b))             // note this a tuple argument
(f [a b])     = (f (cons a (cons b nil)))  // note f now takes a list
[(f [a b]) 5]



RAII
Tail Call Elimination ABI

Dependancy tree is always acyclic


When you ask the compiler the definition you get it
The language IR is first class (everything expands to intrinsics)


namespacing for structural typing and  overloading
https://deque.blog/2017/06/14/10-things-idris-improved-over-haskell/


'basicaly' SML modules
```rust
trait Foo  // this is effectively an SML signature
{ type a;
  fn bar()->i32;
}

struct J; // in SML J is only made at the same time it is implemented
impl Foo for J {
  type a = Vec<i32>;
  fn bar()->i32 {5}
}
fn hoo<T:Foo>()->i32{ T::bar() }
```


interesting...
```
tuple A&B:  (A -> B -> T) -> T
union A|B:  (A -> T) -> (B -> T) -> T
```