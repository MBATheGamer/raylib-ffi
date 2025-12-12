use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_random_value, init_window,
    keyboard::is_key_pressed,
    mouse::{get_mouse_position, is_mouse_button_down},
    set_target_fps, window_should_close,
  },
  enums::{KeyboardKey, MouseButton},
  shape::{draw_circle_v, draw_rectangle, draw_rectangle_lines},
  structs::{Color, Vector2},
  text::{draw_fps, draw_text},
  texture::fade,
};

#[repr(i32)]
#[derive(Clone, Copy, PartialEq)]
enum ParticleType {
  Water = 0,
  Smoke,
  Fire,
}

impl Default for ParticleType {
  fn default() -> Self {
    return Self::Water;
  }
}

const MAX_PARTICLES: usize = 3000;
const PARTICLE_TYPE_NAMES: [&'static str; 3] = ["Water", "Smoke", "Fire"];

#[derive(Default)]
struct Particle {
  particle_type: ParticleType,
  position: Vector2,
  velocity: Vector2,
  radius: f32,
  color: Color,

  life_time: f32,
  alive: bool,
}

struct CircularBuffer {
  head: usize,
  tail: usize,
  buffer: Vec<Particle>,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - simple particles",
  );

  let mut particles: Vec<Particle> = vec![];

  for _ in 0..MAX_PARTICLES {
    particles.push(Particle::default());
  }

  let mut circular_buffer = CircularBuffer {
    head: 0,
    tail: 0,
    buffer: particles,
  };

  let mut emission_rate = -2;
  let mut current_type = ParticleType::Water;
  let mut emitter_position = Vector2 {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
  };

  set_target_fps(60);

  while !window_should_close() {
    if emission_rate < 0 {
      if get_random_value(0, -emission_rate) == 0 {
        emit_particle(&mut circular_buffer, emitter_position, current_type);
      }
    } else {
      for _ in 0..emission_rate {
        emit_particle(&mut circular_buffer, emitter_position, current_type);
      }
    }

    update_particles(&mut circular_buffer, SCREEN_WIDTH, SCREEN_HEIGHT);

    update_circular_buffer(&mut circular_buffer);

    if is_key_pressed(KeyboardKey::KeyUp) {
      emission_rate += 1;
    }
    if is_key_pressed(KeyboardKey::KeyDown) {
      emission_rate -= 1;
    }

    if is_key_pressed(KeyboardKey::KeyRight) {
      current_type = match current_type {
        ParticleType::Fire => ParticleType::Water,
        ParticleType::Water => ParticleType::Smoke,
        ParticleType::Smoke => ParticleType::Fire,
      };
    }
    if is_key_pressed(KeyboardKey::KeyLeft) {
      current_type = match current_type {
        ParticleType::Water => ParticleType::Fire,
        ParticleType::Fire => ParticleType::Smoke,
        ParticleType::Smoke => ParticleType::Water,
      };
    }

    if is_mouse_button_down(MouseButton::Left) {
      emitter_position = get_mouse_position();
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_particles(&circular_buffer);

    draw_rectangle(5, 5, 315, 75, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(5, 5, 315, 75, colors::BLUE);

    draw_text("CONTROLS:", 15, 15, 10, colors::BLACK);
    draw_text(
      "UP/DOWN: Change Particle Emission Rate",
      15,
      35,
      10,
      colors::BLACK,
    );
    draw_text(
      "LEFT/RIGHT: Change Particle Type (Water, Smoke, Fire)",
      15,
      55,
      10,
      colors::BLACK,
    );

    if emission_rate < 0 {
      draw_text(
        &format!(
          "Particles every {} frames | Type: {}",
          -emission_rate, PARTICLE_TYPE_NAMES[current_type as usize]
        ),
        15,
        95,
        10,
        colors::DARKGRAY,
      );
    } else {
      draw_text(
        &format!(
          "{} Particles per frame | Type: {}",
          emission_rate + 1,
          PARTICLE_TYPE_NAMES[current_type as usize]
        ),
        15,
        95,
        10,
        colors::DARKGRAY,
      );
    }

    draw_fps(SCREEN_WIDTH - 80, 10);

    end_drawing();
  }

  close_window();
}

fn emit_particle(
  circular_buffer: &mut CircularBuffer,
  emitter_position: Vector2,
  particle_type: ParticleType,
) {
  let mut new_particle = add_to_circular_buffer(circular_buffer);

  if let Some(particle) = &mut new_particle {
    particle.position = emitter_position;
    particle.alive = true;
    particle.life_time = 0.0;
    particle.particle_type = particle_type;
    let mut speed = get_random_value(0, 10) as f32 / 5.0;
    match particle_type {
      ParticleType::Water => {
        particle.radius = 5.0;
        particle.color = colors::BLUE;
      }
      ParticleType::Smoke => {
        particle.radius = 7.0;
        particle.color = colors::GRAY;
      }
      ParticleType::Fire => {
        particle.radius = 10.0;
        particle.color = colors::YELLOW;
        speed /= 10.0;
      }
    }

    let direction = get_random_value(0, 360) as f32;
    particle.velocity = Vector2 {
      x: speed * direction.to_radians().cos(),
      y: speed * direction.to_radians().sin(),
    };
  }
}

fn add_to_circular_buffer(circular_buffer: &mut CircularBuffer) -> Option<&mut Particle> {
  let mut particle: Option<&mut Particle> = None;

  if ((circular_buffer.head + 1) % MAX_PARTICLES) != circular_buffer.tail {
    particle = Some(&mut circular_buffer.buffer[circular_buffer.head]);
    circular_buffer.head = (circular_buffer.head + 1) % MAX_PARTICLES;
  }

  return particle;
}

fn update_particles(circular_buffer: &mut CircularBuffer, screen_width: i32, screen_height: i32) {
  let mut i = circular_buffer.tail;
  while i != circular_buffer.head {
    circular_buffer.buffer[i].life_time += 1.0 / 60.0;

    match circular_buffer.buffer[i].particle_type {
      ParticleType::Water => {
        circular_buffer.buffer[i].position.x += circular_buffer.buffer[i].velocity.x;
        circular_buffer.buffer[i].velocity.y += 0.2;
        circular_buffer.buffer[i].position.y += circular_buffer.buffer[i].velocity.y;
      }
      ParticleType::Smoke => {
        circular_buffer.buffer[i].position.x += circular_buffer.buffer[i].velocity.x;
        circular_buffer.buffer[i].velocity.y -= 0.05;
        circular_buffer.buffer[i].position.y += circular_buffer.buffer[i].velocity.y;
        circular_buffer.buffer[i].radius += 0.5;
        circular_buffer.buffer[i].color.alpha -= 4;

        if circular_buffer.buffer[i].color.alpha < 4 {
          circular_buffer.buffer[i].alive = false;
        }
      }
      ParticleType::Fire => {
        circular_buffer.buffer[i].position.x += circular_buffer.buffer[i].velocity.x
          + (circular_buffer.buffer[i].life_time * 215.0).cos();
        circular_buffer.buffer[i].velocity.y -= 0.05;
        circular_buffer.buffer[i].position.y += circular_buffer.buffer[i].velocity.y;
        circular_buffer.buffer[i].radius -= 0.15;
        circular_buffer.buffer[i].color.green -= 3;

        if circular_buffer.buffer[i].radius <= 0.02 {
          circular_buffer.buffer[i].alive = false;
        }
      }
    }

    let center = circular_buffer.buffer[i].position;
    let radius = circular_buffer.buffer[i].radius;

    if center.x < -radius
      || center.x > (screen_width as f32 + radius)
      || center.y < -radius
      || center.y > (screen_height as f32 + radius)
    {
      circular_buffer.buffer[i].alive = false;
    }

    i = (i + 1) % MAX_PARTICLES;
  }
}

fn update_circular_buffer(circular_buffer: &mut CircularBuffer) {
  while (circular_buffer.tail != circular_buffer.head)
    && !circular_buffer.buffer[circular_buffer.tail].alive
  {
    circular_buffer.tail = (circular_buffer.tail + 1) % MAX_PARTICLES;
  }
}

fn draw_particles(circular_buffer: &CircularBuffer) {
  let mut i = circular_buffer.tail;
  while i != circular_buffer.head {
    if circular_buffer.buffer[i].alive {
      draw_circle_v(
        circular_buffer.buffer[i].position,
        circular_buffer.buffer[i].radius,
        circular_buffer.buffer[i].color,
      );
    }

    i = (i + 1) % MAX_PARTICLES;
  }
}
