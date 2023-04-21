type Word       = usize;     // a machine word
type SizeBytes  = Word;
type TypeHash   = u128;
type Byte       = u8;

type ReadOnly<T> = alloc::sync::Arc<T>;

struct ByteSlice
{ src : ReadOnly<[Byte]>
, offset_start : Word
, offset_end   : Word
}

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
{ alloc        : Allocation
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
{ Request(Type) // &self -> Type
, Into   (Type) // self  -> Type
, Innate (Type) // ()    -> Type
}

struct Unique
{ hash     : TypeHash
, val_type : Type
, props    : ReadOnly<[Property]>
}

enum Type 
{ Byte
, Word 
, Array   (ReadOnly<Array>)
, TypedMem(ReadOnly<TypedMem>)
, Unique  (ReadOnly<Unique>)
}