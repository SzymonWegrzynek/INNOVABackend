use base64::{engine::general_purpose, Engine};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub struct StockImage {
    pub data: Vec<u8>,
}

impl StockImage {
    pub fn get_stock_image(file_path: &str) -> Self {
        let path = Path::new(file_path);
        let file = File::open(path).expect("Błąd: Nie można otworzyć pliku.");
        let mut reader = BufReader::new(file);
        let mut storage: Vec<u8> = Vec::new();
        reader.read_to_end(&mut storage).unwrap();

        StockImage { data: storage }
    }

    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.data)
    }
}
