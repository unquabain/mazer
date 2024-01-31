use base64::prelude::{Engine as _, BASE64_URL_SAFE};
use std::fmt;
use urlencoding;

#[derive(Debug)]
pub struct MazeParams {
    pub width: usize,
    pub height: usize,
    pub seed: [u8; 32],
    pub solution: bool,
}

impl MazeParams {
    pub fn write_seed(&mut self, b64: &str) -> Result<(), String> {
        let b64 = urlencoding::decode(b64).unwrap();
        let input: &[u8] = b64.as_bytes();
        let mut buff: [u8; 64] = [0; 64];
        if let Err(err) = &BASE64_URL_SAFE.decode_slice(input, &mut buff) {
            return Err(format!("could not parse seed: {:?}", err));
        }
        self.seed.clone_from_slice(&buff[..32]);

        Ok(())
    }
}

impl fmt::Display for MazeParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b64seed = &BASE64_URL_SAFE.encode(self.seed);
        write!(f, "?width={}&height={}&seed={}",  self.width, self.height, urlencoding::encode(b64seed))
    }
}

impl Default for MazeParams {
    fn default() -> Self {
        Self{
            width: 60,
            height: 40,
            seed: rand::random::<[u8; 32]>(),
            solution: false,
        }
    }
}

