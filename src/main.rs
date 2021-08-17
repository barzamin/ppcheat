use core::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, multispace0},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Register(pub u8);

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Opcode {
    /// Rotate Left Word Immediate then AND with Mask
    Rlwinm {
        ra: Register,
        rs: Register,
        sh: u8,
        mb: u8,
        me: u8,
    },

    /// Rotate Left Word Immediate then Mask Insert
    Rlwimi {
        ra: Register,
        rs: Register,
        sh: u8,
        mb: u8,
        me: u8,
    },

    /// Rotate Left Word then AND with Mask
    Rlwnm {
        ra: Register,
        rs: Register,
        rb: Register,
        mb: u8,
        me: u8,
    },
}

fn parse_register(inp: &str) -> IResult<&str, Register> {
    preceded(
        tag("r"),
        map_res(digit1, |x: &str| x.parse::<u8>().map(Register)),
    )(inp)
}

fn parse_immediate(inp: &str) -> IResult<&str, u8> {
    alt((
        map_res(preceded(tag("0x"), hex_digit1), |x: &str| {
            u8::from_str_radix(x, 16)
        }),
        map_res(digit1, |x: &str| x.parse::<u8>()),
    ))(inp)
}

fn whitespace(inp: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ())(inp)
}

fn comma_sep(inp: &str) -> IResult<&str, ()> {
    map(tuple((multispace0, tag(","), multispace0)), |_| ())(inp)
}

fn parse_rlwinm(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rlwinm"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, sh, mb, me)| Opcode::Rlwinm { ra, rs, sh, mb, me },
        ),
    )(inp)
}

fn parse_rlwimi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rlwimi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, sh, mb, me)| Opcode::Rlwimi { ra, rs, sh, mb, me },
        ),
    )(inp)
}

fn parse_rlwnm(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rlwimi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, rb, mb, me)| Opcode::Rlwnm { ra, rs, rb, mb, me },
        ),
    )(inp)
}

fn parse_opcode(inp: &str) -> IResult<&str, Opcode> {
    alt((parse_rlwinm, parse_rlwimi, parse_rlwnm))(inp)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlwinm() {
        let asm = "rlwinm r0,r7,0x10,0x0,0xf";
        let (_, op) = parse_opcode(asm).expect("parse failed");

        assert_eq!(
            op,
            Opcode::Rlwinm {
                ra: Register(0),
                rs: Register(7),
                sh: 16,
                mb: 0,
                me: 15
            }
        );
    }
}
