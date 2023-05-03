#![allow(unused)]
extern crate alloc;
extern crate core;
use crate::bootstrap_cia::{Word, Byte};

pub(crate) type TypeHash = u128; 


pub(crate) type ByteSlice   = ReadOnly<[Byte]>;
pub(crate) type ReadOnly<T> = alloc::sync::Arc<T>;


pub(crate) struct Field 
{ pub(crate) offset: Word
, pub(crate) val_type : Type
}
pub(crate) struct TypedMem
{ pub(crate) type_byte_offsets : ReadOnly<[Field]> }


pub(crate) struct Array
{ /// the stride is computed by the `val_type`'s alignment and size
  pub(crate) val_type : ReadOnly<Type>
, pub(crate) len      : Word
}

pub(crate) enum Property
{ Request (Type) // &self     -> Type
, Update  (Type) // &mut self -> Type
, Into    (Type) // self      -> Type
, Innate  (Type) // ()        -> Type
}

pub(crate) struct Unique
{ pub(crate) module_path : ReadOnly<[Symbol]> // the last symbol is the name
, pub(crate) val_type    : ReadOnly<Type>
, pub(crate) props       : ReadOnly<[Property]>
}

pub(crate) struct CallConvention { /* mapping :  Lang_InOut -> MapArch_InOut */ }
pub(crate) struct RewriteRules { /* substitutions : ReadOnly<[ Ast -> Ast ]>  */ }// things like commutativity|associativity ... 

pub(crate) enum FnTags
{ CallConvention (CallConvention)
, RewriteRules   (RewriteRules)
}

/// can decay into a function pointer
pub(crate) struct FnStateFree
{ pub(crate) arg  : ReadOnly<Type>
, pub(crate) out  : ReadOnly<Type>

// should this really be here? maybe this should be left to Unique?
// , pub(crate) tags : ReadOnly<[FnTags]> // 
}

pub(crate) struct TypeUnion (pub(crate) ReadOnly<[Type]>);
pub(crate) struct Symbol(pub(crate) ByteSlice);

/// Concrete Types are a tree
pub(crate) enum Type
{ Type                  // maybe generic args instead

, Byte
, Word
, Unit  // guaranteed to be ZST

, SelfT // used to avoid reference cycles

, Symbol      (Symbol)
, Array       (Array)
, TypedMem    (TypedMem)
, TypeTag     (TypeHash)
, Unique      (Unique)
, FnStateFree (FnStateFree)

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

  ...

*/

