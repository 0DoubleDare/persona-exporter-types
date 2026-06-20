
pub enum DataUnit {
    Bytes = 1,
    KiloBytes = 1024,
    MegaBytes = 1024 * 1024,
    GigaBytes = 1024 * 1024 * 1024,
    TeraBytes = 1024 * 1024 * 1024 * 1024,
    PetaBytes = 1024 * 1024 * 1024 * 1024 * 1024,
}
pub trait Converter {
    fn get_bytes(&self) -> u64;


}