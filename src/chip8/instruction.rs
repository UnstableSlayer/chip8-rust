macro_rules! enum_from_u8 {
    ($name:ident { $($variant:ident = $val:expr),* }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn from_u8(value: u8) -> Option<Self> {
                match value {
                    $($val => Some($name::$variant),)*
                    _ => None,
                }
            }
        }
    }
}

enum_from_u8!(Chip8Register {
    V0 = 0x0,
    V1 = 0x1,
    V2 = 0x2,
    V3 = 0x3,
    V4 = 0x4,
    V5 = 0x5,
    V6 = 0x6,
    V7 = 0x7,
    V8 = 0x8,
    V9 = 0x9,
    VA = 0xA,
    VB = 0xB,
    VC = 0xC,
    VD = 0xD,
    VE = 0xE,
    VF = 0xF
});

#[derive(Debug, Clone, Copy)]
pub enum Chip8Instruction {
    CLS,
    RET,
    JP {
        addr: u16,
    },
    JP_V0 {
        addr: u16,
        vx: Chip8Register,
    },
    CALL {
        addr: u16,
    },
    SNE {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    SNE_BYTE {
        vx: Chip8Register,
        byte: u8,
    },
    SE {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    SE_BYTE {
        vx: Chip8Register,
        byte: u8,
    },
    LD {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    LD_BYTE {
        vx: Chip8Register,
        byte: u8,
    },
    LD_I {
        addr: u16,
    },
    LD_DT_to_VX {
        vx: Chip8Register,
    },
    LD_VX_to_DT {
        vx: Chip8Register,
    },
    LD_VX_to_ST {
        vx: Chip8Register,
    },
    LD_SpriteLoc_to_I {
        vx: Chip8Register,
    },
    LD_BCD_VX_to_I {
        vx: Chip8Register,
    },
    LD_V0_VX_to_I {
        vx: Chip8Register,
    },
    LD_I_to_V0_VX {
        vx: Chip8Register,
    },
    LD_Key_to_VX {
        vx: Chip8Register,
    },
    OR {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    AND {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    XOR {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    ADD {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    ADD_BYTE {
        vx: Chip8Register,
        byte: u8,
    },
    ADD_I {
        vx: Chip8Register,
    },
    SUB {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    SHR {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    SHL {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    SUBN {
        vx: Chip8Register,
        vy: Chip8Register,
    },
    RND {
        vx: Chip8Register,
        byte: u8,
    },
    DRW {
        vx: Chip8Register,
        vy: Chip8Register,
        nibble: u8,
    },
    SKP {
        vx: Chip8Register,
    },
    SKNP {
        vx: Chip8Register,
    },
    INVALID,
}

impl Chip8Instruction {
    pub fn decode(opcode: u16) -> Chip8Instruction {
        return match opcode {
            0x00E0 => Chip8Instruction::CLS,
            0x00EE => Chip8Instruction::RET,
            opcode if (opcode & 0xF000) == 0x1000 => Chip8Instruction::JP {
                addr: opcode & 0x0FFF,
            },
            opcode if (opcode & 0xF000) == 0x2000 => Chip8Instruction::CALL {
                addr: (opcode & 0x0FFF),
            },
            opcode if (opcode & 0xF000) == 0x3000 => Chip8Instruction::SE_BYTE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                byte: (opcode & 0x00FF) as u8,
            },
            opcode if (opcode & 0xF000) == 0x4000 => Chip8Instruction::SNE_BYTE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                byte: (opcode & 0x00FF) as u8,
            },
            opcode if (opcode & 0xF00F) == 0x5000 => Chip8Instruction::SE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF000) == 0x6000 => Chip8Instruction::LD_BYTE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                byte: (opcode & 0x00FF) as u8,
            },
            opcode if (opcode & 0xF000) == 0x7000 => Chip8Instruction::ADD_BYTE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                byte: (opcode & 0x00FF) as u8,
            },
            opcode if (opcode & 0xF00F) == 0x8000 => Chip8Instruction::LD {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8001 => Chip8Instruction::OR {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8002 => Chip8Instruction::AND {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8003 => Chip8Instruction::XOR {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8004 => Chip8Instruction::ADD {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8005 => Chip8Instruction::SUB {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8006 => Chip8Instruction::SHR {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x8007 => Chip8Instruction::SUBN {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x800E => Chip8Instruction::SHL {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF00F) == 0x9000 => Chip8Instruction::SNE {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
            },
            opcode if (opcode & 0xF000) == 0xA000 => Chip8Instruction::LD_I {
                addr: (opcode & 0x0FFF),
            },
            opcode if (opcode & 0xF000) == 0xB000 => Chip8Instruction::JP_V0 {
                addr: (opcode & 0x0FFF),
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF000) == 0xC000 => Chip8Instruction::RND {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                byte: (opcode & 0x00FF) as u8,
            },
            opcode if (opcode & 0xF000) == 0xD000 => Chip8Instruction::DRW {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
                vy: Chip8Register::from_u8(((opcode & 0x00F0) >> 4) as u8).unwrap(),
                nibble: (opcode & 0x000F) as u8,
            },
            opcode if (opcode & 0xF0FF) == 0xE09E => Chip8Instruction::SKP {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xE0A1 => Chip8Instruction::SKNP {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF007 => Chip8Instruction::LD_DT_to_VX {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF00A => Chip8Instruction::LD_Key_to_VX {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF015 => Chip8Instruction::LD_VX_to_DT {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF018 => Chip8Instruction::LD_VX_to_ST {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF01E => Chip8Instruction::ADD_I {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF029 => Chip8Instruction::LD_SpriteLoc_to_I {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF033 => Chip8Instruction::LD_BCD_VX_to_I {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF055 => Chip8Instruction::LD_V0_VX_to_I {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            opcode if (opcode & 0xF0FF) == 0xF065 => Chip8Instruction::LD_I_to_V0_VX {
                vx: Chip8Register::from_u8(((opcode & 0x0F00) >> 8) as u8).unwrap(),
            },
            _ => Chip8Instruction::INVALID,
        };
    }
}
