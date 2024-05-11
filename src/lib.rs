#[macro_use]
extern crate lazy_static;

use std::{collections::BTreeSet, pin::Pin, str::FromStr};

use btleplug::api::{CharPropFlags, Characteristic, Peripheral, WriteType};
use uuid::Uuid;

// let led_off_command: [u8; 5] = [0x00, 0x0a, 0x04, 0x3e, 0x00];
// let led_on_command: [u8; 5] = [0x00, 0x0a, 0x04, 0x3e, 0x01];
// let boot_mode_on_command: [u8; 5] = [0x00, 0x0a, 0x04, 0x0b, 0x01];
// let power_off_command: [u8; 5] = [0x00, 0x0a, 0x84, 0x25, 0x00];

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


pub struct Command {
    payload: Vec<u8>,
    write_type: WriteType
}

pub fn send_command <'a, P> (peripheral: &'a P, command: &'a Command) -> Pin<Box<dyn std::future::Future<Output = Result<(), btleplug::Error>> + Send + 'a>>
    where P: Peripheral + 'a {
    peripheral.write(&CMD_CHAR, &command.payload, command.write_type)
}

#[derive(Debug)]
pub enum CommandBuilderError {
    /// Returned when requested volume is superior to 60
    IncorrectVolumeValue
}

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

pub fn set_volume_command(volume: u8) -> Result<Command, CommandBuilderError> {
    if volume > 60 {
        return Err(CommandBuilderError::IncorrectVolumeValue)
    }

    Ok(Command {
        payload: vec![0x00, 0x0a, 0x04, 0x02, volume],
        write_type: WriteType::WithResponse
    })
}

pub fn set_volume_mode_setting(mode: OperationalMode, vms: VolumeModeSetting) -> Result<Command, CommandBuilderError> {
    Ok(Command {
        payload: vec![0x00, 0x0a, 0x04, 0x4f, mode as u8, vms as u8],
        write_type: WriteType::WithResponse
    })
}