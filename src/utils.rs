#[derive(Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub const BLACK: Self = Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Pixel {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
}

pub struct FrameBuffer {
    width: u32,
    height: u32,

    pixels: Vec<Pixel>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::BLACK; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: Pixel) {
        let i = x + y * self.width as usize;
        self.pixels[i] = value;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.pixels.len() * 3);
        for p in self.pixels.iter() {
            bytes.push(p.r);
            bytes.push(p.g);
            bytes.push(p.b);
        }
        bytes
    }
}
