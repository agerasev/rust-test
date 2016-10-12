extern crate mm;

struct FPSCounter<'a> {
	m: &'a mm::System,
	start: u32,
	frames: u32,
	pub fps: f64,
}

impl<'a> FPSCounter<'a> {
	pub fn new(m: &mm::System) -> FPSCounter {
		FPSCounter { m: m, start: m.get_ticks(), frames: 0, fps: 0.0 }
	}
	pub fn count(&mut self) {
		self.frames += 1;
		let ticks = self.m.get_ticks();
		let diff = ticks - self.start;
		if diff > 1000 {
			self.fps = (self.frames as f64) * 1000.0/(diff as f64);
			self.frames = 0;
			self.start += (diff/1000)*1000;
			println!("fps: {:.2}", self.fps);
		}
	}
}

fn main() {
	let mut i = 0;
	let m = mm::System::new(800, 600).unwrap();
	let mut fpscnt = FPSCounter::new(&m);
	while m.handle() == 0 {
		let (w,h) = m.get_size();
		for ix in 0..w {
			for iy in 0..h {
				m.draw_pixel(ix, iy, i | (0xff << 3*8));
			}
		}
		m.update();
		// m.sleep(40);
		fpscnt.count();
		i += 1;
	}
}
