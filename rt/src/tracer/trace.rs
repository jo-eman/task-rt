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
    let mut data = vec![0u8; width * height * 3];

    // clear the objects, remove the objects that are not good to trace
    let good_to_trace = self.good_to_trace(objects);

    // iterate over the pixels of the camera to create rays
    for row in 0..height {
      for col in 0..width {
        // index of the pixel in the data array
        let index = ((row * width + col) * 3) as usize;

        // find the pixel color for the ray
        let color = self.pixel_color(row, col, &good_to_trace);

        // set the pixel color in the data array
        data[index] = color.r;
        data[index + 1] = color.g;
        data[index + 2] = color.b;

      }
    }

    data
    
  }

}