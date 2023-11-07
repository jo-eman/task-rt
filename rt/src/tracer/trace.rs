use super::scene::Scene;

use rayon::prelude::*;
use std::sync::Arc;

impl Scene {
    pub fn trace(&self) -> Vec<u8> {
        let camera = &self.camera;
        let width = camera.width;
        let height = camera.height;

        // Pre-calculate values for good_to_trace and avoid cloning inside the loop
        let good_to_trace = Arc::new(self.good_to_trace(&self.objects));
        let good_to_trace_ref = Arc::as_ref(&good_to_trace);

        // Flatten the iteration over pixels
        (0..(width * height))
            .into_par_iter()
            .map(|i| {
                let row = i / width;
                let col = i % width;
                // Use the reference directly without cloning
                let color = self.pixel_color(row, col, good_to_trace_ref);
                vec![color.r, color.g, color.b] // Return the pixel color as a flat vector
            })
            .flatten() // Flatten the results into a single vector
            .collect() // Collect into a Vec<u8> and return
    }
}
