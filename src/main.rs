use dundefsave::crc;

fn main() {
    let table = crc::table::CRCLookupTable::new();
    println!("{:02X?}", table);
}
