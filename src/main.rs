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

    // ---- pseudomnemonics ----

    Extlwi {
        ra: Register,
        rs: Register,
        n: u8,
        b: u8,
    },

    Extrwi {
        ra: Register,
        rs: Register,
        n: u8,
        b: u8,
    },

    Rotlwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Rotrwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Slwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Srwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Clrlwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Clrrwi {
        ra: Register,
        rs: Register,
        n: u8,
    },

    Clrlslwi {
        ra: Register,
        rs: Register,
        b: u8,
        n: u8,
    },

    Rotlw {
        ra: Register,
        rs: Register,
        rb: Register,
    },

    Inslwi {
        ra: Register,
        rs: Register,
        n: u8,
        b: u8,
    },

    Insrwi {
        ra: Register,
        rs: Register,
        n: u8,
        b: u8,
    },
}

impl Opcode {
    pub fn highlevel(&self) -> String {
        match self {
            Self::Rlwinm { ra, rs, sh, mb, me } => {
                format!("{dest} = ({src} << {sh}) & MASK({mb}..{me})", dest=ra, src=rs, sh=sh, mb=mb, me=me)
            },
            _ => unimplemented!(),
        }
    }

    pub fn canonicalize(&self) -> Self {
        match self {
            &Self::Rlwinm { .. } => self.clone(),
            &Self::Rlwimi { .. } => self.clone(),
            &Self::Rlwnm  { .. } => self.clone(),

            &Self::Inslwi { ra, rs, n, b } => Self::Rlwinm { ra, rs, sh: 32-b, mb: b, me: b+n-1 },
            &Self::Insrwi { ra, rs, n, b } => Self::Rlwinm { ra, rs, sh: 32-(b+n), mb: b, me: (b+n)-1 },

            &Self::Extlwi { ra, rs, n, b } => Self::Rlwinm { ra, rs, sh: b, mb: 0, me: n-1 },
            &Self::Extrwi { ra, rs, n, b } => Self::Rlwinm { ra, rs, sh: b + n, mb: 32-n, me: 31 },

            &Self::Rotlwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: n, mb: 0, me: 31 },
            &Self::Rotrwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: 32-n, mb: 0, me: 31 },

            &Self::Slwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: n, mb: 0, me: 31-n },
            &Self::Srwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: 32-n, mb: n, me: 31 },

            &Self::Clrlwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: 0, mb: n, me: 31 },
            &Self::Clrrwi { ra, rs, n } => Self::Rlwinm { ra, rs, sh: 0, mb: 0, me: 31-n },

            &Self::Clrlslwi { ra, rs, b, n } => Self::Rlwinm { ra, rs, sh: n, mb: b-n, me: 31-n},

            &Self::Rotlw { ra, rs, rb } => Self::Rlwnm { ra, rs, rb, mb: 0, me: 31 },
        }
    }
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_register),
            )),
            |(ra, rs, rb)| Opcode::Rotlw { ra, rs, rb, },
        ),
    )(inp)
}

fn parse_inslwi(inp: &str) -> IResult<&str, Opcode> {
    preceded(
        tag("inslwi"),
        map(
            tuple((
                preceded(whitespace, parse_register),
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
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
                preceded(comma_sep,  parse_register),
                preceded(comma_sep,  parse_immediate),
                preceded(comma_sep,  parse_immediate),
            )),
            |(ra, rs, n, b)| Opcode::Insrwi { ra, rs, n, b },
        ),
    )(inp)
}

fn parse_opcode(inp: &str) -> IResult<&str, Opcode> {
    alt((
        parse_rlwinm,
        parse_rlwimi,
        parse_rlwnm,
        parse_extlwi, parse_extrwi,
        parse_rotlwi, parse_rotrwi,
        parse_slwi, parse_srwi,
        parse_clrlwi, parse_clrrwi,
        parse_clrlslwi,
        parse_rotlw,
    ))(inp)
}

fn main() {
    let asm = "rlwinm r0,r7,0x10,0x0,0xf";
    let (_, op) = parse_opcode(asm).unwrap();
    println!("{}", asm);
    println!("{}", op.highlevel());
}

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
