pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,  // El buffer que contiene los píxeles
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    // Constructor para Framebuffer
    pub fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height;
        println!("Inicializando Framebuffer con tamaño de buffer: {}", buffer_size);

        Framebuffer {
            width,
            height,
            buffer: vec![0; buffer_size],  // Cada píxel es un valor u32
            background_color: 0x000000,    // Color de fondo predeterminado (negro)
            current_color: 0xFFFFFF,       // Color actual predeterminado (blanco)
        }
    }

    // Método para establecer el color actual
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Método para obtener el tamaño actual del buffer
    pub fn get_buffer_size(&self) -> usize {
        self.buffer.len()
    }

    // Método para limpiar el buffer
    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        let buffer_size = self.get_buffer_size();
        let index = y * self.width + x;
    
        // Validar que no estamos accediendo fuera del buffer
        if x < self.width && y < self.height && buffer_size > index {
            self.buffer[index] = self.current_color;
        } else {
            panic!("Acceso inválido al buffer: x: {}, y: {}, index: {}, width: {}, height: {}", x, y, index, self.width, self.height);
        }
    }
    
}
