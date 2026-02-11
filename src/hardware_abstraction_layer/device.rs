use crate::core::DeviceCompiler;
use crate::core::LightState;
use btleplug::api::Peripheral;
use btleplug::api::WriteType;
use btleplug::api::bleuuid::uuid_from_u16;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum DeviceKind {
    KS03Old,
    KS03New,
}

impl DeviceKind {
    pub fn from_name(name: &str) -> Option<Self> {
        if name.starts_with("KS03-") {
            Some(Self::KS03Old)
        } else if name.starts_with("KS03~") {
            Some(Self::KS03New)
        } else {
            None
        }
    }
}

pub struct DeviceProfile {
    pub write_service: Uuid,
    pub write_characteristic: Uuid,
    pub notify_service: Uuid,
    pub notify_characteristic: Uuid,
    pub read_service: Uuid,
    pub read_characteristic: Uuid,
}

impl DeviceProfile {
    pub fn from_kind(kind: DeviceKind) -> Self {
        match kind {
            DeviceKind::KS03New => Self {
                write_service: uuid_from_u16(0xAFD0),
                write_characteristic: uuid_from_u16(0xAFD1),
                notify_service: uuid_from_u16(0xAFD0),
                notify_characteristic: uuid_from_u16(0xAFD2),
                read_service: uuid_from_u16(0xAFD0),
                read_characteristic: uuid_from_u16(0xAFD3),
            },
            DeviceKind::KS03Old => Self {
                write_service: uuid_from_u16(0xFFF0),
                write_characteristic: uuid_from_u16(0xFFF3),
                notify_service: uuid_from_u16(0xFFF0),
                notify_characteristic: uuid_from_u16(0xFFF3),
                read_service: uuid_from_u16(0xFFF0),
                read_characteristic: uuid_from_u16(0xFFF3),
            },
        }
    }
}

pub struct SupportedDevice {
    pub name: String,
    pub kind: DeviceKind,
    pub peripheral: btleplug::platform::Peripheral,
    pub device_profile: DeviceProfile,
}

impl SupportedDevice {
    pub async fn send_commands(&self, state: &LightState) -> anyhow::Result<()> {
        let cmds = self.kind.compile(state);

        self.peripheral.discover_services().await?;

        let chars = self.peripheral.characteristics();
        let cmd_char = chars
            .iter()
            .find(|ch| ch.uuid == self.device_profile.write_characteristic)
            .expect("Unable to find write characteristic");

        for cmd in cmds {
            self.peripheral
                .write(cmd_char, &cmd.bytes, WriteType::WithoutResponse)
                .await?;
            time::sleep(Duration::from_millis(200)).await;
        }

        Ok(())
    }
}
