#![forbid(unsafe_code)]
#[macro_use]
extern crate clap;
use opencv::{highgui, imgcodecs, Result};

use clap::App;

// This macro supports visibility-specifiers, like `build_info!(pub fn how_this_crate_was_built)`.
build_info::build_info!(fn build_info);

fn show_pic(filename: &str) -> Result<()> {
    let image = imgcodecs::imread(filename, imgcodecs::IMREAD_ANYCOLOR)?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &image)?;
    highgui::wait_key(10000)?;
    Ok(())
}
#[allow(dead_code)]
fn flag() -> () {
    let s = "\r\nVersion:".to_owned()
        + &crate_version!().to_owned()
        + "\r\ngit:"
        + &crate_description!().to_owned();
    let matches = App::new("imgshow")
        .version(&*s)
        .author(crate_authors!())
        .about(
            "查看图片的工具.
	具体看帮助:./imgshow --help
	",
        )
        // .args_from_usage("-c,--createdb '创建数据库'")
        // .args_from_usage("-s,--show=[NAME] '显示某个name的信息'")
        // .args_from_usage("-i,--insert '插入数据库'")
        .args_from_usage("-i,--img=[IMG] '图片文件名'")
        .get_matches();

    if matches.is_present("version") {
        use std::process;
        process::exit(0x0100);
    }
    if matches.is_present("img") {
        let filename = matches.value_of("img").unwrap_or("");
        // println!{"env:{:#?}",env}
        if filename.len() == 0 {
            println!("文件名不能为空");
        } else {
            let _r = show_pic(filename);
        }
    }
}
fn main() {
    println!("{:#?}", build_info());

    // ... or format it directly to a single `&'static str` at compile time
//    println!(
//        "{}",
//        build_info::format!("{{{} v{} built with {} at {}}}", $.crate_info.name, $.crate_info.version, $.compiler, $.timestamp)
//    );
    flag();
}
