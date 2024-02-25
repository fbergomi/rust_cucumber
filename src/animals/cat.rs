use std::fmt::Formatter;

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
pub struct Cat {
    pub hungry: bool,
    pub vomiting: bool,
    pub starving: bool,
    pub alive: bool,
}

impl Cat {
    pub fn feed(&mut self) {
        if !self.alive {
            return;
        }

        if self.starving {
            self.starving = false;
            self.hungry = true;
            self.vomiting = false;
        } else if self.hungry {
            self.starving = false; //finally!
            self.hungry = false;
            self.vomiting = false;
        } else if self.vomiting {
            self.alive = false; //dead
            self.hungry = false; //whatever
            self.vomiting = false; //whatever
            self.starving = false;
        } else {
            self.hungry = false;
            self.starving = false;
            self.vomiting = true;
        }
    }

    pub fn starve(&mut self) {
        if !self.alive {
            return;
        }
        if self.starving {
            self.alive = false;
            self.starving = false;
            self.hungry = false;
            self.vomiting = false;
            return;
        }

        if self.alive {
            if self.hungry {
                self.starving = true;
            } else {
                self.hungry = true;
                self.vomiting = false;
                self.starving = false;
            }
        }
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let status = match self.alive {
            true => "alive",
            _ => "dead",
        };

        let hungry = match self.hungry {
            true => "hungry",
            _ => "not hungry",
        };

        let health = {
            if self.vomiting {
                "vomits"
            } else if self.starving {
                "starves"
            } else {
                "feels ok"
            }
        };
        write!(f, "A cat ({status}) who is {hungry} and {health}.")
    }
}
