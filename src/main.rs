use std::time::Instant;

use rc_controller::{simple_loader::simple_loader, util::clear};
fn main() {
    let mut c = simple_loader(10.0);
    loop {
        let t = Instant::now();
        c.update().unwrap();
        clear();
        println!("{:?}", t.elapsed());
        println!("{:?}", c.get_typr());
    }
}
