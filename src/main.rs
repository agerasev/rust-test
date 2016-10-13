extern crate mm;

struct FPSCounter {
	start: u32,
	frames: u32,
	pub fps: f64,
}

impl FPSCounter {
	pub fn new() -> FPSCounter {
		FPSCounter { start: mm::get_ticks(), frames: 0, fps: 0.0 }
	}
	pub fn count(&mut self) {
		self.frames += 1;
		let ticks = mm::get_ticks();
		let diff = ticks - self.start;
		if diff > 1000 {
			self.fps = (self.frames as f64) * 1000.0/(diff as f64);
			self.frames = 0;
			self.start += (diff/1000)*1000;
			println!("fps: {:.2}", self.fps);
		}
	}
}

fn xn(x: (f64, f64), c: (f64, f64)) -> (f64, f64) {
	(x.0*x.0 - x.1*x.1 + c.0, 2.0*x.0*x.1 + c.1)
}

fn depth(c: (f64, f64), n: u32) -> u32 {
	let mut x = c;
	for i in 0..n {
		x = xn(x, c);
		if x.0*x.0 + x.1*x.1 > 9.0 {
			return i;
		}
	}
	n
}

fn main() {
	mm::init();
	let mut win = mm::Window::new(800, 600).unwrap();
	let mut fpscnt = FPSCounter::new();

	let n = 32;
	let (w,h) = win.get_size();
	for ix in 0..w {
		for iy in 0..h {
			let d = depth((4.0*(ix as f64 - 0.5*w as f64)/(h as f64), 4.0*(iy as f64 - 0.5*h as f64)/(h as f64)), n);
			let m = d as f64 / (n - 1) as f64;
			let b: u32 = (255.0*m) as u32;
			win.set_pixel(ix, iy, if d != n { (0xFF<<3*8) | (b<<2*8) | (b<<1*8) | (b<<0*8) } else { 0xFF000000 });
		}
	}
	win.update();

	while !mm::is_closed() {
		mm::sleep(40);
		fpscnt.count();
	}
	mm::quit();
}
