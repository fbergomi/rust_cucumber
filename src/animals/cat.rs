use std::fmt::{Formatter};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
pub struct Cat {
    pub hungry: bool,
    pub vomit: bool,
    pub starving: bool,
    pub alive : bool,
}



impl Cat {
    pub fn feed(&mut self) {
        if self.alive{
        if self.hungry {
            self.starving = false;//finally!
            self.hungry = false;
            self.vomit = false;
        } else {
            self.hungry = false;
            self.starving = false;
            self.vomit = true;
        }
    }
    }

    pub fn starve(&mut self) {
        if self.starving {
            self.alive = false;
        }

        if self.alive {

        if self.hungry {
            self.starved=true;
        } else {
            self.hungry = true;
            self.vomit = false;
        }
    }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {

        let status = match self.starved{
            true => "dead",
            _ => ""
        };

        let verb = match self.starved{
            true => "was",
            _ => "is"
        };

        let hungry = match self.hungry {
            true => "hungry",
            _ => "not hungry",
        };

        let vomit = match self.vomit {
            true => "and vomits",
            _ => "and does not vomit",
        };

        write!(f, "A {status} cat who {verb} {hungry} {vomit}.")
    }
}
