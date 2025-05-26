

/// An rgba color. 
/// 
/// The alpha channel goes from 0-100.
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Default)]
pub struct Rgba(u8,u8,u8,u8);

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Default)]
pub struct Color<C>(C);

impl<C> Color<C>{
    fn rgba(r:u8,g:u8,b:u8,a:u8) -> Color<Rgba>{
        Color(Rgba(r, g, b, a))
    }
}