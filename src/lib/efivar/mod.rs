pub mod types;

pub mod attributes {
    use types::EfiVariableAttribute;

    pub static NON_VOLATILE:EfiVariableAttribute
        = EfiVariableAttribute::init("Non Volatile", 0x1);

    pub static BOOTSERVICE_ACCESS:EfiVariableAttribute
        = EfiVariableAttribute::init("Bootservice Access", 0x2);

    pub static RUNTIME_ACCESS:EfiVariableAttribute
        = EfiVariableAttribute::init("Runtime Access", 0x4);

    pub static HARDWARE_ERROR_RECORD:EfiVariableAttribute
        = EfiVariableAttribute::init("Hardware Error Record", 0x8);

    pub static AUTHENTICATED_WRITE_ACCESS:EfiVariableAttribute
        = EfiVariableAttribute::init("Authenticated Write Access", 0x10);

    pub static TIME_BASED_AUTHENTICATED_WRITE_ACCESS:EfiVariableAttribute
        = EfiVariableAttribute::init("Time-Based Authenticated Write Access", 0x20);

    pub static APPEND_WRITE:EfiVariableAttribute
        = EfiVariableAttribute::init("Append Write", 0x40);

    pub static ENHANCED_AUTHENTICATED_ACCESS:EfiVariableAttribute
        = EfiVariableAttribute::init("Enhanced Authenticated Access", 0x80);
}
