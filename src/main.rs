extern crate mm;

struct FPSCounter<'a> {
	ctx: &'a mm::Context,
	start: u32,
	frames: u32,
	pub fps: f64,
}

impl<'a> FPSCounter<'a> {
	pub fn new(ctx: &mm::Context) -> FPSCounter {
		FPSCounter { ctx: ctx, start: ctx.get_ticks(), frames: 0, fps: 0.0 }
	}
	pub fn count(&mut self) {
		self.frames += 1;
		let ticks = self.ctx.get_ticks();
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
	let ctx = mm::Context::acquire().unwrap();

	let mut win = mm::Window::new(&ctx, 800, 600).unwrap();

	let mut fpscnt = FPSCounter::new(&ctx);

	let mut i = 0;
	while !ctx.is_closed() {
		
		let (w,h) = win.get_size();
		for ix in 0..w {
			for iy in 0..h {
				win.set_pixel(ix, iy, i | (0xFF << 3*8));
			}
		}
		win.update();
		
		
		ctx.sleep(40);
		fpscnt.count();
		i += 1;
	}
}
