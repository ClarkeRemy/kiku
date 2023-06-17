use crate::{bootstrap_cia::*, type_cia::{Symbol}};

trait Tokeniser 
{ type Source
; type ByteStreamState
; type ByteStream : Iteration<State = Self::ByteStreamState, Yield = Byte>
; type TokenStreamState
; type TokenStream : Iteration<State = Self::TokenStreamState, Yield = Symbol>
; fn init_state(s : Self::Source) -> <Self::ByteStream as Iteration>::State
; fn init_token_stream(bytes : <Self::ByteStream as Iteration>::State ) -> <Self::TokenStream as Iteration>::State
; fn tokenise(s  : Self::Source) -> <Self::TokenStream as Iteration>::State 
  { Self::init_token_stream(Self::init_state(s)) }
}



enum IterProc<State, Yield>
{ Yield( (State, Yield) )
, Empty( State )
}

trait Iteration
{ type State
; type Yield
; fn step_iter(s : Self::State) -> IterProc<Self::State, Self::Yield>
; 
}


/*
Module
=> TypedMem 
   { type_byte_offsets : 
     [ Field { offset : 0 , val_type : Slice<Slice> } // module members
     , Field { offset : _ , val_type : Slice<Slice<Symbol>>}// module_paths to definitions
     ]
   }

*/