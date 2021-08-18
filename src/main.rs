use core::fmt;

mod parser;
use parser::parse_opcode;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Register(pub u8);

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
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
                format!(
                    "{dest} = ({src} << {sh}) & MASK({mb}..{me})",
                    dest = ra,
                    src = rs,
                    sh = sh,
                    mb = mb,
                    me = me
                )
            }
            _ => unimplemented!(),
        }
    }

    pub fn canonicalize(&self) -> Self {
        match self {
            &Self::Rlwinm { .. } => self.clone(),
            &Self::Rlwimi { .. } => self.clone(),
            &Self::Rlwnm { .. } => self.clone(),

            &Self::Inslwi { ra, rs, n, b } => Self::Rlwinm {
                ra,
                rs,
                sh: 32 - b,
                mb: b,
                me: b + n - 1,
            },
            &Self::Insrwi { ra, rs, n, b } => Self::Rlwinm {
                ra,
                rs,
                sh: 32 - (b + n),
                mb: b,
                me: (b + n) - 1,
            },

            &Self::Extlwi { ra, rs, n, b } => Self::Rlwinm {
                ra,
                rs,
                sh: b,
                mb: 0,
                me: n - 1,
            },
            &Self::Extrwi { ra, rs, n, b } => Self::Rlwinm {
                ra,
                rs,
                sh: b + n,
                mb: 32 - n,
                me: 31,
            },

            &Self::Rotlwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: n,
                mb: 0,
                me: 31,
            },
            &Self::Rotrwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: 32 - n,
                mb: 0,
                me: 31,
            },

            &Self::Slwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: n,
                mb: 0,
                me: 31 - n,
            },
            &Self::Srwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: 32 - n,
                mb: n,
                me: 31,
            },

            &Self::Clrlwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: 0,
                mb: n,
                me: 31,
            },
            &Self::Clrrwi { ra, rs, n } => Self::Rlwinm {
                ra,
                rs,
                sh: 0,
                mb: 0,
                me: 31 - n,
            },

            &Self::Clrlslwi { ra, rs, b, n } => Self::Rlwinm {
                ra,
                rs,
                sh: n,
                mb: b - n,
                me: 31 - n,
            },

            &Self::Rotlw { ra, rs, rb } => Self::Rlwnm {
                ra,
                rs,
                rb,
                mb: 0,
                me: 31,
            },
        }
    }
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
