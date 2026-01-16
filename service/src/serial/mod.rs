use tokio_serial::SerialPort;

pub struct Serial {
    pub port: Box<dyn SerialPort>,
}

impl Serial {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.port.write(data)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buffer)
    }
}
