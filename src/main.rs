use std::fs;

struct Chip {
	memory: Vec<u8>,
	registers: Vec<u8>,
	address_register: u16,
	pc: u16,
	sp: u16,
}

impl Chip {
	fn new() -> Chip {
		Chip {
			memory: Vec::new(),
			registers: Vec::new(),
			address_register: 0x0,
			pc: 0x200,
			sp: 0x0,
		}
	}

	fn print_memory(self, pos: u16) {

		println!(" ---  0x{:X} - 0x{:X}  --- ", (pos/256)*256, ((pos/256)+1)*256);

		let start = (pos/256)*256;
		let end = ((pos/256)+1)*256;

		for i in start..end {
			// Check if memory is in bounds.
			if i < (self.memory.len() as u16) {
				print!("{:02X} ", self.memory[i as usize]);
			}
			else {
				print!("00 ")
			}
			if (i+1)%32 == 0 {
				print!("\n");
			}
		}
	}

	fn step(self) {
		let b1 = self.memory[self.pc as usize];
		let b2 = self.memory[(self.pc+1) as usize];

		let nnn: u16 = ((b1 as u16) << 8)+(b2 as u16);
		let nn: u8 = b1;
		let n: u8 = b1 & 0x0f;

		let X = b1 & 0x0f;
		let Y = b2 & 0xf0;
	}
}

fn main() {
	let mut c = Chip::new();
	c.memory = std::fs::read("./test.txt").expect("Could not read ROM file.");
	c.print_memory(257);
}