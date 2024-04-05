extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let curr_content = ctx.get_contents().unwrap();
    println!("{:?}", curr_content);
    ctx.set_contents(String::from("Hello, world!")).unwrap();
    let curr_content = ctx.get_contents().unwrap();
    println!("{:?}", curr_content);
}
