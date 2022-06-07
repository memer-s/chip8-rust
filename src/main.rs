mod chip8;
use std::io::Read;

fn main() {
	let mut c = chip8::Chip::new();
	c.load(std::fs::read("./Airplane.ch8").expect("Could not read ROM file."));
	c.print_memory(0x200);
	let mut frame_cound = 0;
	loop {
		frame_cound+=1;
		println!("{}", frame_cound);

		// 2 reads are required for some reason...
		if frame_cound%5==0 {
			std::io::stdin().read(&mut [0]).unwrap();
			std::io::stdin().read(&mut [0]).unwrap();
		}
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
		c.print_registers();
	}
}
