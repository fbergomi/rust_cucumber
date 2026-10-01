use std::fmt::Formatter;

/// A toy cat whose state changes when it is fed or starved.
/// This is the code under test of the Cucumber scenarios in `tests/`.
#[derive(Debug)]
pub struct Cat {
    pub hungry: bool,
    pub vomiting: bool,
    pub starving: bool,
    pub alive: bool,
}

/// By default a cat is alive and satiated.
impl Default for Cat {
    fn default() -> Self {
        Self {
            alive: true,
            hungry: false,
            vomiting: false,
            starving: false,
        }
    }
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
