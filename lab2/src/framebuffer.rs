use raylib::prelude::*;

pub struct Framebuffer {
    pub color_buffer: Image,
    pub width: u32,
    pub height: u32,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Self {
            color_buffer,
            width,
            height,
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        // Verificar límites antes de pintar
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, color);
        }
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            // Obtener las dimensiones de la ventana antes de comenzar el dibujo
            let screen_width = window.get_screen_width();
            let screen_height = window.get_screen_height();
            
            let mut d = window.begin_drawing(thread);
            d.clear_background(Color::BLACK);
            
            // Calcular el escalado para que la textura llene la ventana
            let scale_x = screen_width as f32 / self.width as f32;
            let scale_y = screen_height as f32 / self.height as f32;
            let scale = scale_x.min(scale_y); // Usar el escalado más pequeño para mantener proporción
            
            let scaled_width = self.width as f32 * scale;
            let scaled_height = self.height as f32 * scale;
            let x_offset = (screen_width as f32 - scaled_width) / 2.0;
            let y_offset = (screen_height as f32 - scaled_height) / 2.0;
            
            d.draw_texture_ex(
                &texture,
                Vector2::new(x_offset, y_offset),
                0.0,
                scale,
                Color::WHITE,
            );
        }
    }
}
