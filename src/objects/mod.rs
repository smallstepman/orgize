/// objects
///
/// objects is something that included in an element.
pub(crate) mod cookie;
pub(crate) mod emphasis;
pub(crate) mod fn_ref;
pub(crate) mod inline_call;
pub(crate) mod inline_src;
pub(crate) mod link;
pub(crate) mod macros;
pub(crate) mod radio_target;
pub(crate) mod snippet;
pub(crate) mod target;
pub(crate) mod timestamp;

pub use self::cookie::Cookie;
pub use self::fn_ref::FnRef;
pub use self::inline_call::InlineCall;
pub use self::inline_src::InlineSrc;
pub use self::link::Link;
pub use self::macros::Macros;
pub use self::snippet::Snippet;
pub use self::timestamp::*;
