pub mod types;


pub mod attributes {
    use types::EfivarAttribute;

    pub static NAME:EfivarAttribute = EfivarAttribute::init( "name", 0);
}
