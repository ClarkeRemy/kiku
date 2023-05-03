use crate::{bootstrap_cia::*, type_cia::FnStateFree, core};

struct ByteStream{}
struct TokenStream;

struct StreamCont<S, Yield, O> {state : S, yields : Yield, next :  fn(S)->StreamProcess<S, Yield, O> }
enum StreamProcess<S, Yield, O>
{ Continue(StreamCont<S, Yield, O>)
, Complete(O)
}

macro_rules! SC {() => {StreamCont<Self::State, Self::Yield, Self::Output>};}
macro_rules! SP {() => {StreamProcess<Self::State, Self::Yield, Self::Output>};}
trait Stream
{ type Source
; type State
; type Yield
; type Output
; fn init_state(src : Self::Source) -> Self::State
; fn step_in(cont : Self::State) -> SP!()
; fn step_once(curr : SC!()) -> SP!();
}

trait ConsumeStream<T> where T : Stream
{ type YieldConsumer
; fn consume
  ( sp : StreamProcess<T::State, T::Yield, T::Output>
  , yc : (Self::YieldConsumer , fn(Self::YieldConsumer, T::Yield)->Self::YieldConsumer)
  ) -> (T::Output, Self::YieldConsumer);
}

trait Tokeniser 
{ fn tokenise(bytes : ByteStream) -> TokenStream
;
}

fn h () 
{
}