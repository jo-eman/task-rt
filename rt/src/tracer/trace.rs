use super::scene::Scene;

impl Scene {
  pub fn trace(&self) -> Vec<u8> {
    //todo: dev gap, replace later by real trace
    self.dummy_trace()
  }

  fn real_trace(&self) -> Vec<u8> {
    // objects of the scene to iterate over
    let objects = &self.objects;
    // camera of the scene
    let camera = &self.camera;
    // light of the scene
    let light = &self.light;

    // width of the camera
    let width = camera.width;
    // height of the camera
    let height = camera.height;

    // data to be returned
    let mut data = vec![0; (width * height * 3) as usize];

    // iterate over the pixels of the camera to create rays
    for row in 0..height {
      for col in 0..width {
        // index of the pixel in the data array
        // todo: before loops, calc the upper left corner of the camera. Then move the camera to the right and down, and calc the ray for each pixel
      }
    }
    Vec::new() //todo dev gap
  }

}