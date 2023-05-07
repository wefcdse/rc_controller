pub fn clear() {
    print!("\x1b[2J");
    print!("\x1b[H");
}

pub trait CanSync {
    fn can_sync() {}
    fn can_sync_i(&self) {}
}
impl<T: Sync> CanSync for T {}

pub trait CanSend {
    fn can_send() {}
    fn can_send_i(&self) {}
}
impl<T: Send> CanSend for T {}
