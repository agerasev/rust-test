#[macro_use(color)]
extern crate mm;
use mm::Color;


fn main() {
	let mut i = 0;
	mm::init(800, 600);
	while mm::handle() == 0 {
		let r = (1 + i%100) as f32;
		let g = 100 as f32;
		let b = (100 - i%100) as f32;
		mm::set_color(color!(0.01*r, 0.01*g, 0.01*b));
		let (w,h) = mm::get_size();
		for ix in 0..w {
			for iy in 0..h {
				mm::draw_point(ix, iy);
			}
		}
		mm::update();
		// mm::sleep(40);
		i += 1;
	}
	mm::quit();
}
