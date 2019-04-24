use memchr::{memchr, memchr2};

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Cookie<'a> {
    Percent(&'a str),
    Slash(&'a str, &'a str),
}

impl<'a> Cookie<'a> {
    #[inline]
    pub(crate) fn parse(src: &str) -> Option<(Cookie<'_>, usize)> {
        debug_assert!(src.starts_with('['));

        let bytes = src.as_bytes();
        let num1 =
            memchr2(b'%', b'/', bytes).filter(|&i| bytes[1..i].iter().all(u8::is_ascii_digit))?;

        if bytes[num1] == b'%' && *bytes.get(num1 + 1)? == b']' {
            Some((Cookie::Percent(&src[1..num1]), num1 + 2))
        } else {
            let num2 = memchr(b']', bytes)
                .filter(|&i| bytes[num1 + 1..i].iter().all(u8::is_ascii_digit))?;

            Some((Cookie::Slash(&src[1..num1], &src[num1 + 1..num2]), num2 + 1))
        }
    }
}

#[test]
fn parse() {
    assert_eq!(
        Cookie::parse("[1/10]"),
        Some((Cookie::Slash("1", "10"), "[1/10]".len()))
    );
    assert_eq!(
        Cookie::parse("[1/1000]"),
        Some((Cookie::Slash("1", "1000"), "[1/1000]".len()))
    );
    assert_eq!(
        Cookie::parse("[10%]"),
        Some((Cookie::Percent("10"), "[10%]".len()))
    );
    assert_eq!(
        Cookie::parse("[%]"),
        Some((Cookie::Percent(""), "[%]".len()))
    );
    assert_eq!(
        Cookie::parse("[/]"),
        Some((Cookie::Slash("", ""), "[/]".len()))
    );
    assert_eq!(
        Cookie::parse("[100/]"),
        Some((Cookie::Slash("100", ""), "[100/]".len()))
    );
    assert_eq!(
        Cookie::parse("[/100]"),
        Some((Cookie::Slash("", "100"), "[/100]".len()))
    );

    assert_eq!(Cookie::parse("[10% ]"), None);
    assert_eq!(Cookie::parse("[1//100]"), None);
    assert_eq!(Cookie::parse("[1\\100]"), None);
    assert_eq!(Cookie::parse("[10%%]"), None);
}
