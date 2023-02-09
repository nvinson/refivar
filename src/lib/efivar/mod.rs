pub mod types;


pub mod attributes {
    use types::EfivarAttribute;

    pub static NON_VOLATILE:EfivarAttribute
        = EfivarAttribute::init("Non Volatile", 0x1);

    pub static BOOTSERVICE_ACCESS:EfivarAttribute
        = EfivarAttribute::init("Bootservice Access", 0x2);

    pub static RUNTIME_ACCESS:EfivarAttribute
        = EfivarAttribute::init("Runtime Access", 0x4);

    pub static HARDWARE_ERROR_RECORD:EfivarAttribute
        = EfivarAttribute::init("Hardware Error Record", 0x8);

    pub static AUTHENTICATED_WRITE_ACCESS:EfivarAttribute
        = EfivarAttribute::init("Authenticated Write Access", 0x10);

    pub static TIME_BASED_AUTHENTICATED_WRITE_ACCESS:EfivarAttribute
        = EfivarAttribute::init("Time Based Authenticated Write Access", 0x20);

    pub static APPEND_WRITE:EfivarAttribute
        = EfivarAttribute::init("Append Write", 0x40);

    pub static ENHANCED_AUTHENTICATED_ACCESS:EfivarAttribute
        = EfivarAttribute::init("Enhanced Authenticated Access", 0x80);
}
