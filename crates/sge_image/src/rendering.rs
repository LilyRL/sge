use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sge_vectors::{IVec2, USizeVec2, Vec2, ivec2};

use sge_camera::d2::Camera2D;
use sge_color::u8::ColorU8;

use super::Image;

const OPTIMIZATION_RECT_FILL_MULTITHREADING_CUTOFF: i32 = 100_000;
const OPTIMIZATION_CIRCLE_RADIUS_MULTITHREADING_CUTOFF: i32 = 50; // 50 radius circle contains about 8000 pixels

#[allow(unused)]
#[allow(clippy::unnecessary_cast)]
impl Image {
    pub fn line_internal<F>(&mut self, mut a: IVec2, mut b: IVec2, color: ColorU8, mut set: F)
    where
        F: FnMut(&mut Image, i32, i32, ColorU8),
    {
        use std::mem::swap;

        let is_steep = (a.x - b.x).abs() < (a.y - b.y).abs();

        if is_steep {
            swap(&mut a.x, &mut a.y);
            swap(&mut b.x, &mut b.y);
        }

        if a.x > b.x {
            swap(&mut a.x, &mut b.x);
            swap(&mut a.y, &mut b.y);
        }

        let dx = b.x - a.x;
        let dy = b.y - a.y;

        let error_increment = dy.abs() * 2;
        let mut error = 0;

        let mut y = a.y;
        for x in a.x..b.x {
            if is_steep {
                set(self, y, x, color);
            } else {
                set(self, x, y, color);
            }
            error += error_increment;

            if error > dx {
                y += if b.y > a.y { 1 } else { -1 };
                error -= dx * 2;
            }
        }
    }

