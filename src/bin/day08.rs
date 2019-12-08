static INPUT: &str = include_str!("input/day08.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

#[derive(Copy, Clone)]
struct Image<'a> {
    data: &'a [u8],
}

impl<'a> Image<'a> {
    fn layer_count(self) -> usize {
        self.data.len() / LAYER_SIZE
    }

    fn layer(self, layer: usize) -> Layer<'a> {
        let index = layer * LAYER_SIZE;
        Layer {
            data: &self.data[index..index + LAYER_SIZE],
        }
    }

    fn layers(self) -> impl Iterator<Item = Layer<'a>> {
        (0..self.layer_count()).map(move |i| self.layer(i))
    }

    fn layers_rev(self) -> impl Iterator<Item = Layer<'a>> {
        (0..self.layer_count()).rev().map(move |i| self.layer(i))
    }
}

#[derive(Copy, Clone)]
struct Layer<'a> {
    data: &'a [u8],
}

impl<'a> Layer<'a> {
    fn count(&self, digit: u8) -> usize {
        self.data.iter().filter(|&&b| b == digit).count()
    }

    fn printable_row(&self, row: usize) -> impl Iterator<Item = char> + 'a {
        let index = row * WIDTH;
        let row = &self.data[index..index + WIDTH];
        row.iter().map(move |b| match b {
            b'1' => '#',
            _ => ' ',
        })
    }
}

fn main() {
    let image = Image {
        data: INPUT.as_bytes(),
    };

    let min_layer = image
        .layers()
        .min_by_key(|layer| layer.count(b'0'))
        .unwrap();
    let part_1 = min_layer.count(b'1') * min_layer.count(b'2');
    println!("Part 1: {}", part_1);

    let mut buf = [0; LAYER_SIZE];
    for layer in image.layers_rev() {
        for (dst, &src) in buf.iter_mut().zip(layer.data) {
            match src {
                b'0' | b'1' => *dst = src,
                b'2' => continue,
                _ => unimplemented!(),
            }
        }
    }

    println!("Part 2:");
    let result = Layer { data: &buf };
    for y in 0..HEIGHT {
        for c in result.printable_row(y) {
            print!("{}{}", c, c);
        }
        println!();
    }
}
