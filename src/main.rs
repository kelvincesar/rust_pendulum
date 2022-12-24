use std::rc::Rc;

use space_vector::SpaceVector;

use speedy2d::color::Color;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    println!("Renderizando pendulo simples");
    
    // Cria janela
    let window = Window::new_centered("Pendulo Simples",(800, 600)).unwrap();
    

    // Cria handler dos pendulos
    let mut handler = MyWindowHandler::new();
    // Cria pendulos
    let pendulum = Pendulum::new(400.0, 0.0, 400.0);
    let pendulum_2 = Pendulum::new(400.0, 0.0, 200.0);

    // Adiciona os pendulos ao handler
    handler.add(pendulum);
    handler.add(pendulum_2);
    
    // Chama loop de renderização
    window.run_loop(handler)
}

// Gerenciador da janela gráfica
struct MyWindowHandler {
    penduluns: Vec<Pendulum>,
    font: Font
}

impl MyWindowHandler {
    fn new() -> Self {
        let font = Font::new(include_bytes!("../assets/economica-regular.ttf")).unwrap();
        
        Self {
            penduluns: Vec::new(),
            font: font 
        }
    }
    fn add(&mut self, pendulum: Pendulum) {
        self.penduluns.push(pendulum);
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
            &mut self,
            helper: &mut WindowHelper<()>,
            graphics: &mut Graphics2D
        ) {
        // Limpa a tela
        graphics.clear_screen(Color::WHITE);
        
        let mut text_pos = (10.0, 10.0);

        // iterate over pendulums returning it structure and an index
 

        // itera pelos pendulos
        for (i, pendulum) in self.penduluns.iter_mut().enumerate() {
            // Atualiza o pendulo
            pendulum.update();

            // Desenha o pendulo
            pendulum.draw(graphics, Color::BLUE);

            let text = format!("Pêndulo {} - Vel. angular: {:.2} rad/s", i, pendulum.angular_velocity);
            // Redenriza texto com a aceleração
            let data = self.font.layout_text(&text, 24.0, TextOptions::new());

            text_pos = (text_pos.0, text_pos.1 + 30.0);
            // desenha texto logo abaixo do pendulo
            graphics.draw_text(
                (text_pos.0, text_pos.1), 
                Color::BLACK, 
                &data
            );

        }




        // Desenha o frame
        helper.request_redraw();
        
    }
}

struct Pendulum {
    // Offset da posição do pendulo
    origin: SpaceVector,

    // Posição da bola do pendulo
    position: SpaceVector,
    
    angle: f32,
    angular_velocity: f32,  // Velocidade angular
    angular_acceleration: f32,  // Aceleração angular
    r: f32, // Raio do pendulo
    g: f32, // Aceleração da gravidade
}

impl Pendulum {
    /// Creates a new [`Pendulum`].
    fn new(x: f32, y: f32, r: f32) -> Self {
        Self {
            origin: SpaceVector::new(x, y),
            position: SpaceVector::new(x, y),
            angle: 1.0,                 // Ângulo inicial
            angular_velocity: 0.0,      // Velocidade estacionaria
            angular_acceleration: 0.0,  // Não está acelerando...
            r,
            g: 1.0,
        }
    }

    fn update(&mut self){
        // Cálculo da aceleração angular
        self.angular_acceleration = -self.g * self.angle.sin() / self.r;

        // Cálculo da velocidade angular
        self.angular_velocity += self.angular_acceleration;

        // Cálculo do ângulo
        self.angle += self.angular_velocity;

        // Posição do pendulo convertendo a cordenada polar para retangular
        self.position.set(
            self.r * self.angle.sin(),
            self.r * self.angle.cos()
        );
        
        // Aplica offset para origem
        self.position.add(&self.origin); 
    }

    fn draw(&self, graphics: &mut Graphics2D, color: Color){
        // Cria haste do pendulo
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            2.0,
            Color::GREEN,
        );
        
        // Cria a bolinha do pendulo
        graphics.draw_circle(
            (self.position.x, self.position.y),
            10.0,
            color,
        );
    }
}

mod space_vector {
    pub struct SpaceVector {
        pub x: f32,
        pub y: f32,
    }

    impl SpaceVector {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub fn add(&mut self, vector: &SpaceVector) -> &SpaceVector {
            self.x += vector.x;
            self.y += vector.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}