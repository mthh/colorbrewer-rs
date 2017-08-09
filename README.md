# colorbrewer-rs
[![Build Status](https://travis-ci.org/mthh/colorbrewer-rs.svg?branch=master)](https://travis-ci.org/mthh/colorbrewer-rs)

Get a `Vec` of colors (hexadecimal code) from one of the famous _**ColorBrewer**_ palette.
These color specifications and designs are developed by Cynthia Brewer (http://colorbrewer2.org/).

## Usage
```rust
use colorbrewer::{Palette, get_ramp};

// Use an existing palette from the `Palette` Enum and the wanted number of colors:
let ramp = get_ramp(Palette::Pastel2, 3);
assert_eq!(ramp, Some(vec!["#b3e2cd", "#fdcdac", "#cbd5e8"]));

// `None` is returned if the number is invalid:
let ramp = get_ramp(Palette::Pastel2, 35);
assert_eq!(ramp, None);
```

## License
Licensed under Apache License, Version 2.0.

## Credits
- Shameless copy/paste/replace all/etc. of the the `colorbrewer` JS `Object` from https://github.com/saikocat/colorbrewer to transform it in nested rust `match`, wrapped in a function.
- `rustfmt` mostly did the rest of the job.
