#[macro_use]
extern crate lazy_static;

use std::{collections::BTreeSet, pin::Pin, str::FromStr};

use btleplug::api::{CharPropFlags, Characteristic, Peripheral, WriteType};
use uuid::Uuid;

lazy_static! {
    static ref CSR_GAIA_SERVICE_UUID: Uuid = Uuid::from_str("00001100-d102-11e1-9b23-00025b00a5a5").unwrap();
    static ref CSR_GAIA_COMMAND_ENDPOINT_UUID: Uuid = Uuid::from_str("00001101-d102-11e1-9b23-00025b00a5a5").unwrap();
    static ref CSR_GAIA_RESPONSE_ENDPOINT_UUID: Uuid = Uuid::from_str("00001102-d102-11e1-9b23-00025b00a5a5").unwrap();

    static ref CMD_CHAR: Characteristic = Characteristic {
        uuid: *CSR_GAIA_COMMAND_ENDPOINT_UUID,
        service_uuid: *CSR_GAIA_SERVICE_UUID,
        properties: CharPropFlags::from_name("WRITE").unwrap(),
        descriptors: BTreeSet::new(),
    };
}

#[derive(Copy, Clone)]
enum CommandKind {
    Request,
    Command,
}

impl Into<WriteType> for CommandKind {
    fn into(self) -> WriteType {
        match self {
            CommandKind::Request => WriteType::WithResponse,
            CommandKind::Command => WriteType::WithoutResponse,
        }
    }
}

pub struct Command {
    kind: CommandKind,
    payload: Vec<u8>,
}

pub fn send_command <'a, P> (peripheral: &'a P, command: &'a Command) -> Pin<Box<dyn std::future::Future<Output = Result<(), btleplug::Error>> + Send + 'a>>
    where P: Peripheral + 'a {
    peripheral.write(&CMD_CHAR, &command.payload, command.kind.into())
}

#[derive(Debug)]
pub enum CommandBuilderError {
    /// Returned when requested volume is superior to 60
    IncorrectVolumeValue
}

type CommandResult = Result<Command, CommandBuilderError>;

#[repr(u8)]
pub enum OperationalMode {
    Rx = 0x01,
    Tx = 0x02,
}

#[repr(u8)]
pub enum VolumeModeSetting {
    Adjustable = 0x01,
    ThirtyPercent = 0x02,
    FiftyPercent = 0x03,
    SeventyPercent = 0x04,
    HundredPercent = 0x05,
}

#[repr(u8)]
pub enum LedIndicator {
    Off = 0x00,
    On = 0x01,
}

#[repr(u8)]
pub enum BootOnPowerOn {
    Off = 0x00,
    On = 0x01,
}

pub fn set_volume_command(volume: u8) -> CommandResult {
    if volume > 60 {
        return Err(CommandBuilderError::IncorrectVolumeValue)
    }

    Ok(Command {
        payload: vec![0x00, 0x0a, 0x04, 0x02, volume],
        kind: CommandKind::Request
    })
}

pub fn set_volume_mode_setting_command(mode: OperationalMode, vms: VolumeModeSetting) -> CommandResult {
    Ok(Command {
        payload: vec![0x00, 0x0a, 0x04, 0x4f, mode as u8, vms as u8],
        kind: CommandKind::Request
    })
}

pub fn set_led_indicator_command(led_indicator: LedIndicator) -> CommandResult {
    Ok(
        Command {
            payload: vec![0x00, 0x0a, 0x04, 0x3e, led_indicator as u8],
            kind: CommandKind::Request
        }
    )
}

pub fn set_boot_mode_command(boot_mode: BootOnPowerOn) -> CommandResult {
    Ok(
        Command {
            payload: vec![0x00, 0x0a, 0x04, 0x0b, boot_mode as u8],
            kind: CommandKind::Request
        }
    )
}

pub fn power_off_commqnd() -> CommandResult {
    Ok(
        Command {
            payload: vec![0x00, 0x0a, 0x84, 0x25, 0x00],
            kind: CommandKind::Command
        }
    )
}
