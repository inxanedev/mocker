/// Trait representing a Mocker.
/// A Mocker is a little utility which has a method which determines if the current letter should be uppercase or not.
pub trait Mocker {
    /// Returns true if the current character should be uppercase or not.
    fn uppercase(&mut self) -> bool;
}

pub fn mock<T: AsRef<str>, U: Mocker>(text: &T, mocker: &mut U) -> String {
    let data = text.as_ref();

    let size = data.chars().count();
    let mut result = String::with_capacity(size);

    for c in data.chars() {
        if mocker.uppercase() {
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}