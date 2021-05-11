use crate::Mocker;

/// A built-in Mocker, which alternates between uppercase and lowercase.
pub struct AlternatingMocker {
    current: bool
}

impl AlternatingMocker {
    /// Creates a new AlternatingMocker. This by default starts with a lowercase letter, if you wanna control that, use `AlternatingMocker::new_from_bool()`.
    pub fn new() -> Self {
        Self {
            current: false
        }
    }
    /// Creates a new AlternatingMocker, and allows you to control the case of the starting letter with the parameter.
    pub fn new_from_bool(starting_case: bool) -> Self {
        Self {
            current: starting_case
        }
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
    /// Creates a new RandomMocker.
    pub fn new() -> Self {
        Self {}
    }
}

impl Mocker for RandomMocker {
    fn uppercase(&mut self) -> bool {
        rand::random()
    }
}

/// A built-in Mocker, which takes in a closure that returns a boolean. Useful if you don't feel like implementing the Mocker trait yourself.
pub struct ClosureMocker<T: Fn() -> bool> {
    closure: T
}

impl<T: Fn() -> bool> ClosureMocker<T> {
    /// Creates a new ClosureMocker.
    /// The parameter must be a closure which returns a `bool`.
    /// The closure will be executed to determine whether the letter should be uppercase or not.
    pub fn new(closure: T) -> Self {
        Self {
            closure: closure
        }
    }
}

impl<T: Fn() -> bool> Mocker for ClosureMocker<T> {
    fn uppercase(&mut self) -> bool {
        (self.closure)()
    }
}