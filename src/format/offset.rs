//! Formatting helpers for a `UtcOffset`.

#![allow(non_snake_case)]

use super::parse::{try_consume_exact_digits_in_range, try_consume_first_match};
use super::{Padding, ParseError, ParseResult, ParsedItems};
use crate::{Sign, UtcOffset};
use core::fmt::{self, Formatter};

/// UTC offset
pub(crate) fn fmt_z(f: &mut Formatter<'_>, offset: UtcOffset) -> fmt::Result {
    let offset = offset.as_duration();

    write!(
        f,
        "{:+03}{:02}",
        offset.whole_hours(),
        offset.whole_minutes() - 60 * offset.whole_hours()
    )
}

/// UTC offset
pub(crate) fn parse_z(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    let sign = try_consume_first_match(
        s,
        [("+", Sign::Positive), ("-", Sign::Negative)]
            .iter()
            .cloned(),
    )
    .ok_or(ParseError::InvalidOffset)?;

    let hours: i16 = try_consume_exact_digits_in_range(s, 2, 0..24, Padding::Zero)
        .ok_or(ParseError::InvalidOffset)?;

    let minutes = try_consume_exact_digits_in_range(s, 2, 0..60, Padding::Zero)
        .ok_or(ParseError::InvalidOffset)?;

    items.offset = UtcOffset::minutes(sign * hours * 60 + minutes).into();
    Ok(())
}