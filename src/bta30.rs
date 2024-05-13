use bitflags::bitflags;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OperationalMode {
    Rx = 0x01,
    Tx = 0x02,
}

const MAGIC_REQUEST: [u8; 3] = [0x00, 0x0a, 0x04];
const MAGIC_REPLY: [u8; 3] = [0x00, 0x0a, 0x84];

pub type Payload = Vec<u8>;

/// All request opcodes consist of a single byte preprended by the 
/// MAGIC_REAQUEST bytes
pub trait Command {}
macro_rules! define_command {
    ($($T:tt);* $(;)?) => {
        $(
            struct $T;
            impl Command for $T {}
        )*
    };
}

fn payload(magic: &[u8], opcode: u8, parameters: Option<&[u8]>) -> Payload {
        let mut payload = vec![];
        payload.extend_from_slice(magic);
        payload.push(opcode);
        if let Some(parameters) = parameters {
            payload.extend_from_slice(parameters);
        }
        payload
} 



pub trait Get: Command {
    fn opcode() -> u8;
    fn request_payload() -> Payload { payload(&MAGIC_REQUEST, Self::opcode(), None) }
}

pub trait ModeGet: Command {
    fn opcode() -> u8;
    fn request_payload(mode: OperationalMode) -> Payload { payload(&MAGIC_REQUEST, Self::opcode(), Some(&[mode as u8])) }
}

pub trait Set: Command {
    type Parameter;

    fn opcode() -> u8;
    fn request_payload_parameters(parameter: &dyn Parameter) -> Vec<u8> {
        parameter.value()
    }

    fn request_payload(parameter: &dyn Parameter) -> Payload { payload(&MAGIC_REQUEST, Self::opcode(), Some(&Self::request_payload_parameters(parameter))) }
}

pub trait ModeSet: Command {
    type Parameter;

    fn opcode() -> u8;


    fn request_payload_parameters(parameter: &dyn Parameter) -> Vec<u8> {
        parameter.value()
    }
    fn request_payload(mode: OperationalMode, parameter: &dyn Parameter) -> Payload {
        // Terrible, terrible code
        let mut tail =  (&Self::request_payload_parameters(parameter)).clone();
        tail.insert(0, mode as u8);
        payload(&MAGIC_REQUEST, Self::opcode(), Some(&tail)) }
}

// Todo: Extensively test those
pub trait ParameterlessSet: Command {
    fn opcode() -> u8;
    fn request_payload(mode: OperationalMode) -> Payload { payload(&MAGIC_REQUEST, Self::opcode(), Some(&[mode as u8])) }
}

macro_rules! get {
    ($($T:ty => $Op:expr);* $(;)?) => {
        $(impl Get for $T { fn opcode() -> u8 { $Op } })*
    };
}

macro_rules! mode_get {
    ($($T:ty => $Op:expr);* $(;)?) => {
        $(impl Get for $T { fn opcode() -> u8 { $Op } })*
    };
}

macro_rules! set {
    ($($T:ty => $Op:expr, $Pr:ty );* $(;)?) => {
        $(impl Set for $T { 
            type Parameter = $Pr;
            fn opcode() -> u8 { $Op } 
        })*
    };
}

macro_rules! mode_set {
    ($($T:ty => $Op:expr, $Pr:ty );* $(;)?) => {
        $(impl ModeSet for $T { 
            type Parameter = $Pr;
            fn opcode() -> u8 { $Op } 

        })*
    };
}

macro_rules! parameterless_set {
    ($($T:ty => $Op:expr);* $(;)?) => {
        $(impl Get for $T { fn opcode() -> u8 { $Op } })*
    };
}

pub trait Toggle: Get + Set {}
macro_rules! impl_toggle {
    ($($T:ty);* $(;)?) => {
        $(impl Toggle for $T {})*        
    };
}

define_command!(
    DeviceVersion;
    CurrentConnectionCodec;
    BootMode;
    LedMode;
    LedPattern;
    TxLdacQualitySetting;
    SpdifVolumeAdjustmentSetting;
    UpsamplingSetting;
    VolumeSetting;
    DacLowpassSetting;
    OutputBalance;
    Name;
    InputSources;
    VolumeMode;
    EnabledCodecs;
    FactoryReset;
    PowerOff;
    ClearPairing;
);

