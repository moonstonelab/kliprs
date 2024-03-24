extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        println!("{:?}", ctx.get_contents());
        ctx.set_contents("some string".to_owned()).unwrap();
        println!("{:?}", ctx.get_contents());
    }
}