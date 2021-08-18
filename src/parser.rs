use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, multispace0},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};

use crate::Opcode;
use crate::Register;

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
        tag("rlwnm"),
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

fn parse_extlwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("extlwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n, b)| Opcode::Extlwi { ra, rs, n, b },
        ),
    )(inp)
}

fn parse_extrwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("extrwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n, b)| Opcode::Extrwi { ra, rs, n, b },
        ),
    )(inp)
}

fn parse_rotlwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rotlwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Rotlwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_rotrwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rotrwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Rotrwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_slwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("slwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Slwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_srwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("srwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Srwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_clrlwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("clrlwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Clrlwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_clrrwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("clrrwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n)| Opcode::Clrrwi { ra, rs, n },
        ),
    )(inp)
}

fn parse_clrlslwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("clrlslwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, b, n)| Opcode::Clrlslwi { ra, rs, b, n },
        ),
    )(inp)
}

fn parse_rotlw(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("rotlw"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_register),
            )),
            |(ra, rs, rb)| Opcode::Rotlw { ra, rs, rb },
        ),
    )(inp)
}

fn parse_inslwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("inslwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n, b)| Opcode::Inslwi { ra, rs, n, b },
        ),
    )(inp)
}

fn parse_insrwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("insrwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep, parse_register),
                preceded(comma_sep, parse_immediate),
                preceded(comma_sep, parse_immediate),
            )),
            |(ra, rs, n, b)| Opcode::Insrwi { ra, rs, n, b },
        ),
    )(inp)
}

pub fn parse_opcode(inp: &str) -> IResult<&str, Opcode> {
    alt((
        parse_rlwinm,
        parse_rlwimi,
        parse_rlwnm,
        parse_extlwi,
        parse_extrwi,
        parse_rotlwi,
        parse_rotrwi,
        parse_slwi,
        parse_srwi,
        parse_clrlwi,
        parse_clrrwi,
        parse_clrlslwi,
        parse_rotlw,
        parse_inslwi,
        parse_insrwi,
    ))(inp)
}
