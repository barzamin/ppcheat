use nom::{IResult, branch::alt, bytes::complete::tag, character::complete::{digit1, hex_digit1, multispace0, multispace1}, combinator::{map, map_res}, sequence::{preceded, tuple}};


#[derive(Debug, Copy, Clone)]
struct Register(pub u8);

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Rlwinm {
        ra: Register,
        rs: Register,
        sh: u8,
        mb: u8,
        me: u8,
    },
}

fn parse_register(inp: &str) -> IResult<&str, Register> {
    preceded(tag("r"),
        map_res(digit1, |x: &str| x.parse::<u8>().map(Register)))(inp)
}

fn parse_immediate(inp: &str) -> IResult<&str, u8> {
    alt((
        map_res(preceded(tag("0x"), hex_digit1), |x: &str| u8::from_str_radix(x, 16)),
        map_res(digit1, |x: &str| x.parse::<u8>())
    ))(inp)
}

fn whitespace(inp: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ())(inp)
}

fn comma_sep(inp: &str) -> IResult<&str, ()> {
    map(tuple((multispace0, tag(","), multispace0)), |_| ())(inp)
}

fn parse_rlwinm(inp: &str) -> IResult<&str, Opcode> {
    preceded(tag("rlwinm"),
        map(tuple((
            preceded(whitespace, parse_register),
            preceded(comma_sep, parse_register),
            preceded(comma_sep, parse_immediate),
            preceded(comma_sep, parse_immediate),
            preceded(comma_sep, parse_immediate),
        )), |(ra, rs, sh, mb, me)| Opcode::Rlwinm {ra, rs, sh, mb, me})
    )(inp)
}

fn parse_opcode(inp: &str) -> IResult<&str, Opcode> {
    // alt((
    //     parse_rlwinm,
    // ))(inp)
    parse_rlwinm(inp)
}

fn main() {
    let test = "rlwinm r0,r7,0x10,0x0,0xf";

    println!("parse_opcode: {:?}", parse_opcode(test));
}
