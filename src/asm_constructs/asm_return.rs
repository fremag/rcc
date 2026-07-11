use std::fmt;
use std::fmt::Formatter;

pub struct AsmReturn {
}

impl fmt::Debug for AsmReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsmReturn")
            .finish()
    }
}