get!(
    DeviceVersion => 0x18;
    CurrentConnectionCodec => 0x16;
    BootMode => 0x1C;
    LedMode => 0x3D;
    LedPattern => 0x4A;
    TxLdacQualitySetting => 0x4C;
    SpdifVolumeAdjustmentSetting => 0x52;
    UpsamplingSetting => 0x50;
    VolumeSetting => 0x12;
    DacLowpassSetting => 0x11;
    OutputBalance => 0x13;
    Name => 0x45;
);

mode_get!(
    InputSources => 0x48;
    VolumeMode => 0x4E;
    EnabledCodecs => 0x17;
);

set!(
    BootMode => 0x0B, StateParameter;
    LedMode => 0x3E, StateParameter;
    LedPattern => 0x4B, LedPatternParameter;
    TxLdacQualitySetting => 0x4D, TxLdacQualitySettingParameter;
    SpdifVolumeAdjustmentSetting => 0x53, StateParameter;
    UpsamplingSetting => 0x51, StateParameter;
    VolumeSetting => 0x02, VolumeSettingParameter;
    DacLowpassSetting => 0x01, DacLowpassSettingParametter;
    OutputBalance => 0x03, OutputBalanceParameter;
    // Name => 0x46; // TODO: Something something very large vector that we chunk, or a custom implementation. Dunno.
);

mode_set!(
    InputSources => 0x49, InputSourcesParameter;
    VolumeMode => 0x4F, VolumeModeParameter;
    EnabledCodecs => 0x07, EnabledCodecsParameter;
);

parameterless_set!(
    FactoryReset => 0x04;
    PowerOff => 0x25;
    ClearPairing => 0x43;
);

impl_toggle!(
    BootMode;
    LedMode;
    LedPattern;
    SpdifVolumeAdjustmentSetting;
    UpsamplingSetting;
);

pub trait Parameter {
    fn value(&self) -> Vec<u8>;
}

impl<P: Copy + Into<u8>> Parameter for P {
    fn value(&self) -> Vec<u8> {
        vec![Into::into(*self)]
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum StateParameter {
    Off = 0x00,
    On = 0x01,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LedPatternParameter {
    RedGreen = 0x00,
    RedBlue = 0x01
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum TxLdacQualitySettingParameter {
    AudioQualityFirst = 0x01,
    Standard = 0x02,
    ConnectionFirst = 0x03,
    Adaptive = 0x04,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VolumeSettingParameter(u8);

impl VolumeSettingParameter {
    pub fn new(volume: u8) -> Result<Self, CommandBuilderError> {
        if volume > 60 {
            return Err(CommandBuilderError::IncorrectVolumeValue)
        }
        
        return Ok(Self(volume))
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DacLowpassSettingParametter {
    SharpRollOffFilter = 0x00,
    SlowRollOffFilter = 0x01,
    ShortDelaySharpRollOffFilter = 0x02,
    ShortDelaySlowRollOffFilter = 0x03,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum OutputBalancePosition {
    Left = 0x01,
    Right = 0x02,
}


#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct OutputBalanceAmout(u8);

impl OutputBalanceAmout {
    pub fn new(amount: u8) -> Result<Self, CommandBuilderError> {
        if amount > 12 {
            return Err(CommandBuilderError::IncorrectOutputBalanceAmount)
        }
        
        return Ok(Self(amount))
    }
}

#[derive(Copy, Clone)]
pub enum OutputBalanceParameter {
    Balanced,
    Unbalanced(OutputBalancePosition, OutputBalanceAmout)
}

impl Parameter for OutputBalanceParameter {
    fn value(&self) -> Vec<u8> {
        match self {
            OutputBalanceParameter::Balanced => vec![0x02, 0x00],
            OutputBalanceParameter::Unbalanced(position, amount) => vec![*position as u8, amount.0], // TODO: There must be a better way to write this line?
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum InputSourcesParameter {
    Usb = 0x01,
    Coax = 0x02,
    UsbAndCoax = 0x03
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum VolumeModeParameter {
    Adjustable = 0x01,
    ThirtyPercent = 0x02,
    FiftyPercent = 0x03,
    SeventyPercent = 0x04,
    HundredPercent = 0x05,
}

bitflags! {
    pub struct EnabledCodecsParameter: u8 {
        const Aac       = 0b00000010;
        const Ldac      = 0b00000100;
        const Aptx      = 0b00001000;
        const AptxLl    = 0b00010100;
        const AptxHd    = 0b00100100;
    }
}

#[derive(Debug)]
pub enum CommandBuilderError {
    /// Returned when requested volume is superior to 60
    IncorrectVolumeValue,

    /// Returned when requested volume balance is superior to 12
    IncorrectOutputBalanceAmount
}
