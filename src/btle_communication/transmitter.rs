use crate::core::DeviceCommand;

pub trait Transmitter {
    async fn close(&mut self);

    async fn send_raw(&self, bytes: &[u8]);

    async fn send(&self, cmd: &DeviceCommand) {
        self.send_raw(&cmd.bytes).await
    }

    async fn send_all(&self, cmds: &[DeviceCommand]) {
        for c in cmds {
            self.send(c).await;
        }
    }
}
