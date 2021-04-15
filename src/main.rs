extern crate clap;
use opencv::{
	highgui,
	imgcodecs,
	Result,
};
use clap::App;

fn flag()-> (){

}
fn main() -> Result<()> {
	let image = imgcodecs::imread("lena.jpg",imgcodecs::IMREAD_ANYCOLOR)?;
	highgui::named_window("hello opencv!", 0)?;
	highgui::imshow("hello opencv!", &image)?;
	highgui::wait_key(10000)?;
	Ok(())
}
