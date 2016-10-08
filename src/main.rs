extern crate mm;

fn main() {
	mm::init();
	while mm::handle() == 0 {
	    mm::sleep(40);
	}
	mm::quit();
}
