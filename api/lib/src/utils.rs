use nom::bytes::complete::{tag, take_until};
use nom::character::complete::alphanumeric1;
use nom::character::is_alphanumeric;
use nom::combinator::opt;
use nom::IResult;

pub fn prune_escape(input: &str) -> IResult<&str, &str> {
    let (rem, _) = opt(tag("\""))(input)?;
    Ok((rem, ""))
}

pub fn prune_str(input: &str) -> IResult<&str, &str> {
    let (rem, _) = prune_escape(input)?;
    let (_, word) = alphanumeric1(rem)?;
    Ok((word, ""))
}

pub fn uuid_part(input: &str) -> IResult<&str, &str> {
    let (rem, _) = opt(tag("-"))(input)?;
    let (rem, part) = take_until("-")(rem)?;
    Ok((rem, part))
}
