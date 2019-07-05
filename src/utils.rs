use crate::text::Text;
use std::cmp::max;
use yaml_rust::yaml::{Hash, Yaml};

/// Get the number of base 10 digits in a number.
///
/// # Examples
///
/// ```rust
/// # use songmaster_rs::utils::num_digits;
/// assert_eq!(2, num_digits(12));
/// assert_eq!(3, num_digits(900));
/// ```
pub fn num_digits(mut number: usize) -> usize {
    let mut count = 0;
    while number != 0 {
        number /= 10;
        count += 1;
    }
    max(count, 1)
}

/// Creates a text that is the given list of text separated by commas.
///
/// # Examples
///
/// ```rust
/// # use songmaster_rs::{text::Text, utils::comma_separated};
/// let text = [Text::new("foo"), Text::with_ascii("bar", "baar"), Text::new("baz")];
/// assert_eq!(Text::with_ascii("foo, bar, baz", "foo, baar, baz"), comma_separated(&text[..]));
/// ```
pub fn comma_separated(text: &[Text]) -> Text {
    let mut res = Text::new("");
    let sep = Text::new(", ");
    for (i, t) in text.iter().enumerate() {
        if i != 0 {
            res += &sep;
        }
        res += t;
    }
    res
}

pub enum HashDeErr {
    KeyNotFound,
    InvalidValue,
}

pub fn parse_key_from_hash<F, T>(
    hash: &mut Hash,
    key: &'static str,
    transform: &F,
) -> Result<T, HashDeErr>
where
    F: Fn(Yaml) -> Option<T>,
{
    let value = pop!(hash[key]).ok_or(HashDeErr::KeyNotFound)?;
    transform(value).ok_or(HashDeErr::InvalidValue)
}

pub fn parse_vec_key_from_hash<F, T>(
    hash: &mut Hash,
    key: &'static str,
    transform: &F,
) -> Result<Vec<T>, HashDeErr>
where
    F: Fn(Yaml) -> Option<T>,
{
    let value = pop!(hash[key]).ok_or(HashDeErr::KeyNotFound)?;
    value
        .into_vec()
        .ok_or(HashDeErr::InvalidValue)?
        .into_iter()
        .map(transform)
        .collect::<Option<Vec<_>>>()
        .ok_or(HashDeErr::InvalidValue)
}

pub fn parse_singular_or_plural<F, T>(
    hash: &mut Hash,
    singular: &'static str,
    plural: &'static str,
    transform: F,
) -> Result<Vec<T>, HashDeErr>
where
    F: Fn(Yaml) -> Option<T>,
{
    if let Ok(value) = parse_key_from_hash(hash, singular, &transform) {
        return Ok(vec![value]);
    }

    parse_vec_key_from_hash(hash, plural, &transform)
}

#[cfg(test)]
mod tests {
    use super::{comma_separated, num_digits, Text};

    macro_rules! num_digits_tests {
        ($( $name:ident($num:expr, $digits:expr); )* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(num_digits($num), $digits);
                }
            )*
        }
    }

    num_digits_tests! {
        zero_has_one_digit(0, 1);
        one_has_one_digit(1, 1);
        two_has_one_digit(2, 1);
        nine_has_one_digit(9, 1);
        ten_has_two_digits(10, 2);
        eleven_has_two_digits(11, 2);
        ninety_nine_has_two_digits(99, 2);
        one_hundred_has_three_digits(100, 3);
        one_hundred_and_one_has_three_digits(101, 3);
    }

    #[test]
    fn comma_separated_empty_vec_is_empty() {
        let text = &[];
        assert_eq!(comma_separated(text), Text::new(""));
    }

    #[test]
    fn comma_separated_single_is_same() {
        let text = &[Text::with_ascii("foo", "bar")];
        assert_eq!(comma_separated(text), Text::with_ascii("foo", "bar"));
    }

    #[test]
    fn comma_separated_double_is_correct() {
        let text = &[Text::with_ascii("foo", "bar"), Text::new("baz")];
        assert_eq!(
            comma_separated(text),
            Text::with_ascii("foo, baz", "bar, baz")
        );
    }

    #[test]
    fn comma_separated_triple_is_correct() {
        let text = &[
            Text::with_ascii("foo", "bar"),
            Text::new("baz"),
            Text::with_ascii("quux", "other"),
        ];
        assert_eq!(
            comma_separated(text),
            Text::with_ascii("foo, baz, quux", "bar, baz, other")
        );
    }
}
