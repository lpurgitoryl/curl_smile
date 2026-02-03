use crate::core::DeviceCommand;

pub trait Transmitter {
    fn close(&self) -> ();
    fn send_raw(&self, bytes: &[u8]) -> ();

    fn send(&self, cmd: &DeviceCommand) -> () {
        self.send_raw(&cmd.bytes)
    }

    fn send_all(&self, cmds: &[DeviceCommand]) -> () {
        for c in cmds {
            self.send(c);
        }
    }
}
