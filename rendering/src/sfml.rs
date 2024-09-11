use sfml::window::Event;
use sfml::window::VideoMode;
use sfml::graphics::{CircleShape, Color, Shape, Transformable};
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use sfml::system::Vector2f;

pub struct SFMLRenderer {
    window: RenderWindow,
}

impl SFMLRenderer {
    pub fn new() -> Self {
        let video_mode = VideoMode::new(
            crate::tunables::SIMULATION_WIDTH as u32,
            crate::tunables::SIMULATION_HEIGHT as u32,
            Self::get_desktop_bits_per_pixel(),
        );

        Self {
            window: Self::create_window(video_mode),
        }
    }

    pub fn enter_graphics_loop(&mut self) {
        while self.window.is_open() {
            Self::prepare_window(&mut self.window);

            while let Some(event) = self.window.poll_event() {
                if event == Event::Closed { self.window.close() }
            }
            
            if let Some(simulation) = unsafe { crate::SIMULATION.as_ref() } {
                for entity in &simulation.entities {
                    if let molesim_core::entities::Entity::Circle(circle) = entity {
                        let mut c = CircleShape::new(5.0, 30);
                        c.set_position(
                            Vector2f::new(
                                circle.kinematics().position().x as f32,
                                (crate::tunables::SIMULATION_HEIGHT - circle.kinematics().position().y) as f32,
                            )
                        );
                        c.set_fill_color(Color::BLACK);
                        
                        self.window.draw(
                            &c
                        )
                    }
                }
            }

            Self::render_window(&mut self.window);
        }
    }

    fn prepare_window(window: &mut RenderWindow) {
        window.clear(Color::WHITE);
    }

    fn render_window(window: &mut RenderWindow) {
        window.display();
    }

    fn create_window(video_mode: VideoMode) -> RenderWindow {
        let mut window = RenderWindow::new(
            video_mode,
            "MoleSim",
            sfml::window::Style::CLOSE,
            &sfml::window::ContextSettings::default(),
        );

        window.clear(Color::WHITE);
        window.display();

        window
    }


    fn get_desktop_bits_per_pixel() -> u32 {
        VideoMode::desktop_mode().bits_per_pixel
    }
}
