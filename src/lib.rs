#[macro_use]
extern crate lazy_static;

pub mod bta30;

use std::{collections::BTreeSet, pin::Pin, str::FromStr};
use std::convert::Into;

use btleplug::api::{CharPropFlags, Characteristic, WriteType};
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
