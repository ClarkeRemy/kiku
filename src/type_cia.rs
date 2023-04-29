// #![allow(unused)]
extern crate alloc;
extern crate core;

use crate::bootstrap_cia::{Word, Byte};

type TypeHash = u128; 


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
{ hash     : TypeHash  // salt instead?
, val_type : ReadOnly<Type>
, props    : ReadOnly<[Property]>
}

struct CallConvention
{ // mapping :  Lang_InOut -> MapArch_InOut
}

enum FnTags 
{ CallConvention
}

/// can decay into a function pointer
struct FnStateless // can 
{ arg : ReadOnly<Type>
, out : ReadOnly<Type>
, 
// call convention
// safety
// 
}

struct TypeUnion (ReadOnly<[Type]>);

/// Concrete Types are a tree
enum Type
{ Type                  // maybe generic args instead
, Symbol    (ByteSlice) // Expecting Utf-8?

, Byte
, Word
, Unit

, SelfT // this is only be found in propery signatures, used to avoid reference cycles

, Array       (Array)
, TypedMem    (TypedMem)
, TypeTag     (TypeHash)
, Unique      (Unique)
, FnStateless (FnStateless)

, TypeUnion   (TypeUnion)
}

/*
  Internal of Record and Struct
  Record and Struct
  => TypedMem
     { type_byte_offsets :
       [ 
         Field { offset : 0 , val_type : Slice<Symbol> } 
       , Field { offset : _ , val_type : TypedMem { type_byte_offsets : [ ..... ] }
      ]
    }
  internaly uses 2D indexing to get pairs of names and types

  Record with numeric Fields from 0..=n => Tuple


  Function


  k =
  | s32 
  | i32


*/

