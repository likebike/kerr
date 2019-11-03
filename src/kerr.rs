
#[derive(Debug)]
pub struct KErr {
    pub err   : String,
    pub chain : LList<String>,
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
impl PartialEq for KErr {
    fn eq(&self, other:&Self) -> bool { self.err==other.err }
}