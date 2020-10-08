# colorbrewer-rs
[![Build Status](https://travis-ci.org/mthh/colorbrewer-rs.svg?branch=master)](https://travis-ci.org/mthh/colorbrewer-rs)

Get a `Vec` of colors (described using [rgb](https://crates.io/crates/rgb) crate) from one of the famous _**ColorBrewer**_ palette.
These color specifications and designs are developed by Cynthia Brewer (http://colorbrewer2.org/).

## Usage
```rust
use colorbrewer::{Palette, get_ramp};

// Use an existing palette from the `Palette` Enum and the wanted number of colors:
let ramp_orange = get_color_ramp(Palette::Oranges, 3);
assert_eq!(
  ramp_orange,
  Some(
    vec![
      rgb::RGB { r: 254, g: 230, b: 206},
      rgb::RGB { r: 253, g: 174, b: 107},
      rgb::RGB { r: 230, g: 85, b: 13},
    ]
  ));

// Or match the name of one palette with the corresponding Enum variant
// using the 'parse' method of a string:
let blue_pal: Palette = "Blues".parse().unwrap();
let ramp = get_color_ramp(blue_pal, 5);

// `None` is returned if the number is invalid:
let ramp = get_color_ramp(Palette::Pastel2, 35);
assert_eq!(ramp, None);
```

## License
Licensed under Apache License, Version 2.0.

## Credits
- Shameless copy/paste/replace all/etc. of the the `colorbrewer` JS `Object` from https://github.com/saikocat/colorbrewer to transform it in nested rust `match`, wrapped in a function.
- `rustfmt` mostly did the rest of the job.
