use crate::Mocker;

/// A built-in Mocker, which alternates between uppercase and lowercase.
pub struct AlternatingMocker {
    current: bool
}

impl AlternatingMocker {
    pub fn new() -> Self {
        Self {
            current: false
        }
    }
}

impl Default for AlternatingMocker {
    fn default() -> Self {
        Self::new()
    }
}

impl Mocker for AlternatingMocker {
    fn uppercase(&mut self) -> bool {
        self.current = !self.current;
        !self.current
    }
}

/// A built-in Mocker, which randomly makes characters uppercase or lowercase.
pub struct RandomMocker;

impl RandomMocker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RandomMocker {
    fn default() -> Self {
        Self::new()
    }
}

impl Mocker for RandomMocker {
    fn uppercase(&mut self) -> bool {
        rand::random()
    }
}

