use crate::parser::{camera_file::Camera, light_file::Light, objects_file::Objects};

pub struct Scene {
  pub camera: Camera,
  pub light: Light,
  pub objects: Vec<Objects>,
}

impl Scene {
  pub fn new(camera: Camera, light: Light, objects: Vec<Objects>) -> Scene {
    Scene {
      camera,
      light,
      objects,
    }
  }

  pub fn trace(&self) -> Vec<u8> {
    //todo: dev gap, replace later by real trace
    self.dummy_trace()
  }

  fn dummy_trace(&self) -> Vec<u8> {
    let width = self.camera.width;
    let height = self.camera.height;
    let mut data = vec![0; (width * height * 3) as usize];
    for row in 0..height {
      for col in 0..width {
        let index = ((row * width + col) * 3) as usize;
        data[index] = 0;
        data[index + 1] = 255;
        data[index + 2] = 0;

        if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
          data[index] = 255;
          data[index + 1] = 0;
          data[index + 2] = 0;
        }
      }
    }
    data

  }

}