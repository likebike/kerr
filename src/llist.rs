use std::fmt;
use std::rc::Rc;
use std::mem;

// An immutable linked list, custom-tailored for our error chain:
pub(crate) struct LList<T>(Option<Rc<LLNode<T>>>);
struct LLNode<T> {
    el  : T,
    next: LList<T>,
}

impl<T> LList<T> {
    pub(crate) fn new() -> Self { LList(None) }
    pub(crate) fn prepend(&self, el:T) -> Self {
        LList(Some(Rc::new(LLNode{el:el,
                                  next:LList::clone(self)})))
    }
    pub(crate) fn head<'a>(&'a self) -> Option<&'a T> {
        self.0.as_ref().map(|rc| &rc.el)
    }

    pub(crate) fn iter(&self) -> Iter<T> { Iter(LList::clone(self)) }
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

pub(crate) struct Iter<T>(LList<T>);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa_llist() {
        let l : LList<String> = LList::new();
        assert_eq!(format!("{}", l), "[]");
        assert_eq!(format!("{:?}", l), "LList[]");
        let l = l.prepend("a".to_string());
        assert_eq!(format!("{}", l), "[ a ]");
        assert_eq!(format!("{:?}", l), r#"LList[ "a" ]"#);
        let l = l.prepend("b".to_string());
        assert_eq!(format!("{}", l), "[ b a ]");
        assert_eq!(format!("{:?}", l), r##"LList[ "b" "a" ]"##);

        let mut l : LList<i32> = LList::new();
        for i in 5..25 { l = l.prepend(i) }
        assert_eq!(format!("{}", l), "[ 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 ]");
    }
}

