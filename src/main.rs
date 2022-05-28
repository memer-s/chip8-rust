mod chip8;
use std::io::Read;

fn main() {
	let mut c = chip8::Chip::new();
	c.load(std::fs::read("./IBM.ch8").expect("Could not read ROM file."));
	c.print_memory(0x200);
	loop {
		std::io::stdin().read(&mut [0]).unwrap();
		let pc = c.step();
		let disp = c.get_display();
		print!("\n\n");
		for i in disp {
			for j in i {
				if j {
					print!("⬜")
				}
				else {
					print!("⬛")
				}
			}
			print!("\n")
		}
		c.print_memory(pc);
	}
}
