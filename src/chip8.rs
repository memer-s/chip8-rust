use colored::Colorize;

enum Mode {
	Normal,
	Debug
}

pub struct Chip {
	memory: [u8;4096],
	stack: [u16;16],
	registers: [u8;16],
	address_register: u16,
	pc: u16,
	sp: u16,
	mode: Mode,
	display: [[bool;64];32]
}

impl Chip {
	pub fn new() -> Chip {
		let mut mem: [u8; 4096] = [0; 4096];
		let fontset: [u8; 80] = [
			0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
			0x20, 0x60, 0x20, 0x20, 0x70, // 1
			0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
			0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
			0x90, 0x90, 0xf0, 0x10, 0x10, // 4
			0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
			0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
			0xf0, 0x10, 0x20, 0x40, 0x40, // 7
			0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
			0xf0, 0x90, 0xf0, 0x10, 0xe0, // 9
			0xf0, 0x80, 0x80, 0x80, 0xf0, // A
			0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
			0xf0, 0x80, 0x80, 0x80, 0xf0, // C
			0xe0, 0x90, 0x90, 0x90, 0xe0, // D
			0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
			0xf0, 0x80, 0xf0, 0x80, 0x80  // F
		];

		for i in 0..fontset.len() {
			mem[i] = fontset[i];
		}

		Chip {
			memory: mem,
			stack: [0; 16],
			registers: [0; 16],
			address_register: 0x0,
			pc: 0x200,
			sp: 0x0,
			mode: Mode::Debug,
			display: [[false; 64]; 32]
		}
	}

	pub fn load(&mut self, rom: Vec<u8>) {
		for i in 0..rom.len() {
			self.memory[i+0x200] = rom[i];
		}
	}

	pub fn print_memory(&self, pos: u16) {
		println!("\n\n   ---  0x{:X} - 0x{:X}  ---  PC: {}, SP: {:03X}",
			(pos/256)*256, 
			((pos/256)+1)*256,
			format!("{:03X}",self.pc).red(),
			self.sp
		);

		let start = (pos/256)*256;
		let end = ((pos/256)+1)*256;

		for i in start..end {
			// Check if memory is in bounds.
			if i < (self.memory.len() as u16) {
				if i == self.pc {
					print!("{}{:02X}", "▸".red(), self.memory[i as usize]);
				}
				else {
					print!(" {:02X}", self.memory[i as usize]);
				}
			}
			if (i+1)%32 == 0 {
				print!("\n");
			}
		}
	}

	pub fn get_display(&self) -> [[bool; 64]; 32] {
		self.display
	}

	pub fn step(&mut self) -> u16 {
		let b1 = self.memory[self.pc as usize];
		let b2 = self.memory[(self.pc+1) as usize];
		let both = ((b1 as u16) << 8) + b2 as u16;

		let nnn: u16 = (((b1&0x0f) as u16) << 8)+(b2 as u16);
		let nn: u8 = b1;
		let n: u8 = b1 & 0x0f;

		let x: usize = (b1 & 0x0f) as usize;
		let y: usize = (b1 & 0xf0) as usize;

		match (b1 & 0xf0) >> 4 {
			0x0 => {
				match both {
					0x00E0 => {
						for i in 0..32 {
							for j in 0..64 {
								self.display[i][j] = false;
							}
						}
					},
					0x00EE => {
						
					},

					_ => {

					},
				}
				self.pc+=2;
			},
			0x1 => {
				self.pc = nnn;
			},
			0x2 => {

			},
			0x3 => {
				if self.registers[x] == nn {
					self.pc += 2;
				}
				self.pc += 2;
			},
			0x4 => {
				if self.registers[x] != nn {
					self.pc += 2;
				}
				self.pc += 2
			},
			0x5 => {
				if self.registers[x] == self.registers[y] {
					self.pc += 2;
				}
				self.pc += 2;
			},
			0x6 => {
				self.registers[x] = nn;
				self.pc += 2;
			},
			0x7 => {
				self.registers[x] = ((self.registers[x] as u16 + nn as u16) & 0x00FF) as u8;
				self.pc += 2;
			},
			0x8 => {
				match n {
					0x0 => {
						self.registers[x] =  self.registers[y];
					},
					0x1 => {
						self.registers[x] |= self.registers[y];
					},
					0x2 => {
						self.registers[x] &= self.registers[y];
					},
					0x3 => {
						self.registers[x] ^= self.registers[y];
					},
					0x4 => {
						self.registers[x] += self.registers[y];
					},
					0x5 => {
						self.registers[x] -= self.registers[y];
					},
					0x6 => {
						self.registers[0xf] = self.registers[x] & 0x1;
						self.registers[x] >>= 1;
					},
					0x7 => {
						self.registers[x] = self.registers[y] - self.registers[x];
					}
					0xE => {
						self.registers[0xf] = self.registers[x] & 0x80;
						self.registers[x] <<= 1;
					},
					_ => {
						println!("INVALID OPCODE: {:04X}", both);
					}
				}
				self.pc += 2;
			},
			0x9 => {
				if self.registers[x] != self.registers[y] {
					self.pc += 2;
				}
				self.pc += 2;
			},
			0xA => {
				self.address_register = nnn;
				self.pc += 2;
			},
			0xB => {
				self.pc = self.registers[0] as u16 + nnn;
			},
			0xC => {
				// Use random number generation...........
				self.registers[x] = 127 & nn;
				self.pc += 2;
			},
			0xD => {
				self.registers[0xf] = 0;
				for i in 0..n {
					let byte = self.memory[i as usize + self.address_register as usize];
					for j in 0..8 {
						let current_pixel = ((byte << j) & 0x80) as u8;
						if current_pixel == 1 {
							self.display[(self.registers[y] + i) as usize][(self.registers[x] + j) as usize] = true; 
						}
					}
				}
				self.display[2][2] = true;
				self.pc += 2;
			},
			0xE => {

			},
			0xF => {
				match b2 {
					0x07 => {
						// GEt delay
					},
					0x0A => {
						// GEt delay
					},
					0x15 => {
						// Set delay
					},
					0x18 => {
						// Set delay
					},
					0x1E => {
						self.address_register += self.registers[x] as u16;
					},
					0x29 => {
						self.address_register = x as u16 * 5;
						self.pc += 2;
					},
					0x33 => {

					},
					0x55 => {
						for i  in 0..x {
							self.memory[(self.address_register + (i as u16)) as usize] = self.registers[i];
						}
					},
					0x65 => {
						for i  in 0..x {
							self.registers[i] = self.memory[(self.address_register + (i as u16)) as usize];
						}
					},
					_ => {
						println!("INVALID OPCODE: {:04X}", both)
					}
				}
			},
			_ => {
				println!("INVALID OPCODE: {:04X}", both);
			}
		}
		self.pc
	}
}