    fn circle_internal(
        &mut self,
        center: IVec2,
        radius: i32,
        color: ColorU8,
        set: fn(&mut Image, i32, i32, ColorU8),
    ) {
        if radius > OPTIMIZATION_CIRCLE_RADIUS_MULTITHREADING_CUTOFF {
            self.circle_internal_multithreaded(center, radius, color);
            return;
        }

        let min = center - radius;
        let max = center + radius;
        let radius_squared = radius * radius;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let distance_squared = (x - center.x).pow(2) + (y - center.y).pow(2);

                if distance_squared <= radius_squared {
                    set(self, x, y, color)
                }
            }
        }
    }

    fn circle_internal_multithreaded(&mut self, center: IVec2, radius: i32, color: ColorU8) {
        let canvas_addr = self as *mut Image as usize;
        let r = radius as f32;

        (-radius..=radius).into_par_iter().for_each(|y_offset| {
            let y_f32 = y_offset as f32;

            let half_w_f32 = (r * r - y_f32 * y_f32).sqrt();
            let half_w = half_w_f32.round() as i32;
            let w = (half_w * 2) as usize;

            let actual_y = center.y + y_offset;
            let actual_x = center.x - half_w;

            if actual_y < 0 || actual_y >= self.height as i32 {
                return;
            }
            if actual_x + w as i32 <= 0 || actual_x >= self.width as i32 {
                return;
            }

            let x_start = actual_x.max(0) as usize;
            let x_end = (actual_x + w as i32).min(self.width as i32) as usize;
            let clipped_w = x_end - x_start;

            if clipped_w > 0 {
                let i = actual_y as usize * self.width + x_start;
                let canvas = unsafe { &mut *(canvas_addr as *mut Image) };
                canvas.buf[i..i + clipped_w].fill(color);
            }
        });
    }

    fn circle_outline_internal(
        &mut self,
        center: IVec2,
        radius: i32,
        outline_width: i32,
        color: ColorU8,
        set: fn(&mut Image, i32, i32, ColorU8),
    ) {
        let min = center - radius;
        let max = center + radius;
        let radius_squared = radius * radius;
        let inner_radius_squared = (radius - outline_width).pow(2);

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let distance_squared = (x - center.x).pow(2) + (y - center.y).pow(2);

                if distance_squared <= radius_squared && distance_squared >= inner_radius_squared {
                    set(self, x, y, color);
                }
            }
        }
    }

    fn rect_internal(&mut self, top_left: IVec2, size: IVec2, color: ColorU8, blend: bool) {
        if size.x.saturating_mul(size.y) > OPTIMIZATION_RECT_FILL_MULTITHREADING_CUTOFF && !blend {
            self.rect_internal_multithreaded(top_left, size, color, blend);
            return;
        }

        let bottom_right = top_left + size;
        let top_left_x = top_left.x.max(0).min(self.width as i32) as usize;
        let top_left_y = top_left.y.max(0).min(self.height as i32) as usize;
        let bottom_right_x = bottom_right.x.max(0).min(self.width as i32) as usize;
        let bottom_right_y = bottom_right.y.max(0).min(self.height as i32) as usize;
        let top_left = USizeVec2::new(top_left_x, top_left_y);
        let bottom_right = USizeVec2::new(bottom_right_x, bottom_right_y);

        let canvas_addr = self as *mut Image as usize;

        ((top_left.y.min(self.height))..(bottom_right.y.min(self.height))).for_each(|y| {
            let i = y * self.width;
            let canvas = unsafe { &mut *(canvas_addr as *mut Image) };

            let ax = top_left.x.min(self.width) + i;
            let bx = bottom_right.x.min(self.width) + i;
            canvas.buf[ax..bx].fill(color);
        });
    }

    fn rect_internal_multithreaded(
        &mut self,
        top_left: IVec2,
        size: IVec2,
        color: ColorU8,
        blend: bool,
    ) {
        let bottom_right = top_left + size;
        let top_left_x = top_left.x.max(0).min(self.width as i32) as usize;
        let top_left_y = top_left.y.max(0).min(self.height as i32) as usize;
        let bottom_right_x = bottom_right.x.max(0).min(self.width as i32) as usize;
        let bottom_right_y = bottom_right.y.max(0).min(self.height as i32) as usize;
        let top_left = USizeVec2::new(top_left_x, top_left_y);
        let bottom_right = USizeVec2::new(bottom_right_x, bottom_right_y);

        let canvas_addr = self as *mut Image as usize;

        ((top_left.y.min(self.height))..(bottom_right.y.min(self.height)))
            .into_par_iter()
            .for_each(|y| {
                let i = y * self.width;
                let canvas = unsafe { &mut *(canvas_addr as *mut Image) };

                let ax = top_left.x.min(self.width) + i;
                let bx = bottom_right.x.min(self.width) + i;
                canvas.buf[ax..bx].fill(color);
            });
    }

    fn rect_outline_internal(
        &mut self,
        top_left: IVec2,
        size: IVec2,
        color: ColorU8,
        set: fn(&mut Image, i32, i32, ColorU8),
    ) {
        let bottom_right = top_left + size;

        let a = top_left;
        let d = bottom_right;
        let b = ivec2(d.x, a.y);
        let c = ivec2(a.x, d.y);

        self.line_internal(a, b, color, set);
        self.line_internal(a, c, color, set);
        self.line_internal(b, d, color, set);
        self.line_internal(c, d, color, set);
    }

    fn triangle_internal(
        &mut self,
        mut a: IVec2,
        mut b: IVec2,
        mut c: IVec2,
        color: ColorU8,
        set: fn(&mut Image, i32, i32, ColorU8),
    ) {
        [&mut a, &mut b, &mut c].sort_by_key(|a| a.y);

        let total_height = (c.y - a.y) as f64;

        for y in a.y..=b.y {
            let segment_height = (b.y - a.y) as f64;
            if segment_height == 0.0 {
                continue;
            }
            let alpha = (y - a.y) as f64 / total_height;
            let beta = (y - a.y) as f64 / segment_height;

            let left_point_x = a.x as f64 + ((c.x - a.x) as f64 * alpha);
            let right_point_x = a.x as f64 + ((b.x - a.x) as f64 * beta);

            let line_point_a = IVec2::new(right_point_x as i32, y);
            let line_point_b = IVec2::new(left_point_x as i32, y);

            self.line_internal(line_point_a, line_point_b, color, set);
        }

        for y in b.y..=c.y {
            let segment_height = (c.y - b.y) as f64;
            if segment_height == 0.0 {
                continue;
            }
            let alpha = (y - a.y) as f64 / total_height;
            let beta = (y - b.y) as f64 / segment_height;

            let left_point_x = a.x as f64 + ((c.x - a.x) as f64 * alpha);
            let right_point_x = b.x as f64 + ((c.x - b.x) as f64 * beta);

            let line_point_a = IVec2::new(right_point_x as i32, y);
            let line_point_b = IVec2::new(left_point_x as i32, y);

            self.line_internal(line_point_a, line_point_b, color, set);
        }
    }

    fn triangle_outline_internal(
        &mut self,
        a: IVec2,
        b: IVec2,
        c: IVec2,
        color: ColorU8,
        set: fn(&mut Image, i32, i32, ColorU8),
    ) {
        let points = [a, b, c, a];

        for p in points.windows(2) {
            self.line_internal(p[0], p[1], color, set);
        }
    }

    pub fn line(&mut self, a: IVec2, b: IVec2, color: ColorU8) {
        self.line_internal(a, b, color, Self::seti);
    }

    pub fn line_blend(&mut self, a: IVec2, b: IVec2, color: ColorU8) {
        self.line_internal(a, b, color, Self::seti_blend);
    }

    /// slow
    pub fn line_thick(&mut self, a: IVec2, b: IVec2, color: ColorU8, radius: i32) {
        let seti_circle = |image: &mut Image, x: i32, y: i32, color: ColorU8| {
            image.circle_filled(ivec2(x, y), radius, color)
        };

        self.line_internal(a, b, color, seti_circle);
    }

    pub fn circle_filled(&mut self, center: IVec2, radius: i32, color: ColorU8) {
        self.circle_internal(center, radius, color, Self::seti);
    }

    pub fn circle_filled_blend(&mut self, center: IVec2, radius: i32, color: ColorU8) {
        self.circle_internal(center, radius, color, Self::seti_blend);
    }

    pub fn circle_outline(
        &mut self,
        center: IVec2,
        radius: i32,
        outline_width: i32,
        color: ColorU8,
    ) {
        self.circle_outline_internal(center, radius, outline_width, color, Self::seti);
    }

    pub fn circle_outline_blend(
        &mut self,
        center: IVec2,
        radius: i32,
        outline_width: i32,
        color: ColorU8,
    ) {
        self.circle_outline_internal(center, radius, outline_width, color, Self::seti_blend);
    }

    pub fn rect_filled(&mut self, top_left: IVec2, size: IVec2, color: ColorU8) {
        self.rect_internal(top_left, size, color, false);
    }

    pub fn rect_filled_blend(&mut self, top_left: IVec2, size: IVec2, color: ColorU8) {
        self.rect_internal(top_left, size, color, true);
    }

    pub fn rect_outline(&mut self, top_left: IVec2, size: IVec2, color: ColorU8) {
        self.rect_outline_internal(top_left, size, color, Self::seti);
    }

    pub fn rect_outline_blend(&mut self, top_left: IVec2, size: IVec2, color: ColorU8) {
        self.rect_outline_internal(top_left, size, color, Self::seti_blend);
    }

    pub fn triangle_filled(&mut self, a: IVec2, b: IVec2, c: IVec2, color: ColorU8) {
        self.triangle_internal(a, b, c, color, Self::seti);
    }

    pub fn triangle_filled_blend(&mut self, a: IVec2, b: IVec2, c: IVec2, color: ColorU8) {
        self.triangle_internal(a, b, c, color, Self::seti_blend);
    }

    pub fn triangle_outline(&mut self, a: IVec2, b: IVec2, c: IVec2, color: ColorU8) {
        self.triangle_outline_internal(a, b, c, color, Self::seti);
    }

    pub fn triangle_outline_blend(&mut self, a: IVec2, b: IVec2, c: IVec2, color: ColorU8) {
        self.triangle_outline_internal(a, b, c, color, Self::seti_blend);
    }

    pub fn square_outline(&mut self, top_left: IVec2, size: i32, color: ColorU8) {
        self.rect_outline(top_left, IVec2::splat(size), color);
    }

    pub fn square_outline_blend(&mut self, top_left: IVec2, size: i32, color: ColorU8) {
        self.rect_outline_blend(top_left, IVec2::splat(size), color);
    }

    pub fn square_filled(&mut self, top_left: IVec2, size: i32, color: ColorU8) {
        self.rect_filled(top_left, IVec2::splat(size), color);
    }

    pub fn square_filled_blend(&mut self, top_left: IVec2, size: i32, color: ColorU8) {
        self.rect_filled_blend(top_left, IVec2::splat(size), color);
    }

    #[inline]
    pub fn is_point_in_bounds(&self, point: IVec2) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }

    #[inline]
    pub fn aabb_intersects_screen(&self, min: IVec2, max: IVec2) -> bool {
        if max.x < 0 || min.x >= self.width as i32 || max.y < 0 || min.y >= self.height as i32 {
            return false;
        }
        true
    }

    pub fn line_world(&mut self, a: Vec2, b: Vec2, camera: &mut Camera2D, color: ColorU8) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x);
        let max_x = a_screen.x.max(b_screen.x);
        let min_y = a_screen.y.min(b_screen.y);
        let max_y = a_screen.y.max(b_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.line(a_screen, b_screen, color);
    }

    pub fn line_world_blend(&mut self, a: Vec2, b: Vec2, camera: &mut Camera2D, color: ColorU8) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x);
        let max_x = a_screen.x.max(b_screen.x);
        let min_y = a_screen.y.min(b_screen.y);
        let max_y = a_screen.y.max(b_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.line_blend(a_screen, b_screen, color);
    }

    pub fn circle_filled_world(
        &mut self,
        center: Vec2,
        radius: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let center_screen = camera.world_to_screen(center).as_ivec2();
        let radius_screen = camera.world_distance_to_screen(radius as f32) as i32;

        let min = center_screen - radius_screen;
        let max = center_screen + radius_screen;

        if !self.aabb_intersects_screen(min, max) {
            return;
        }

        self.circle_filled(center_screen, radius_screen, color);
    }

    pub fn circle_filled_world_blend(
        &mut self,
        center: Vec2,
        radius: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let center_screen = camera.world_to_screen(center).as_ivec2();
        let radius_screen = camera.world_distance_to_screen(radius as f32) as i32;

        let min = center_screen - radius_screen;
        let max = center_screen + radius_screen;

        if !self.aabb_intersects_screen(min, max) {
            return;
        }

        self.circle_filled_blend(center_screen, radius_screen, color);
    }

    pub fn circle_outline_world(
        &mut self,
        center: Vec2,
        radius: f32,
        outline_width: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let center_screen = camera.world_to_screen(center).as_ivec2();
        let radius_screen = camera.world_distance_to_screen(radius as f32) as i32;
        let outline_screen = camera.world_distance_to_screen(outline_width as f32) as i32;
        self.circle_outline(center_screen, radius_screen, outline_screen, color);
    }

    pub fn circle_outline_world_blend(
        &mut self,
        center: Vec2,
        radius: f32,
        outline_width: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let center_screen = camera.world_to_screen(center).as_ivec2();
        let radius_screen = camera.world_distance_to_screen(radius as f32) as i32;
        let outline_screen = camera.world_distance_to_screen(outline_width as f32) as i32;
        self.circle_outline_blend(center_screen, radius_screen, outline_screen, color);
    }

    pub fn rect_filled_world(
        &mut self,
        top_left: Vec2,
        size: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = IVec2::new(
            camera.world_distance_to_screen(size.x as f32) as i32,
            camera.world_distance_to_screen(size.y as f32) as i32,
        );

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.rect_filled(top_left_screen, size_screen, color);
    }

    pub fn rect_filled_world_blend(
        &mut self,
        top_left: Vec2,
        size: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = IVec2::new(
            camera.world_distance_to_screen(size.x as f32) as i32,
            camera.world_distance_to_screen(size.y as f32) as i32,
        );

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.rect_filled_blend(top_left_screen, size_screen, color);
    }

    pub fn rect_outline_world(
        &mut self,
        top_left: Vec2,
        size: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = IVec2::new(
            camera.world_distance_to_screen(size.x as f32) as i32,
            camera.world_distance_to_screen(size.y as f32) as i32,
        );

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.rect_outline(top_left_screen, size_screen, color);
    }

    pub fn rect_outline_world_blend(
        &mut self,
        top_left: Vec2,
        size: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = IVec2::new(
            camera.world_distance_to_screen(size.x as f32) as i32,
            camera.world_distance_to_screen(size.y as f32) as i32,
        );

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.rect_outline_blend(top_left_screen, size_screen, color);
    }

    pub fn triangle_filled_world(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();
        let c_screen = camera.world_to_screen(c).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x).min(c_screen.x);
        let max_x = a_screen.x.max(b_screen.x).max(c_screen.x);
        let min_y = a_screen.y.min(b_screen.y).min(c_screen.y);
        let max_y = a_screen.y.max(b_screen.y).max(c_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.triangle_filled(a_screen, b_screen, c_screen, color);
    }

    pub fn triangle_filled_world_blend(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();
        let c_screen = camera.world_to_screen(c).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x).min(c_screen.x);
        let max_x = a_screen.x.max(b_screen.x).max(c_screen.x);
        let min_y = a_screen.y.min(b_screen.y).min(c_screen.y);
        let max_y = a_screen.y.max(b_screen.y).max(c_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.triangle_filled_blend(a_screen, b_screen, c_screen, color);
    }

    pub fn triangle_outline_world(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();
        let c_screen = camera.world_to_screen(c).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x).min(c_screen.x);
        let max_x = a_screen.x.max(b_screen.x).max(c_screen.x);
        let min_y = a_screen.y.min(b_screen.y).min(c_screen.y);
        let max_y = a_screen.y.max(b_screen.y).max(c_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.triangle_outline(a_screen, b_screen, c_screen, color);
    }

    pub fn triangle_outline_world_blend(
        &mut self,
        a: Vec2,
        b: Vec2,
        c: Vec2,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let a_screen = camera.world_to_screen(a).as_ivec2();
        let b_screen = camera.world_to_screen(b).as_ivec2();
        let c_screen = camera.world_to_screen(c).as_ivec2();

        let min_x = a_screen.x.min(b_screen.x).min(c_screen.x);
        let max_x = a_screen.x.max(b_screen.x).max(c_screen.x);
        let min_y = a_screen.y.min(b_screen.y).min(c_screen.y);
        let max_y = a_screen.y.max(b_screen.y).max(c_screen.y);

        if !self.aabb_intersects_screen(ivec2(min_x, min_y), ivec2(max_x, max_y)) {
            return;
        }

        self.triangle_outline_blend(a_screen, b_screen, c_screen, color);
    }

    pub fn square_outline_world(
        &mut self,
        top_left: Vec2,
        size: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = camera.world_distance_to_screen(size as f32) as i32;

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.square_outline(top_left_screen, size_screen, color);
    }

    pub fn square_outline_world_blend(
        &mut self,
        top_left: Vec2,
        size: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = camera.world_distance_to_screen(size as f32) as i32;

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.square_outline_blend(top_left_screen, size_screen, color);
    }

    pub fn square_filled_world(
        &mut self,
        top_left: Vec2,
        size: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = camera.world_distance_to_screen(size as f32) as i32;

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.square_filled(top_left_screen, size_screen, color);
    }

    pub fn square_filled_world_blend(
        &mut self,
        top_left: Vec2,
        size: f32,
        camera: &mut Camera2D,
        color: ColorU8,
    ) {
        let top_left_screen = camera.world_to_screen(top_left).as_ivec2();
        let size_screen = camera.world_distance_to_screen(size as f32) as i32;

        let max = top_left_screen + size_screen;

        if !self.aabb_intersects_screen(top_left_screen, max) {
            return;
        }

        self.square_filled_blend(top_left_screen, size_screen, color);
    }

    // pub fn draw_text(&mut self, text: &str, top_left: IVec2, color: Pixel, font: &BitmapFont) {
    //     font.draw_text(self, text, top_left.x, top_left.y, 1, color);
    // }

    // pub fn draw_text_ex(
    //     &mut self,
    //     text: &str,
    //     top_left: IVec2,
    //     scale: u32,
    //     color: Pixel,
    //     font: &BitmapFont,
    // ) {
    //     font.draw_text(self, text, top_left.x, top_left.y, scale, color);
    // }

    // pub fn draw_text_world(
    //     &mut self,
    //     text: &str,
    //     top_left: Vec2,
    //     color: Pixel,
    //     font: &BitmapFont,
    //     camera: &mut Camera2D,
    // ) {
    //     self.draw_text_world_ex(text, top_left, 1.0, color, font, camera);
    // }

    // pub fn draw_text_world_ex(
    //     &mut self,
    //     text: &str,
    //     top_left: Vec2,
    //     scale: f32,
    //     color: Pixel,
    //     font: &BitmapFont,
    //     camera: &mut Camera2D,
    // ) {
    //     let top_left_screen = camera.world_to_screen(top_left).as_ivec22();

    //     let scale_screen = camera.world_distance_to_screen(scale as f32);
    //     let approx_width = (text.len() as i32) * (font.glyph_width as i32) * (scale_screen as i32);
    //     let approx_height = (font.glyph_height as i32) * (scale_screen as i32);

    //     let max = top_left_screen + ivec2(approx_width, approx_height);

    //     if !self.aabb_intersects_screen(top_left_screen, max) {
    //         return;
    //     }

    //     font.draw_text(
    //         self,
    //         text,
    //         top_left_screen.x,
    //         top_left_screen.y,
    //         scale_screen,
    //         color,
    //     );
    // }
}
