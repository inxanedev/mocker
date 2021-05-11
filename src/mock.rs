/// Trait representing a Mocker.
/// A Mocker is a little utility which has a method which determines if the current letter should be uppercase or not.
pub trait Mocker {
    /// Returns true if the current character should be uppercase or not.
    fn uppercase(&mut self) -> bool;
}

/// Function that performs the mocking. It takes in a string type and a mocker by reference.
/// It will return a String with the mocking performed.
/// # Examples
///
/// ```
/// let data = String::from("hello world");
/// let result = mock(&data, &AlternatingMocker::new());
/// 
/// assert_eq!(String::from("hElLo wOrLd"), result);
/// ```
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