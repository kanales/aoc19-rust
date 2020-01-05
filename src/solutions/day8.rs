// TODO make it look nicer
#[derive(Debug)]
pub struct Image {
    height: usize,
    width: usize,
    layers: usize,
    data: Vec<u8>,
}

use std::fmt;

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let translate = |p: u8| match p {
            0 => '█',
            1 => '░',
            2 => ' ',
            x => x.into(),
        };

        for n in 0..self.layers {
            writeln!(f, "Layer {}:", n)?;
            for i in 0..self.height {
                for j in 0..self.width {
                    write!(f, "{}", translate(self.get(i, j, n)))?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl Image {
    pub fn new(height: usize, width: usize, data: Vec<u8>) -> Self {
        Image {
            height: height,
            width: width,
            layers: data.len() / (height * width),
            data: data,
        }
    }

    #[allow(dead_code)]
    pub fn layer<'a>(&'a self, n: usize) -> Layer<'a> {
        let from = self.height * self.width * n;
        let to = from + self.height * self.width;
        Layer {
            number: n,
            height: self.height,
            width: self.width,
            data: &self.data[from..to],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, n: usize, value: u8) {
        let imsize = self.width * self.height;
        self.data[n * imsize + x * self.width + y] = value
    }

    pub fn get(&self, x: usize, y: usize, n: usize) -> u8 {
        let imsize = self.width * self.height;
        self.data[n * imsize + x * self.width + y]
    }

    pub fn layers<'a>(&'a self) -> Layers<'a> {
        Layers {
            layer: 0,
            image: &self,
        }
    }

    pub fn flatten(&self) -> Image {
        let mut output = Image {
            height: self.height,
            width: self.width,
            layers: 1,
            data: vec![2; self.height * self.width],
        };
        // can end earlier
        for layer in self.layers() {
            for i in 0..self.height {
                for j in 0..self.width {
                    let curr = output.get(i, j, 0);
                    let new = layer.get(i, j);
                    output.set(i, j, 0, combine(curr, new));
                }
            }
        }
        output
    }
}

#[derive(Debug)]
pub struct Layer<'a> {
    number: usize,
    height: usize,
    width: usize,
    data: &'a [u8],
}

impl<'a> Layer<'a> {
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y + x * self.width]
    }

    pub fn getn(&self, n: usize) -> u8 {
        self.data[n]
    }

    pub fn pixels(&'a self) -> Pixel<'a> {
        Pixel {
            pixel: 0,
            layer: &self,
        }
    }
}

fn combine(pixel1: u8, pixel2: u8) -> u8 {
    if pixel1 == 2 {
        pixel2
    } else {
        pixel1
    }
}

pub struct Layers<'a> {
    layer: usize,
    image: &'a Image,
}

impl<'a> Iterator for Layers<'a> {
    type Item = Layer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let height = self.image.height;
        let width = self.image.width;
        let layers = self.image.layers;

        if self.layer < layers {
            let from = height * width * self.layer;
            let to = from + height * width;
            self.layer += 1;
            Some(Layer {
                number: self.layer - 1,
                height: height,
                width: width,
                data: &self.image.data[from..to],
            })
        } else {
            None
        }
    }
}

pub struct Pixel<'a> {
    pixel: usize,
    layer: &'a Layer<'a>,
}

impl<'a> Iterator for Pixel<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel < self.layer.height * self.layer.width {
            self.pixel += 1;
            Some(self.layer.getn(self.pixel - 1))
        } else {
            None
        }
    }
}

use std::convert::TryInto;

pub fn digit_to_u8(digit: char) -> Option<u8> {
    let num23: u32 = digit.to_digit(10)?;
    let num8: u8 = num23.try_into().ok()?;
    Some(num8)
}

pub fn part1(input: &String) -> i32 {
    let v: Option<Vec<u8>> = input.chars().map(digit_to_u8).collect();
    let img = Image::new(6, 25, v.unwrap());
    //println!("{:?}", tree);
    let layer: Layer = img
        .layers()
        .min_by_key(|la: &Layer| la.pixels().filter(|p| *p == 0).count())
        .unwrap();

    let ones = layer.pixels().filter(|p| *p == 1).count();
    let twos = layer.pixels().filter(|p| *p == 2).count();
    (ones * twos).try_into().unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &String) -> String {
    let v: Option<Vec<u8>> = input.chars().map(digit_to_u8).collect();
    let img = Image::new(6, 25, v.unwrap());

    let flatimg = img.flatten();
    format!("{}", flatimg)
}

#[test]
fn part1_test() {
    let input: String = super::get_input(8).unwrap();
    assert_eq!(part1(&input), 2048)
}

#[test]
fn part2_test() {
    let input: String = super::get_input(8).unwrap();
    let output = "Layer 0:\n░██░█░░░░█░███░█░░██░██░█\n░██░█░████░███░░██░█░█░██\n░░░░█░░░███░█░█░██░█░░███\n░██░█░██████░██░░░░█░█░██\n░██░█░██████░██░██░█░█░██\n░██░█░██████░██░██░█░██░█\n";
    assert_eq!(part2(&input), output.to_string())
}
