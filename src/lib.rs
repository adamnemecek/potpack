
#[derive(Copy, Clone)]
pub struct InputSize {
    pub id: u32,
    pub w: f32,
    pub h: f32,
}

#[derive(Copy, Clone)]
pub struct Packing {
    pub w: f32,
    pub h: f32,
    pub fill: f32
}

#[derive(Copy, Clone)]
pub struct Space {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<&InputSize> for Space {
    fn from(v: &InputSize) -> Self {
        Self {
            id: v.id,
            x: 0.0,
            y: 0.0,
            w: v.w,
            h: v.h,
        }
    }
}

impl Space {
    pub fn area(&self) -> f32 {
        self.w * self.h
    }
}

pub struct PotPack {
    pub packing: Packing,
    pub spaces: Vec<Space>,
}

pub fn approx_eq(a: f32, b: f32) -> bool {
    (a-b).abs() < 1e-04
}


impl PotPack {
    pub fn new(
        boxes: &[InputSize]
    ) -> Self {
        let mut boxes: Vec<Space> = boxes.iter().map(|x|
            x.into()
        ).collect();

        // calculate total box area and maximum box width
        let mut area: f32 = 0.0;
        let mut max_width: f32 = 0.0;

        // sort the boxes for insertion by height, descending
        for box_ in boxes.iter() {
            area += box_.area();
            max_width = max_width.max(box_.w);
        }

        // sort the boxes for insertion by height, descending
        boxes.sort_by(|a,b| b.h.partial_cmp(&a.h).unwrap());

        let mut start_width = (area/0.95).sqrt().ceil().max(max_width);

        let mut spaces = vec![
            Space { id: 0, x: 0.0, y: 0.0, w: start_width, h: f32::MAX }
        ];

        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for mut box_ in boxes.iter_mut() {
            // look through spaces backwards so that we check smaller spaces first
            for (i, space) in spaces.iter().enumerate().rev() {

                // // look for empty spaces that can accommodate the current box
                if box_.w > space.w || box_.h > space.h {
                    continue;
                }

                // found the space; add the box to its top-left corner
                // |-------|-------|
                // |  box  |       |
                // |_______|       |
                // |         space |
                // |_______________|
                box_.x = space.x;
                box_.y = space.y;

                height = height.max(box_.y + box_.h);
                width = width.max(box_.x + box_.w);

                if box_.w == space.w && box_.h == space.h {
                    // space matches the box exactly; remove it
                    let last = spaces.pop();
                    if i < spaces.len() {
                        spaces[i] = last.unwrap();
                    }

                } else if approx_eq(box_.h, space.h) {
                    // space matches the box height; update it accordingly
                    // |-------|---------------|
                    // |  box  | updated space |
                    // |_______|_______________|
                    spaces[i].x += box_.w;
                    spaces[i].w -= box_.w;

                } else if approx_eq(box_.w, space.w) {
                    // space matches the box width; update it accordingly
                    // |---------------|
                    // |      box      |
                    // |_______________|
                    // | updated space |
                    // |_______________|
                    spaces[i].y += box_.h;
                    spaces[i].h -= box_.h;

                } else {
                    // otherwise the box splits the space into two spaces
                    // |-------|-----------|
                    // |  box  | new space |
                    // |_______|___________|
                    // | updated space     |
                    // |___________________|
                    spaces.push(Space {
                        id: space.id,
                        x: space.x + box_.w,
                        y: space.y,
                        w: space.w - box_.w,
                        h: box_.h
                    });
                    spaces[i].y += box_.h;
                    spaces[i].h -= box_.h;
                }
                break;
            }
        }

        Self {
            packing: Packing {
                w: width,
                h: height,
                fill: area / (width * height)
            },
            spaces
        }
    }
}

mod tests {
    use super::{
        approx_eq,
        PotPack,
        InputSize
    };

    #[test]
    fn test_approx_eq() {
        let a = 1.00005;
        let b = 1.00006;
        assert!(approx_eq(a,b));
    }

    #[test]
    fn test_packing() {
        let mut boxes = vec![];

        for i in 0..100 {
            boxes.push(InputSize {
                id: i,
                w: i as _,
                h: i as _
            });
        }

        let pack = PotPack::new(&boxes);
        assert!(pack.packing.w == 588.0);
        assert!(pack.packing.h == 595.0);
        assert!(pack.packing.fill > 0.93);
    }
}


