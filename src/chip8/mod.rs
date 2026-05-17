use std::time::{Duration, Instant};

mod rand;
use rand::Rand;

mod input;
use input::Chip8Keypad;

mod instruction;
use instruction::{Chip8Instruction, Chip8Register};

pub struct Chip8Config {
    pub cpu_hz: u32,
    pub display_hz: u32,

    pub reset_vf: bool,
    pub increment_i: bool,
    pub clip_sprites: bool,
    pub shift_vy_instead_vx: bool,
    pub jump_vx_instead_v0: bool,
    pub wait_for_vblank: bool,
}

impl Chip8Config {
    pub fn default() -> Self {
        Self {
            cpu_hz: 700,
            display_hz: 60,

            reset_vf: true,
            increment_i: true,
            clip_sprites: true,
            shift_vy_instead_vx: true,
            jump_vx_instead_v0: false,
            wait_for_vblank: true,
        }
    }
}

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub stack: [u16; 16],

    pub v_x: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,

    pub delay_timer: u8,
    pub sound_timer: u8,
    last_timer_tick: Instant,

    pub display: [bool; 64 * 32],
    pub display_changed: bool,
    vblank: bool,
    pub buzzer_hz: u16,

    pub keypad: Chip8Keypad,
    pub rand: Rand,

    pub config: Chip8Config,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0u8; 4096],
            stack: [0u16; 16],

            v_x: [0u8; 16],
            i: 0,
            pc: 0x200,
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            last_timer_tick: Instant::now(),

            display: [false; 64 * 32],
            display_changed: false,
            vblank: false,
            buzzer_hz: 420,

            keypad: Chip8Keypad::new(),
            rand: Rand { seed: 0 },
            config: Chip8Config::default(),
        };

        const FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        chip8.memory[0..80].copy_from_slice(&FONT);
        return chip8;
    }

    pub fn load_program_from_file(&mut self, file_dir: &str) -> Result<(), std::io::Error> {
        let bytes = std::fs::read(file_dir)?;
        for (i, byte) in bytes.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
        Ok(())
    }

    fn execute(&mut self, op: Chip8Instruction) {
        match op {
            Chip8Instruction::CLS => {
                self.display = [false; 64 * 32];
                self.display_changed = true
            }
            Chip8Instruction::RET => {
                if self.sp != 0 {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
            }
            Chip8Instruction::JP { addr } => self.pc = addr,
            Chip8Instruction::CALL { addr } => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Chip8Instruction::SE_BYTE { vx, byte } => {
                let vx = self.v_x[vx as usize];
                if vx == byte {
                    self.pc += 2;
                }
            }
            Chip8Instruction::SNE_BYTE { vx, byte } => {
                let vx = self.v_x[vx as usize];
                if vx != byte {
                    self.pc += 2;
                }
            }
            Chip8Instruction::SE { vx, vy } => {
                let vx = self.v_x[vx as usize];
                let vy = self.v_x[vy as usize];

                if vx == vy {
                    self.pc += 2;
                }
            }
            Chip8Instruction::LD_BYTE { vx, byte } => {
                self.v_x[vx as usize] = byte;
            }
            Chip8Instruction::ADD_BYTE { vx, byte } => {
                self.v_x[vx as usize] = self.v_x[vx as usize].wrapping_add(byte);
            }
            Chip8Instruction::LD { vx, vy } => {
                self.v_x[vx as usize] = self.v_x[vy as usize];
            }
            Chip8Instruction::OR { vx, vy } => {
                self.v_x[vx as usize] |= self.v_x[vy as usize];
                if self.config.reset_vf {
                    self.v_x[Chip8Register::VF as usize] = 0;
                }
            }
            Chip8Instruction::AND { vx, vy } => {
                self.v_x[vx as usize] &= self.v_x[vy as usize];
                if self.config.reset_vf {
                    self.v_x[Chip8Register::VF as usize] = 0;
                }
            }
            Chip8Instruction::XOR { vx, vy } => {
                self.v_x[vx as usize] ^= self.v_x[vy as usize];
                if self.config.reset_vf {
                    self.v_x[Chip8Register::VF as usize] = 0;
                }
            }
            Chip8Instruction::ADD { vx, vy } => {
                let carry =
                    (self.v_x[vx as usize] as u16 + self.v_x[vy as usize] as u16 > 255) as u8;
                self.v_x[vx as usize] = self.v_x[vx as usize].wrapping_add(self.v_x[vy as usize]);
                self.v_x[Chip8Register::VF as usize] = carry;
            }
            Chip8Instruction::SUB { vx, vy } => {
                let borrow = (self.v_x[vx as usize] >= self.v_x[vy as usize]) as u8;
                self.v_x[vx as usize] = self.v_x[vx as usize].wrapping_sub(self.v_x[vy as usize]);
                self.v_x[Chip8Register::VF as usize] = borrow;
            }
            Chip8Instruction::SHR { vx, vy } => {
                if self.config.shift_vy_instead_vx {
                    self.v_x[vx as usize] = self.v_x[vy as usize];
                }

                let shifted_bit = ((self.v_x[vx as usize] & 0x01) == 0x01) as u8;
                self.v_x[vx as usize] >>= 1;
                self.v_x[Chip8Register::VF as usize] = shifted_bit;
            }
            Chip8Instruction::SUBN { vx, vy } => {
                let borrow = (self.v_x[vy as usize] >= self.v_x[vx as usize]) as u8;

                self.v_x[vx as usize] = self.v_x[vy as usize].wrapping_sub(self.v_x[vx as usize]);
                self.v_x[Chip8Register::VF as usize] = borrow;
            }
            Chip8Instruction::SHL { vx, vy } => {
                if self.config.shift_vy_instead_vx {
                    self.v_x[vx as usize] = self.v_x[vy as usize];
                }

                let shifted_bit = ((self.v_x[vx as usize] & 0x80) == 0x80) as u8;
                self.v_x[vx as usize] = self.v_x[vx as usize] << 1;
                self.v_x[Chip8Register::VF as usize] = shifted_bit;
            }
            Chip8Instruction::SNE { vx, vy } => {
                let vx = self.v_x[vx as usize];
                let vy = self.v_x[vy as usize];

                if vx != vy {
                    self.pc += 2;
                }
            }
            Chip8Instruction::LD_I { addr } => {
                self.i = addr;
            }
            Chip8Instruction::JP_V0 { addr, vx } => {
                let reg = if self.config.jump_vx_instead_v0 {
                    self.v_x[vx as usize]
                } else {
                    self.v_x[0]
                };

                self.pc = reg as u16 + addr;
            }
            Chip8Instruction::RND { vx, byte } => {
                let rng = self.rand.range(0, 255);
                self.v_x[vx as usize] = rng as u8 & byte;
            }
            Chip8Instruction::DRW { vx, vy, nibble } => {
                if !self.vblank && self.config.wait_for_vblank {
                    self.pc -= 2;
                    return;
                }
                self.vblank = false;
                self.display_changed = true;

                let reg_x = (self.v_x[vx as usize] % 64) as usize;
                let reg_y = (self.v_x[vy as usize] % 32) as usize;
                let n = nibble as usize;

                self.v_x[Chip8Register::VF as usize] = 0;

                for offset in 0..n {
                    let byte = self.memory[(self.i + offset as u16) as usize];
                    for bit in 0..8 {
                        let pixel = (byte >> (7 - bit)) & 1;
                        let sprite_x = reg_x + bit;
                        let sprite_y = reg_y + offset;

                        if self.config.clip_sprites && (sprite_x >= 64 || sprite_y >= 32) {
                            continue;
                        }

                        let display_coord = (sprite_y % 32) * 64 + sprite_x % 64;

                        if pixel == 1 && self.display[display_coord] == true {
                            self.v_x[Chip8Register::VF as usize] = 1;
                        }
                        self.display[display_coord] ^= pixel == 1;
                    }
                }
            }
            Chip8Instruction::SKP { vx } => {
                let Some(key) = self.keypad.get_key() else {
                    return;
                };

                if key as u8 == self.v_x[vx as usize] {
                    self.pc += 2;
                }
            }
            Chip8Instruction::SKNP { vx } => {
                let Some(key) = self.keypad.get_key() else {
                    self.pc += 2;
                    return;
                };

                if key as u8 != self.v_x[vx as usize] {
                    self.pc += 2;
                }
            }
            Chip8Instruction::LD_DT_to_VX { vx } => {
                self.v_x[vx as usize] = self.delay_timer;
            }
            Chip8Instruction::LD_Key_to_VX { vx } => {
                let Some(key) = self.keypad.get_key() else {
                    self.pc -= 2;
                    return;
                };

                self.v_x[vx as usize] = key as u8;
            }
            Chip8Instruction::LD_VX_to_DT { vx } => {
                self.delay_timer = self.v_x[vx as usize];
            }
            Chip8Instruction::LD_VX_to_ST { vx } => {
                self.sound_timer = self.v_x[vx as usize];
            }
            Chip8Instruction::ADD_I { vx } => {
                self.i = self.i.wrapping_add(self.v_x[vx as usize] as u16);
            }
            Chip8Instruction::LD_SpriteLoc_to_I { vx } => {
                self.i = (self.v_x[vx as usize] as u16) * 5;
            }
            Chip8Instruction::LD_BCD_VX_to_I { vx } => {
                let val = self.v_x[vx as usize];
                self.memory[self.i as usize] = val / 100;
                self.memory[self.i as usize + 1] = (val / 10) % 10;
                self.memory[self.i as usize + 2] = val % 10;
            }
            Chip8Instruction::LD_V0_VX_to_I { vx } => {
                let n = vx as usize + 1;
                let mut index = self.i as usize;
                for offset in 0..n {
                    self.memory[if self.config.increment_i {
                        self.i as usize
                    } else {
                        index
                    }] = self.v_x[offset];

                    if self.config.increment_i {
                        self.i += 1;
                    } else {
                        index += 1;
                    }
                }
            }
            Chip8Instruction::LD_I_to_V0_VX { vx } => {
                let n = vx as usize + 1;
                let mut index = self.i as usize;
                for offset in 0..n {
                    self.v_x[offset] = self.memory[if self.config.increment_i {
                        self.i as usize
                    } else {
                        index
                    }];

                    if self.config.increment_i {
                        self.i += 1;
                    } else {
                        index += 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn tick_timers(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_timer_tick).as_secs_f64();
        let display_delay = (Duration::from_secs(1) / self.config.display_hz).as_secs_f64();

        if elapsed < display_delay {
            return false;
        }

        self.last_timer_tick = now - Duration::from_secs_f64(elapsed % display_delay);

        self.vblank = true;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        return true;
    }

    pub fn step(&mut self) {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[self.pc as usize + 1] as u16;
        let opcode: u16 = (hi << 8) + lo;

        self.pc += 2;

        let op = Chip8Instruction::decode(opcode);
        self.execute(op);
    }
}
