// #![allow(unused)]
extern crate alloc;

type Word     = usize;     // a machine word
type TypeHash = u128;
type Byte     = u8;

type ByteSlice   = ReadOnly<[Byte]>;
type ReadOnly<T> = alloc::sync::Arc<T>;


struct Allocation
{ align : Word
, size  : Word
}


struct ArrayMemDesc
{ alloc  : Allocation
, len    : Word
, stride : Word
}

// a typeless memory description for tuples, structs, and records
struct MemDesc
{ alloc       : Allocation
, byte_offsets : ReadOnly<[Word]> // number of fields is implicit
                                  // the offsets must always be increasing
                                  // if the type is a ZST, the slice has len 0
}

struct TypedMem
{ val_type : ReadOnly<[Type]> // Sorted in MemDesc memory order
, access   : MemDesc
}

struct Array
{ val_type : Type
, access   : ArrayMemDesc
}

enum Property
{ Request (Type) // &self -> Type
, Into    (Type) // self  -> Type
, Innate  (Type) // ()    -> Type
}

struct Unique
{ hash     : TypeHash
, val_type : Type
, props    : ReadOnly<[Property]>
}

struct FnPointer
{ arg : Type
, out : Type
}

/// Concrete Types are a tree
enum Type 
{ Type
, Byte
, Word
, SELF  // this is only be found in propery signatures, used to avoid reference cycles
, Array     (ReadOnly<Array>)
, TypedMem  (ReadOnly<TypedMem>)
, TypeTag   (TypeHash)
, Unique    (ReadOnly<Unique>) // {_.align = Max _.type.align _.align ; }
, FnPointer (ReadOnly<FnPointer>) // {_.align = Word.align ; _.size = Bytes 8}
}

/*

  Record : Array TypedMem [ ]
  Module : (TypedMem,
 */
