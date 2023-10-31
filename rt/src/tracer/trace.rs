use super::scene::Scene;

impl Scene {
  
  pub fn trace(&self) -> Vec<u8> {
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

    // clear the objects, remove the objects that are not good to trace
    let objects = self.good_to_trace(objects);

    // iterate over the pixels of the camera to create rays
    for row in 0..height {
      for col in 0..width {
        // index of the pixel in the data array
        let index = ((row * width + col) * 3) as usize;

        // create ray from the camera to the pixel
        

      }
    }

    self.dev_check_dot_above_below_plane();//todo: remove. dev stuff
    self.dev_print(); //todo: remove. dev stuff

    self.dummy_trace() //todo dev gap
  }

}