use std::fmt;
use std::fmt::Formatter;

pub struct Register {
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .finish()
    }
}