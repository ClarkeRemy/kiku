// #![allow(unused)]
extern crate alloc;
extern crate core;

type Word     = usize;     // a machine word
type TypeHash = u128;
type Byte     = u8;

type ByteSlice   = ReadOnly<[Byte]>;
type ReadOnly<T> = alloc::sync::Arc<T>;


struct Field 
{ offset: Word
, val_type : Type
}
struct TypedMem
{ type_byte_offsets : ReadOnly<[Field]> }


struct Array
{ /// the stride is computed by the `val_type`'s alignment and size
  val_type : ReadOnly<Type>
, len      : Word
}

enum Property
{ Request (Type) // &self     -> Type
, Update  (Type) // &mut self -> Type
, Into    (Type) // self      -> Type
, Innate  (Type) // ()        -> Type
}

struct Unique
{ hash     : TypeHash
, val_type : ReadOnly<Type>
, props    : ReadOnly<[Property]>
}

struct FnPointer
{ arg : ReadOnly<Type>
, out : ReadOnly<Type>
}

/// Concrete Types are a tree
enum Type
{ Type // maybe generic args instead
, Byte
, Word
, Unit
, SELF  // this is only be found in propery signatures, used to avoid reference cycles
, Array     (Array)
, TypedMem  (TypedMem)
, TypeTag   (TypeHash)
, Unique    (Unique)
, FnPointer (FnPointer)
}
