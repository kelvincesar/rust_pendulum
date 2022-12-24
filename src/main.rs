use space_vector::SpaceVector;

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    println!("Renderizando pendulo simples");
    
    // Cria janela
    let window = Window::new_centered("Pendulo Simples",(800, 600)).unwrap();
    
    // Cria hanler para o desenho
    let handler = MyWindowHandler {
        pendulum: Pendulum::new(400.0, 0.0, 200.0),
    };
    // Chama loop de renderização
    window.run_loop(handler)
}

// Gerenciador da janela gráfica
struct MyWindowHandler {
    pendulum: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(
            &mut self,
            helper: &mut WindowHelper<()>,
            graphics: &mut Graphics2D
        ) {
        // Limpa a tela
        graphics.clear_screen(Color::WHITE);

        // Atualiza o pendulo
        self.pendulum.update();

        // Desenha o pendulo
        self.pendulum.draw(graphics);

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

    fn draw(&self, graphics: &mut Graphics2D){
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
            Color::BLACK,
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