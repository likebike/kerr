use std::fmt;
use std::rc::Rc;
use std::mem;

// An immutable linked list, custom-tailored for our error chain:
pub struct LList<T>(Option<Rc<LLNode<T>>>);
pub struct LLNode<T> {
    el  : T,
    next: LList<T>,
}

impl<T> LList<T> {
    pub fn new() -> Self { LList(None) }
    pub fn prepend(&self, el:T) -> Self {
        LList(Some(Rc::new(LLNode{el:el,
                                  next:LList::clone(self)})))
    }
    fn head<'a>(&'a self) -> Option<&'a T> {
        self.0.as_ref().map(|rc| &rc.el)
    }

    fn iter(&self) -> Iter<T> { Iter(LList::clone(self)) }
}

impl<T> Clone for LList<T> {
    fn clone(&self) -> Self {
        match self.0 {
            Some(ref rc) => LList(Some(Rc::clone(rc))),
            None => LList(None),
        }
    }
}

impl<T> fmt::Display for LList<T> where T:fmt::Display {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[")?;

        let mut nonempty = false;
        for node in self.iter() {
            nonempty = true;
            write!(f, " {}", node.head().unwrap())?;
        }

        if nonempty { write!(f, " ")?; }
        write!(f, "]")?;
        Ok(())
    }
}
impl<T> fmt::Debug for LList<T> where T:fmt::Debug {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "LList[")?;

        let mut nonempty = false;
        for node in self.iter() {
            nonempty = true;
            write!(f, " {:?}", node.head().unwrap())?;
        }

        if nonempty { write!(f, " ")?; }
        write!(f, "]")?;
        Ok(())
    }
}

struct Iter<T>(LList<T>);

impl<T> Iterator for Iter<T> {
    type Item = LList<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref rc) = (self.0).0 {
            // Rust doesn't support inter-line non-lexical scoping.  We need to help it out:
            let next = LList::clone(&rc.next);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

