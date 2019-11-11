use super::llist::LList;

use std::fmt;
use std::error::Error;

pub struct KErr {
    pub err   : String,
    chain : LList<String>,
}

impl KErr {
    pub fn new(err:&str) -> Self {
        Self{err:err.to_string(), chain:LList::new()}
    }
    pub fn pre(mut self, info:&str) -> Self {
        self.chain = self.chain.prepend(info.to_string());
        self
    }
}
impl fmt::Display for KErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut nonempty = false;
        for node in self.chain.iter() {
            if nonempty { write!(f, " : ")?; }
            nonempty = true;
            write!(f, "{}", node.head().unwrap())?;
        }
        if nonempty { write!(f, " : ")?; }
        write!(f, "{}", self.err)?;
        Ok(())
    }
}
impl fmt::Debug for KErr {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "KErr{{ err:{:?}, chain:{} }}", self.err, self.chain)
    }
}
impl PartialEq for KErr {
    fn eq(&self, other:&Self) -> bool { self.err==other.err }
}

impl Error for KErr {
    // The defaults are fine for now.
    // Maybe I should create a custom 'source()' implementation...
}

