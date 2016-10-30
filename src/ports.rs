pub enum PortName {
    A,
    B,
    C,
    D,
    E
}

#[repr(C,packed)]
pub struct Port {
    pub pcr: [u32; 32],
    pub gplcr: u32,
    pub gpchr: u32,
    reserved_0: [u8; 24],
    pub isfr: u32,
    reserved_1: [u8; 28],
    pub dfer: u32,
    pub dfcr: u32,
    pub dfwr: u32
}

impl Port {
    pub unsafe fn port(name: PortName) -> *mut Port {
        match name {
            PortName::A => 0x40049000 as *mut Port,
            PortName::B => 0x4004A000 as *mut Port,
            PortName::C => 0x4004B000 as *mut Port,
            PortName::D => 0x4004C000 as *mut Port,
            PortName::E => 0x4004D000 as *mut Port
        }
    }
}

#[repr(C,packed)]
pub struct Gpio {
    pub pdor: u32,
    pub psor: u32,
    pub pcor: u32,
    pub ptor: u32,
    pub pdir: u32,
    pub pddr: u32
}

impl Gpio {
    pub unsafe fn port(name:PortName) -> *mut Gpio {
        match name {
            PortName::A => 0x400FF000 as *mut Gpio,
            PortName::B => 0x400FF040 as *mut Gpio,
            PortName::C => 0x400FF080 as *mut Gpio,
            PortName::D => 0x400FF0C0 as *mut Gpio,
            PortName::E => 0x400FF100 as *mut Gpio
        }
    }
}
