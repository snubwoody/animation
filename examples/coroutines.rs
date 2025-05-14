#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
use std::{ops::{Coroutine, CoroutineState}, pin::Pin};

fn main(){
    let mut coroutine = #[coroutine] || {
        yield 1;
        yield 2;
        yield 3;
        return "done"
    };

    
    loop {
        let coro = Pin::new(&mut coroutine);
        match coro.resume(()){
        CoroutineState::Yielded(val) => {
            dbg!(val);
        }
        CoroutineState::Complete(val) => {
            dbg!(val);
            break;
        }
    }
    }
}