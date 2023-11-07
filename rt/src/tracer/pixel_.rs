use crate::{
    gem::{dot::Dot, gem::Gem, mat::Mat, spear::Spear, utils::F64xyz},
    parser::objects_file::Objects,
};

use std::sync::Arc;

use super::scene::Scene;

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub fresh: bool,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB {
            r,
            g,
            b,
            fresh: true,
        }
    }

    pub fn from_array(array: &[u8; 3]) -> RGB {
        RGB::new(array[0], array[1], array[2])
    }

    /// the same color
    pub fn same(&self) -> RGB {
        RGB::new(self.r, self.g, self.b)
    }

    /// cyan color was choosen as the background color, if the ray does not hit any object properly (f.e.: hit the plane outside the light power distance, or just miss any object)
    pub fn background() -> RGB {
        RGB::new(0, 255, 255)
    }

    /// decrease the color brightness to represent the back side of the object
    ///
    /// just division by the dark factor,
    /// hardcoded into method (not a real simulation of the light)
    ///   
    /// FIRES ONLY ONCE, ON FRESH CREATED COLOR,
    /// after that, the same color is returned
    pub fn dark_side(&self) -> RGB {
        if self.fresh {
            let dark = 2;
            let mut rgb = RGB::new(self.r / dark, self.g / dark, self.b / dark);
            rgb.fresh = false;
            rgb
        } else {
            self.same()
        }
    }

    /// crete color affected by the light power (simple simulation, not a proper one)
    pub fn power_affected(
        rgb: [u8; 3],
        color_position: Dot,
        light_source_position: Dot,
        light_color: RGB,
        light_power_distance: f64,
    ) -> RGB {
        let mut rgb = RGB::from_array(&rgb);
        let distance = color_position.d_dot(&light_source_position);
        if distance < light_power_distance {
            let power_coef = (light_power_distance - distance) / light_power_distance;

            let shade_r = light_color.r as f64 / 255_f64;
            let shade_g = light_color.g as f64 / 255_f64;
            let shade_b = light_color.b as f64 / 255_f64;

            rgb.r = (rgb.r as f64 * power_coef * shade_r) as u8;
            rgb.g = (rgb.g as f64 * power_coef * shade_g) as u8;
            rgb.b = (rgb.b as f64 * power_coef * shade_b) as u8;

            rgb
        } else {
            RGB::background()
        }
    }
}

impl Scene {
    pub fn pixel_color(&self, row: usize, col: usize, good_to_trace: &[Arc<Objects>]) -> RGB {
        let ray = self.camera_ray_to_pixel(row, col);

        let mut rgb = RGB::background();
        let mut nearest_position = Dot::maximum();

        // Create `temp_good_to_trace` outside of the loop
        let temp_good_to_trace: Vec<Objects> =
            good_to_trace.iter().map(|arc| (**arc).clone()).collect();

        // iterate through the objects to find the nearest intersection with the ray
        for (index, arc_object) in good_to_trace.iter().enumerate() {
            let object = &**arc_object;
            match object {
                Objects::Mat { .. } => {
                    (rgb, nearest_position) = self.check_mat(
                        rgb,
                        nearest_position,
                        ray,
                        object,
                        index,
                        &temp_good_to_trace,
                    );
                }
                Objects::Ball { .. } => {
                    (rgb, nearest_position) = self.check_ball(
                        rgb,
                        nearest_position,
                        ray,
                        &object,
                        index,
                        &temp_good_to_trace,
                    );
                }
                Objects::Box { .. } => {
                    (rgb, nearest_position) = self.check_box(
                        rgb,
                        nearest_position,
                        ray,
                        &object,
                        index,
                        &temp_good_to_trace,
                    );
                }
                Objects::Roll { .. } => {
                    (rgb, nearest_position) = self.check_roll(
                        rgb,
                        nearest_position,
                        ray,
                        &object,
                        index,
                        &temp_good_to_trace,
                    );
                }
            }
        }

        rgb
    }
}
