//! # colorbrewer
//! **Library providing colors from ColorBrewer.**
//!
//! Licensed under Apache License, Version 2.0.
//!
//!
//! Use one member of the `Palette` `Enum` and the requested number of colors
//! to get the corresponding color ramp with the `get_color_ramp` function.
//!
//! ```rust
//! use colorbrewer::*;
//!
//! let ramp_orange = get_color_ramp(Palette::Oranges, 3);
//! assert_eq!(Some(vec![rgb::RGB { r: 254, g: 230, b: 206}, rgb::RGB { r: 253, g: 174, b: 107}, rgb::RGB { r: 230, g: 85, b: 13}]), ramp_orange);
//! ```
//! <br>
//! One can also get the corresponding `Palette` enum variant from it's name as a String
//! as it implements the `FromStr` trait.
//!
//! ```rust
//! use colorbrewer::*;
//!
//! let blue_pal: Palette = "Blues".parse().unwrap();
//! assert_eq!(blue_pal, Palette::Blues);
//! let ramp = get_color_ramp(blue_pal, 3);
//! ```
//! <br>
//! Colors are described by their hexadecimal code.<br>
//! These color specifications and designs are developed by Cynthia Brewer (http://colorbrewer2.org/).
//!

use rgb::RGB;

/// Available color palettes
#[derive(Debug, PartialEq)]
pub enum Palette {
    YlGn,
    YlGnBu,
    GnBu,
    BuGn,
    PuBuGn,
    PuBu,
    BuPu,
    RdPu,
    PuRd,
    OrRd,
    YlOrRd,
    YlOrBr,
    Purples,
    Blues,
    Greens,
    Oranges,
    Reds,
    Greys,
    PuOr,
    BrBG,
    PRGn,
    PiYG,
    RdBu,
    RdGy,
    RdYlBu,
    Spectral,
    RdYlGn,
    Accent,
    Dark2,
    Paired,
    Pastel1,
    Pastel2,
    Set1,
    Set2,
    Set3,
}

impl std::str::FromStr for Palette {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YlGn" => Ok(Palette::YlGn),
            "YlGnBu" => Ok(Palette::YlGnBu),
            "GnBu" => Ok(Palette::GnBu),
            "BuGn" => Ok(Palette::BuGn),
            "PuBuGn" => Ok(Palette::PuBuGn),
            "PuBu" => Ok(Palette::PuBu),
            "BuPu" => Ok(Palette::BuPu),
            "RdPu" => Ok(Palette::RdPu),
            "PuRd" => Ok(Palette::PuRd),
            "OrRd" => Ok(Palette::OrRd),
            "YlOrRd" => Ok(Palette::YlOrRd),
            "YlOrBr" => Ok(Palette::YlOrBr),
            "Purples" => Ok(Palette::Purples),
            "Blues" => Ok(Palette::Blues),
            "Greens" => Ok(Palette::Greens),
            "Oranges" => Ok(Palette::Oranges),
            "Reds" => Ok(Palette::Reds),
            "Greys" => Ok(Palette::Greys),
            "PuOr" => Ok(Palette::PuOr),
            "BrBG" => Ok(Palette::BrBG),
            "PRGn" => Ok(Palette::PRGn),
            "PiYG" => Ok(Palette::PiYG),
            "RdBu" => Ok(Palette::RdBu),
            "RdGy" => Ok(Palette::RdGy),
            "RdYlBu" => Ok(Palette::RdYlBu),
            "Spectral" => Ok(Palette::Spectral),
            "RdYlGn" => Ok(Palette::RdYlGn),
            "Accent" => Ok(Palette::Accent),
            "Dark2" => Ok(Palette::Dark2),
            "Paired" => Ok(Palette::Paired),
            "Pastel1" => Ok(Palette::Pastel1),
            "Pastel2" => Ok(Palette::Pastel2),
            "Set1" => Ok(Palette::Set1),
            "Set2" => Ok(Palette::Set2),
            "Set3" => Ok(Palette::Set3),
            _ => Err("not a valid value"),
        }
    }
}

/// Function to get the requested color ramp
/// according to a given name and number of colors.
/// Return `None` if there is no color ramp defined for this value of `nb_value`.
pub fn get_color_ramp(name: Palette, nb_value: u32) -> Option<Vec<RGB<u8>>> {
    match name {
        Palette::YlGn => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 185,
                },
                RGB {
                    r: 173,
                    g: 221,
                    b: 142,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 194,
                    g: 230,
                    b: 153,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 35,
                    g: 132,
                    b: 67,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 194,
                    g: 230,
                    b: 153,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
                RGB {
                    r: 0,
                    g: 104,
                    b: 55,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 163,
                },
                RGB {
                    r: 173,
                    g: 221,
                    b: 142,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
                RGB {
                    r: 0,
                    g: 104,
                    b: 55,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 163,
                },
                RGB {
                    r: 173,
                    g: 221,
                    b: 142,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 132,
                    b: 67,
                },
                RGB { r: 0, g: 90, b: 50 },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 229,
                },
                RGB {
                    r: 247,
                    g: 252,
                    b: 185,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 163,
                },
                RGB {
                    r: 173,
                    g: 221,
                    b: 142,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 132,
                    b: 67,
                },
                RGB { r: 0, g: 90, b: 50 },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 229,
                },
                RGB {
                    r: 247,
                    g: 252,
                    b: 185,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 163,
                },
                RGB {
                    r: 173,
                    g: 221,
                    b: 142,
                },
                RGB {
                    r: 120,
                    g: 198,
                    b: 121,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 132,
                    b: 67,
                },
                RGB {
                    r: 0,
                    g: 104,
                    b: 55,
                },
                RGB { r: 0, g: 69, b: 41 },
            ]),
            _ => None,
        },
        Palette::YlGnBu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 177,
                },
                RGB {
                    r: 127,
                    g: 205,
                    b: 187,
                },
                RGB {
                    r: 44,
                    g: 127,
                    b: 184,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 161,
                    g: 218,
                    b: 180,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 34,
                    g: 94,
                    b: 168,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 161,
                    g: 218,
                    b: 180,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 44,
                    g: 127,
                    b: 184,
                },
                RGB {
                    r: 37,
                    g: 52,
                    b: 148,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 180,
                },
                RGB {
                    r: 127,
                    g: 205,
                    b: 187,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 44,
                    g: 127,
                    b: 184,
                },
                RGB {
                    r: 37,
                    g: 52,
                    b: 148,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 180,
                },
                RGB {
                    r: 127,
                    g: 205,
                    b: 187,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 29,
                    g: 145,
                    b: 192,
                },
                RGB {
                    r: 34,
                    g: 94,
                    b: 168,
                },
                RGB {
                    r: 12,
                    g: 44,
                    b: 132,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 217,
                },
                RGB {
                    r: 237,
                    g: 248,
                    b: 177,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 180,
                },
                RGB {
                    r: 127,
                    g: 205,
                    b: 187,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 29,
                    g: 145,
                    b: 192,
                },
                RGB {
                    r: 34,
                    g: 94,
                    b: 168,
                },
                RGB {
                    r: 12,
                    g: 44,
                    b: 132,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 217,
                },
                RGB {
                    r: 237,
                    g: 248,
                    b: 177,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 180,
                },
                RGB {
                    r: 127,
                    g: 205,
                    b: 187,
                },
                RGB {
                    r: 65,
                    g: 182,
                    b: 196,
                },
                RGB {
                    r: 29,
                    g: 145,
                    b: 192,
                },
                RGB {
                    r: 34,
                    g: 94,
                    b: 168,
                },
                RGB {
                    r: 37,
                    g: 52,
                    b: 148,
                },
                RGB { r: 8, g: 29, b: 88 },
            ]),
            _ => None,
        },
        Palette::GnBu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 224,
                    g: 243,
                    b: 219,
                },
                RGB {
                    r: 168,
                    g: 221,
                    b: 181,
                },
                RGB {
                    r: 67,
                    g: 162,
                    b: 202,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 240,
                    g: 249,
                    b: 232,
                },
                RGB {
                    r: 186,
                    g: 228,
                    b: 188,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 240,
                    g: 249,
                    b: 232,
                },
                RGB {
                    r: 186,
                    g: 228,
                    b: 188,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 67,
                    g: 162,
                    b: 202,
                },
                RGB {
                    r: 8,
                    g: 104,
                    b: 172,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 240,
                    g: 249,
                    b: 232,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 168,
                    g: 221,
                    b: 181,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 67,
                    g: 162,
                    b: 202,
                },
                RGB {
                    r: 8,
                    g: 104,
                    b: 172,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 240,
                    g: 249,
                    b: 232,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 168,
                    g: 221,
                    b: 181,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 78,
                    g: 179,
                    b: 211,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
                RGB {
                    r: 8,
                    g: 88,
                    b: 158,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 240,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 219,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 168,
                    g: 221,
                    b: 181,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 78,
                    g: 179,
                    b: 211,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
                RGB {
                    r: 8,
                    g: 88,
                    b: 158,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 240,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 219,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 168,
                    g: 221,
                    b: 181,
                },
                RGB {
                    r: 123,
                    g: 204,
                    b: 196,
                },
                RGB {
                    r: 78,
                    g: 179,
                    b: 211,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
                RGB {
                    r: 8,
                    g: 104,
                    b: 172,
                },
                RGB {
                    r: 8,
                    g: 64,
                    b: 129,
                },
            ]),
            _ => None,
        },
        Palette::BuGn => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 229,
                    g: 245,
                    b: 249,
                },
                RGB {
                    r: 153,
                    g: 216,
                    b: 201,
                },
                RGB {
                    r: 44,
                    g: 162,
                    b: 95,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 178,
                    g: 226,
                    b: 226,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 178,
                    g: 226,
                    b: 226,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 44,
                    g: 162,
                    b: 95,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 204,
                    g: 236,
                    b: 230,
                },
                RGB {
                    r: 153,
                    g: 216,
                    b: 201,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 44,
                    g: 162,
                    b: 95,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 204,
                    g: 236,
                    b: 230,
                },
                RGB {
                    r: 153,
                    g: 216,
                    b: 201,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 65,
                    g: 174,
                    b: 118,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB { r: 0, g: 88, b: 36 },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 253,
                },
                RGB {
                    r: 229,
                    g: 245,
                    b: 249,
                },
                RGB {
                    r: 204,
                    g: 236,
                    b: 230,
                },
                RGB {
                    r: 153,
                    g: 216,
                    b: 201,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 65,
                    g: 174,
                    b: 118,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB { r: 0, g: 88, b: 36 },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 253,
                },
                RGB {
                    r: 229,
                    g: 245,
                    b: 249,
                },
                RGB {
                    r: 204,
                    g: 236,
                    b: 230,
                },
                RGB {
                    r: 153,
                    g: 216,
                    b: 201,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 164,
                },
                RGB {
                    r: 65,
                    g: 174,
                    b: 118,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
                RGB { r: 0, g: 68, b: 27 },
            ]),
            _ => None,
        },
        Palette::PuBuGn => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 236,
                    g: 226,
                    b: 240,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 28,
                    g: 144,
                    b: 153,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 246,
                    g: 239,
                    b: 247,
                },
                RGB {
                    r: 189,
                    g: 201,
                    b: 225,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 2,
                    g: 129,
                    b: 138,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 246,
                    g: 239,
                    b: 247,
                },
                RGB {
                    r: 189,
                    g: 201,
                    b: 225,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 28,
                    g: 144,
                    b: 153,
                },
                RGB {
                    r: 1,
                    g: 108,
                    b: 89,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 246,
                    g: 239,
                    b: 247,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 28,
                    g: 144,
                    b: 153,
                },
                RGB {
                    r: 1,
                    g: 108,
                    b: 89,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 246,
                    g: 239,
                    b: 247,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 2,
                    g: 129,
                    b: 138,
                },
                RGB {
                    r: 1,
                    g: 100,
                    b: 80,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 251,
                },
                RGB {
                    r: 236,
                    g: 226,
                    b: 240,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 2,
                    g: 129,
                    b: 138,
                },
                RGB {
                    r: 1,
                    g: 100,
                    b: 80,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 251,
                },
                RGB {
                    r: 236,
                    g: 226,
                    b: 240,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 2,
                    g: 129,
                    b: 138,
                },
                RGB {
                    r: 1,
                    g: 108,
                    b: 89,
                },
                RGB { r: 1, g: 70, b: 54 },
            ]),
            _ => None,
        },
        Palette::PuBu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 236,
                    g: 231,
                    b: 242,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 189,
                    g: 201,
                    b: 225,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 5,
                    g: 112,
                    b: 176,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 189,
                    g: 201,
                    b: 225,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
                RGB {
                    r: 4,
                    g: 90,
                    b: 141,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 43,
                    g: 140,
                    b: 190,
                },
                RGB {
                    r: 4,
                    g: 90,
                    b: 141,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 5,
                    g: 112,
                    b: 176,
                },
                RGB {
                    r: 3,
                    g: 78,
                    b: 123,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 251,
                },
                RGB {
                    r: 236,
                    g: 231,
                    b: 242,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 5,
                    g: 112,
                    b: 176,
                },
                RGB {
                    r: 3,
                    g: 78,
                    b: 123,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 251,
                },
                RGB {
                    r: 236,
                    g: 231,
                    b: 242,
                },
                RGB {
                    r: 208,
                    g: 209,
                    b: 230,
                },
                RGB {
                    r: 166,
                    g: 189,
                    b: 219,
                },
                RGB {
                    r: 116,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 54,
                    g: 144,
                    b: 192,
                },
                RGB {
                    r: 5,
                    g: 112,
                    b: 176,
                },
                RGB {
                    r: 4,
                    g: 90,
                    b: 141,
                },
                RGB { r: 2, g: 56, b: 88 },
            ]),
            _ => None,
        },
        Palette::BuPu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 224,
                    g: 236,
                    b: 244,
                },
                RGB {
                    r: 158,
                    g: 188,
                    b: 218,
                },
                RGB {
                    r: 136,
                    g: 86,
                    b: 167,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 136,
                    g: 65,
                    b: 157,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 136,
                    g: 86,
                    b: 167,
                },
                RGB {
                    r: 129,
                    g: 15,
                    b: 124,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 191,
                    g: 211,
                    b: 230,
                },
                RGB {
                    r: 158,
                    g: 188,
                    b: 218,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 136,
                    g: 86,
                    b: 167,
                },
                RGB {
                    r: 129,
                    g: 15,
                    b: 124,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 251,
                },
                RGB {
                    r: 191,
                    g: 211,
                    b: 230,
                },
                RGB {
                    r: 158,
                    g: 188,
                    b: 218,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 140,
                    g: 107,
                    b: 177,
                },
                RGB {
                    r: 136,
                    g: 65,
                    b: 157,
                },
                RGB {
                    r: 110,
                    g: 1,
                    b: 107,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 253,
                },
                RGB {
                    r: 224,
                    g: 236,
                    b: 244,
                },
                RGB {
                    r: 191,
                    g: 211,
                    b: 230,
                },
                RGB {
                    r: 158,
                    g: 188,
                    b: 218,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 140,
                    g: 107,
                    b: 177,
                },
                RGB {
                    r: 136,
                    g: 65,
                    b: 157,
                },
                RGB {
                    r: 110,
                    g: 1,
                    b: 107,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 253,
                },
                RGB {
                    r: 224,
                    g: 236,
                    b: 244,
                },
                RGB {
                    r: 191,
                    g: 211,
                    b: 230,
                },
                RGB {
                    r: 158,
                    g: 188,
                    b: 218,
                },
                RGB {
                    r: 140,
                    g: 150,
                    b: 198,
                },
                RGB {
                    r: 140,
                    g: 107,
                    b: 177,
                },
                RGB {
                    r: 136,
                    g: 65,
                    b: 157,
                },
                RGB {
                    r: 129,
                    g: 15,
                    b: 124,
                },
                RGB { r: 77, g: 0, b: 75 },
            ]),
            _ => None,
        },
        Palette::RdPu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 253,
                    g: 224,
                    b: 221,
                },
                RGB {
                    r: 250,
                    g: 159,
                    b: 181,
                },
                RGB {
                    r: 197,
                    g: 27,
                    b: 138,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 254,
                    g: 235,
                    b: 226,
                },
                RGB {
                    r: 251,
                    g: 180,
                    b: 185,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 174,
                    g: 1,
                    b: 126,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 254,
                    g: 235,
                    b: 226,
                },
                RGB {
                    r: 251,
                    g: 180,
                    b: 185,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 197,
                    g: 27,
                    b: 138,
                },
                RGB {
                    r: 122,
                    g: 1,
                    b: 119,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 254,
                    g: 235,
                    b: 226,
                },
                RGB {
                    r: 252,
                    g: 197,
                    b: 192,
                },
                RGB {
                    r: 250,
                    g: 159,
                    b: 181,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 197,
                    g: 27,
                    b: 138,
                },
                RGB {
                    r: 122,
                    g: 1,
                    b: 119,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 254,
                    g: 235,
                    b: 226,
                },
                RGB {
                    r: 252,
                    g: 197,
                    b: 192,
                },
                RGB {
                    r: 250,
                    g: 159,
                    b: 181,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 221,
                    g: 52,
                    b: 151,
                },
                RGB {
                    r: 174,
                    g: 1,
                    b: 126,
                },
                RGB {
                    r: 122,
                    g: 1,
                    b: 119,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 243,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 221,
                },
                RGB {
                    r: 252,
                    g: 197,
                    b: 192,
                },
                RGB {
                    r: 250,
                    g: 159,
                    b: 181,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 221,
                    g: 52,
                    b: 151,
                },
                RGB {
                    r: 174,
                    g: 1,
                    b: 126,
                },
                RGB {
                    r: 122,
                    g: 1,
                    b: 119,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 243,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 221,
                },
                RGB {
                    r: 252,
                    g: 197,
                    b: 192,
                },
                RGB {
                    r: 250,
                    g: 159,
                    b: 181,
                },
                RGB {
                    r: 247,
                    g: 104,
                    b: 161,
                },
                RGB {
                    r: 221,
                    g: 52,
                    b: 151,
                },
                RGB {
                    r: 174,
                    g: 1,
                    b: 126,
                },
                RGB {
                    r: 122,
                    g: 1,
                    b: 119,
                },
                RGB {
                    r: 73,
                    g: 0,
                    b: 106,
                },
            ]),
            _ => None,
        },
        Palette::PuRd => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 231,
                    g: 225,
                    b: 239,
                },
                RGB {
                    r: 201,
                    g: 148,
                    b: 199,
                },
                RGB {
                    r: 221,
                    g: 28,
                    b: 119,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 215,
                    g: 181,
                    b: 216,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 206,
                    g: 18,
                    b: 86,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 215,
                    g: 181,
                    b: 216,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 221,
                    g: 28,
                    b: 119,
                },
                RGB {
                    r: 152,
                    g: 0,
                    b: 67,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 212,
                    g: 185,
                    b: 218,
                },
                RGB {
                    r: 201,
                    g: 148,
                    b: 199,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 221,
                    g: 28,
                    b: 119,
                },
                RGB {
                    r: 152,
                    g: 0,
                    b: 67,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 241,
                    g: 238,
                    b: 246,
                },
                RGB {
                    r: 212,
                    g: 185,
                    b: 218,
                },
                RGB {
                    r: 201,
                    g: 148,
                    b: 199,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 206,
                    g: 18,
                    b: 86,
                },
                RGB {
                    r: 145,
                    g: 0,
                    b: 63,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 244,
                    b: 249,
                },
                RGB {
                    r: 231,
                    g: 225,
                    b: 239,
                },
                RGB {
                    r: 212,
                    g: 185,
                    b: 218,
                },
                RGB {
                    r: 201,
                    g: 148,
                    b: 199,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 206,
                    g: 18,
                    b: 86,
                },
                RGB {
                    r: 145,
                    g: 0,
                    b: 63,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 244,
                    b: 249,
                },
                RGB {
                    r: 231,
                    g: 225,
                    b: 239,
                },
                RGB {
                    r: 212,
                    g: 185,
                    b: 218,
                },
                RGB {
                    r: 201,
                    g: 148,
                    b: 199,
                },
                RGB {
                    r: 223,
                    g: 101,
                    b: 176,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 206,
                    g: 18,
                    b: 86,
                },
                RGB {
                    r: 152,
                    g: 0,
                    b: 67,
                },
                RGB {
                    r: 103,
                    g: 0,
                    b: 31,
                },
            ]),
            _ => None,
        },
        Palette::OrRd => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 254,
                    g: 232,
                    b: 200,
                },
                RGB {
                    r: 253,
                    g: 187,
                    b: 132,
                },
                RGB {
                    r: 227,
                    g: 74,
                    b: 51,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 254,
                    g: 240,
                    b: 217,
                },
                RGB {
                    r: 253,
                    g: 204,
                    b: 138,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 31,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 254,
                    g: 240,
                    b: 217,
                },
                RGB {
                    r: 253,
                    g: 204,
                    b: 138,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 227,
                    g: 74,
                    b: 51,
                },
                RGB { r: 179, g: 0, b: 0 },
            ]),
            6 => Some(vec![
                RGB {
                    r: 254,
                    g: 240,
                    b: 217,
                },
                RGB {
                    r: 253,
                    g: 212,
                    b: 158,
                },
                RGB {
                    r: 253,
                    g: 187,
                    b: 132,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 227,
                    g: 74,
                    b: 51,
                },
                RGB { r: 179, g: 0, b: 0 },
            ]),
            7 => Some(vec![
                RGB {
                    r: 254,
                    g: 240,
                    b: 217,
                },
                RGB {
                    r: 253,
                    g: 212,
                    b: 158,
                },
                RGB {
                    r: 253,
                    g: 187,
                    b: 132,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 239,
                    g: 101,
                    b: 72,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 31,
                },
                RGB { r: 153, g: 0, b: 0 },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 236,
                },
                RGB {
                    r: 254,
                    g: 232,
                    b: 200,
                },
                RGB {
                    r: 253,
                    g: 212,
                    b: 158,
                },
                RGB {
                    r: 253,
                    g: 187,
                    b: 132,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 239,
                    g: 101,
                    b: 72,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 31,
                },
                RGB { r: 153, g: 0, b: 0 },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 236,
                },
                RGB {
                    r: 254,
                    g: 232,
                    b: 200,
                },
                RGB {
                    r: 253,
                    g: 212,
                    b: 158,
                },
                RGB {
                    r: 253,
                    g: 187,
                    b: 132,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 239,
                    g: 101,
                    b: 72,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 31,
                },
                RGB { r: 179, g: 0, b: 0 },
                RGB { r: 127, g: 0, b: 0 },
            ]),
            _ => None,
        },
        Palette::YlOrRd => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 255,
                    g: 237,
                    b: 160,
                },
                RGB {
                    r: 254,
                    g: 178,
                    b: 76,
                },
                RGB {
                    r: 240,
                    g: 59,
                    b: 32,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 178,
                },
                RGB {
                    r: 254,
                    g: 204,
                    b: 92,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 178,
                },
                RGB {
                    r: 254,
                    g: 204,
                    b: 92,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 240,
                    g: 59,
                    b: 32,
                },
                RGB {
                    r: 189,
                    g: 0,
                    b: 38,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 178,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 118,
                },
                RGB {
                    r: 254,
                    g: 178,
                    b: 76,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 240,
                    g: 59,
                    b: 32,
                },
                RGB {
                    r: 189,
                    g: 0,
                    b: 38,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 178,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 118,
                },
                RGB {
                    r: 254,
                    g: 178,
                    b: 76,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 252,
                    g: 78,
                    b: 42,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 177,
                    g: 0,
                    b: 38,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 255,
                    g: 237,
                    b: 160,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 118,
                },
                RGB {
                    r: 254,
                    g: 178,
                    b: 76,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 252,
                    g: 78,
                    b: 42,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 177,
                    g: 0,
                    b: 38,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 255,
                    g: 237,
                    b: 160,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 118,
                },
                RGB {
                    r: 254,
                    g: 178,
                    b: 76,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 252,
                    g: 78,
                    b: 42,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 189,
                    g: 0,
                    b: 38,
                },
                RGB {
                    r: 128,
                    g: 0,
                    b: 38,
                },
            ]),
            _ => None,
        },
        Palette::YlOrBr => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 255,
                    g: 247,
                    b: 188,
                },
                RGB {
                    r: 254,
                    g: 196,
                    b: 79,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 14,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 212,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 142,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 204,
                    g: 76,
                    b: 2,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 212,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 142,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 14,
                },
                RGB {
                    r: 153,
                    g: 52,
                    b: 4,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 212,
                },
                RGB {
                    r: 254,
                    g: 227,
                    b: 145,
                },
                RGB {
                    r: 254,
                    g: 196,
                    b: 79,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 14,
                },
                RGB {
                    r: 153,
                    g: 52,
                    b: 4,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 212,
                },
                RGB {
                    r: 254,
                    g: 227,
                    b: 145,
                },
                RGB {
                    r: 254,
                    g: 196,
                    b: 79,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 236,
                    g: 112,
                    b: 20,
                },
                RGB {
                    r: 204,
                    g: 76,
                    b: 2,
                },
                RGB {
                    r: 140,
                    g: 45,
                    b: 4,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 229,
                },
                RGB {
                    r: 255,
                    g: 247,
                    b: 188,
                },
                RGB {
                    r: 254,
                    g: 227,
                    b: 145,
                },
                RGB {
                    r: 254,
                    g: 196,
                    b: 79,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 236,
                    g: 112,
                    b: 20,
                },
                RGB {
                    r: 204,
                    g: 76,
                    b: 2,
                },
                RGB {
                    r: 140,
                    g: 45,
                    b: 4,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 229,
                },
                RGB {
                    r: 255,
                    g: 247,
                    b: 188,
                },
                RGB {
                    r: 254,
                    g: 227,
                    b: 145,
                },
                RGB {
                    r: 254,
                    g: 196,
                    b: 79,
                },
                RGB {
                    r: 254,
                    g: 153,
                    b: 41,
                },
                RGB {
                    r: 236,
                    g: 112,
                    b: 20,
                },
                RGB {
                    r: 204,
                    g: 76,
                    b: 2,
                },
                RGB {
                    r: 153,
                    g: 52,
                    b: 4,
                },
                RGB {
                    r: 102,
                    g: 37,
                    b: 6,
                },
            ]),
            _ => None,
        },
        Palette::Purples => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 239,
                    g: 237,
                    b: 245,
                },
                RGB {
                    r: 188,
                    g: 189,
                    b: 220,
                },
                RGB {
                    r: 117,
                    g: 107,
                    b: 177,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 242,
                    g: 240,
                    b: 247,
                },
                RGB {
                    r: 203,
                    g: 201,
                    b: 226,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 106,
                    g: 81,
                    b: 163,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 242,
                    g: 240,
                    b: 247,
                },
                RGB {
                    r: 203,
                    g: 201,
                    b: 226,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 117,
                    g: 107,
                    b: 177,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 143,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 242,
                    g: 240,
                    b: 247,
                },
                RGB {
                    r: 218,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 188,
                    g: 189,
                    b: 220,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 117,
                    g: 107,
                    b: 177,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 143,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 242,
                    g: 240,
                    b: 247,
                },
                RGB {
                    r: 218,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 188,
                    g: 189,
                    b: 220,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 128,
                    g: 125,
                    b: 186,
                },
                RGB {
                    r: 106,
                    g: 81,
                    b: 163,
                },
                RGB {
                    r: 74,
                    g: 20,
                    b: 134,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 252,
                    g: 251,
                    b: 253,
                },
                RGB {
                    r: 239,
                    g: 237,
                    b: 245,
                },
                RGB {
                    r: 218,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 188,
                    g: 189,
                    b: 220,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 128,
                    g: 125,
                    b: 186,
                },
                RGB {
                    r: 106,
                    g: 81,
                    b: 163,
                },
                RGB {
                    r: 74,
                    g: 20,
                    b: 134,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 252,
                    g: 251,
                    b: 253,
                },
                RGB {
                    r: 239,
                    g: 237,
                    b: 245,
                },
                RGB {
                    r: 218,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 188,
                    g: 189,
                    b: 220,
                },
                RGB {
                    r: 158,
                    g: 154,
                    b: 200,
                },
                RGB {
                    r: 128,
                    g: 125,
                    b: 186,
                },
                RGB {
                    r: 106,
                    g: 81,
                    b: 163,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 143,
                },
                RGB {
                    r: 63,
                    g: 0,
                    b: 125,
                },
            ]),
            _ => None,
        },
        Palette::Blues => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 222,
                    g: 235,
                    b: 247,
                },
                RGB {
                    r: 158,
                    g: 202,
                    b: 225,
                },
                RGB {
                    r: 49,
                    g: 130,
                    b: 189,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 239,
                    g: 243,
                    b: 255,
                },
                RGB {
                    r: 189,
                    g: 215,
                    b: 231,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 33,
                    g: 113,
                    b: 181,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 239,
                    g: 243,
                    b: 255,
                },
                RGB {
                    r: 189,
                    g: 215,
                    b: 231,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 49,
                    g: 130,
                    b: 189,
                },
                RGB {
                    r: 8,
                    g: 81,
                    b: 156,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 239,
                    g: 243,
                    b: 255,
                },
                RGB {
                    r: 198,
                    g: 219,
                    b: 239,
                },
                RGB {
                    r: 158,
                    g: 202,
                    b: 225,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 49,
                    g: 130,
                    b: 189,
                },
                RGB {
                    r: 8,
                    g: 81,
                    b: 156,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 239,
                    g: 243,
                    b: 255,
                },
                RGB {
                    r: 198,
                    g: 219,
                    b: 239,
                },
                RGB {
                    r: 158,
                    g: 202,
                    b: 225,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 66,
                    g: 146,
                    b: 198,
                },
                RGB {
                    r: 33,
                    g: 113,
                    b: 181,
                },
                RGB {
                    r: 8,
                    g: 69,
                    b: 148,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 251,
                    b: 255,
                },
                RGB {
                    r: 222,
                    g: 235,
                    b: 247,
                },
                RGB {
                    r: 198,
                    g: 219,
                    b: 239,
                },
                RGB {
                    r: 158,
                    g: 202,
                    b: 225,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 66,
                    g: 146,
                    b: 198,
                },
                RGB {
                    r: 33,
                    g: 113,
                    b: 181,
                },
                RGB {
                    r: 8,
                    g: 69,
                    b: 148,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 251,
                    b: 255,
                },
                RGB {
                    r: 222,
                    g: 235,
                    b: 247,
                },
                RGB {
                    r: 198,
                    g: 219,
                    b: 239,
                },
                RGB {
                    r: 158,
                    g: 202,
                    b: 225,
                },
                RGB {
                    r: 107,
                    g: 174,
                    b: 214,
                },
                RGB {
                    r: 66,
                    g: 146,
                    b: 198,
                },
                RGB {
                    r: 33,
                    g: 113,
                    b: 181,
                },
                RGB {
                    r: 8,
                    g: 81,
                    b: 156,
                },
                RGB {
                    r: 8,
                    g: 48,
                    b: 107,
                },
            ]),
            _ => None,
        },
        Palette::Greens => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 229,
                    g: 245,
                    b: 224,
                },
                RGB {
                    r: 161,
                    g: 217,
                    b: 155,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 233,
                },
                RGB {
                    r: 186,
                    g: 228,
                    b: 179,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 233,
                },
                RGB {
                    r: 186,
                    g: 228,
                    b: 179,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 233,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 192,
                },
                RGB {
                    r: 161,
                    g: 217,
                    b: 155,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 49,
                    g: 163,
                    b: 84,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 237,
                    g: 248,
                    b: 233,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 192,
                },
                RGB {
                    r: 161,
                    g: 217,
                    b: 155,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB { r: 0, g: 90, b: 50 },
            ]),
            8 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 245,
                },
                RGB {
                    r: 229,
                    g: 245,
                    b: 224,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 192,
                },
                RGB {
                    r: 161,
                    g: 217,
                    b: 155,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB { r: 0, g: 90, b: 50 },
            ]),
            9 => Some(vec![
                RGB {
                    r: 247,
                    g: 252,
                    b: 245,
                },
                RGB {
                    r: 229,
                    g: 245,
                    b: 224,
                },
                RGB {
                    r: 199,
                    g: 233,
                    b: 192,
                },
                RGB {
                    r: 161,
                    g: 217,
                    b: 155,
                },
                RGB {
                    r: 116,
                    g: 196,
                    b: 118,
                },
                RGB {
                    r: 65,
                    g: 171,
                    b: 93,
                },
                RGB {
                    r: 35,
                    g: 139,
                    b: 69,
                },
                RGB {
                    r: 0,
                    g: 109,
                    b: 44,
                },
                RGB { r: 0, g: 68, b: 27 },
            ]),
            _ => None,
        },
        Palette::Oranges => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 254,
                    g: 230,
                    b: 206,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 107,
                },
                RGB {
                    r: 230,
                    g: 85,
                    b: 13,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 254,
                    g: 237,
                    b: 222,
                },
                RGB {
                    r: 253,
                    g: 190,
                    b: 133,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 217,
                    g: 71,
                    b: 1,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 254,
                    g: 237,
                    b: 222,
                },
                RGB {
                    r: 253,
                    g: 190,
                    b: 133,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 230,
                    g: 85,
                    b: 13,
                },
                RGB {
                    r: 166,
                    g: 54,
                    b: 3,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 254,
                    g: 237,
                    b: 222,
                },
                RGB {
                    r: 253,
                    g: 208,
                    b: 162,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 107,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 230,
                    g: 85,
                    b: 13,
                },
                RGB {
                    r: 166,
                    g: 54,
                    b: 3,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 254,
                    g: 237,
                    b: 222,
                },
                RGB {
                    r: 253,
                    g: 208,
                    b: 162,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 107,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 241,
                    g: 105,
                    b: 19,
                },
                RGB {
                    r: 217,
                    g: 72,
                    b: 1,
                },
                RGB {
                    r: 140,
                    g: 45,
                    b: 4,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 245,
                    b: 235,
                },
                RGB {
                    r: 254,
                    g: 230,
                    b: 206,
                },
                RGB {
                    r: 253,
                    g: 208,
                    b: 162,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 107,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 241,
                    g: 105,
                    b: 19,
                },
                RGB {
                    r: 217,
                    g: 72,
                    b: 1,
                },
                RGB {
                    r: 140,
                    g: 45,
                    b: 4,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 245,
                    b: 235,
                },
                RGB {
                    r: 254,
                    g: 230,
                    b: 206,
                },
                RGB {
                    r: 253,
                    g: 208,
                    b: 162,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 107,
                },
                RGB {
                    r: 253,
                    g: 141,
                    b: 60,
                },
                RGB {
                    r: 241,
                    g: 105,
                    b: 19,
                },
                RGB {
                    r: 217,
                    g: 72,
                    b: 1,
                },
                RGB {
                    r: 166,
                    g: 54,
                    b: 3,
                },
                RGB {
                    r: 127,
                    g: 39,
                    b: 4,
                },
            ]),
            _ => None,
        },
        Palette::Reds => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 254,
                    g: 224,
                    b: 210,
                },
                RGB {
                    r: 252,
                    g: 146,
                    b: 114,
                },
                RGB {
                    r: 222,
                    g: 45,
                    b: 38,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 254,
                    g: 229,
                    b: 217,
                },
                RGB {
                    r: 252,
                    g: 174,
                    b: 145,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 203,
                    g: 24,
                    b: 29,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 254,
                    g: 229,
                    b: 217,
                },
                RGB {
                    r: 252,
                    g: 174,
                    b: 145,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 222,
                    g: 45,
                    b: 38,
                },
                RGB {
                    r: 165,
                    g: 15,
                    b: 21,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 254,
                    g: 229,
                    b: 217,
                },
                RGB {
                    r: 252,
                    g: 187,
                    b: 161,
                },
                RGB {
                    r: 252,
                    g: 146,
                    b: 114,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 222,
                    g: 45,
                    b: 38,
                },
                RGB {
                    r: 165,
                    g: 15,
                    b: 21,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 254,
                    g: 229,
                    b: 217,
                },
                RGB {
                    r: 252,
                    g: 187,
                    b: 161,
                },
                RGB {
                    r: 252,
                    g: 146,
                    b: 114,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 239,
                    g: 59,
                    b: 44,
                },
                RGB {
                    r: 203,
                    g: 24,
                    b: 29,
                },
                RGB {
                    r: 153,
                    g: 0,
                    b: 13,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 245,
                    b: 240,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 210,
                },
                RGB {
                    r: 252,
                    g: 187,
                    b: 161,
                },
                RGB {
                    r: 252,
                    g: 146,
                    b: 114,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 239,
                    g: 59,
                    b: 44,
                },
                RGB {
                    r: 203,
                    g: 24,
                    b: 29,
                },
                RGB {
                    r: 153,
                    g: 0,
                    b: 13,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 245,
                    b: 240,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 210,
                },
                RGB {
                    r: 252,
                    g: 187,
                    b: 161,
                },
                RGB {
                    r: 252,
                    g: 146,
                    b: 114,
                },
                RGB {
                    r: 251,
                    g: 106,
                    b: 74,
                },
                RGB {
                    r: 239,
                    g: 59,
                    b: 44,
                },
                RGB {
                    r: 203,
                    g: 24,
                    b: 29,
                },
                RGB {
                    r: 165,
                    g: 15,
                    b: 21,
                },
                RGB {
                    r: 103,
                    g: 0,
                    b: 13,
                },
            ]),
            _ => None,
        },
        Palette::Greys => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 240,
                    g: 240,
                    b: 240,
                },
                RGB {
                    r: 189,
                    g: 189,
                    b: 189,
                },
                RGB {
                    r: 99,
                    g: 99,
                    b: 99,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 204,
                    g: 204,
                    b: 204,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 82,
                    g: 82,
                    b: 82,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 204,
                    g: 204,
                    b: 204,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 99,
                    g: 99,
                    b: 99,
                },
                RGB {
                    r: 37,
                    g: 37,
                    b: 37,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 189,
                    g: 189,
                    b: 189,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 99,
                    g: 99,
                    b: 99,
                },
                RGB {
                    r: 37,
                    g: 37,
                    b: 37,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 189,
                    g: 189,
                    b: 189,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 115,
                    g: 115,
                    b: 115,
                },
                RGB {
                    r: 82,
                    g: 82,
                    b: 82,
                },
                RGB {
                    r: 37,
                    g: 37,
                    b: 37,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 240,
                    g: 240,
                    b: 240,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 189,
                    g: 189,
                    b: 189,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 115,
                    g: 115,
                    b: 115,
                },
                RGB {
                    r: 82,
                    g: 82,
                    b: 82,
                },
                RGB {
                    r: 37,
                    g: 37,
                    b: 37,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 240,
                    g: 240,
                    b: 240,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 189,
                    g: 189,
                    b: 189,
                },
                RGB {
                    r: 150,
                    g: 150,
                    b: 150,
                },
                RGB {
                    r: 115,
                    g: 115,
                    b: 115,
                },
                RGB {
                    r: 82,
                    g: 82,
                    b: 82,
                },
                RGB {
                    r: 37,
                    g: 37,
                    b: 37,
                },
                RGB { r: 0, g: 0, b: 0 },
            ]),
            _ => None,
        },
        Palette::PuOr => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 241,
                    g: 163,
                    b: 64,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 153,
                    g: 142,
                    b: 195,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 230,
                    g: 97,
                    b: 1,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 94,
                    g: 60,
                    b: 153,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 230,
                    g: 97,
                    b: 1,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 94,
                    g: 60,
                    b: 153,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 241,
                    g: 163,
                    b: 64,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 153,
                    g: 142,
                    b: 195,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 241,
                    g: 163,
                    b: 64,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 153,
                    g: 142,
                    b: 195,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 224,
                    g: 130,
                    b: 20,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 128,
                    g: 115,
                    b: 172,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 224,
                    g: 130,
                    b: 20,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 128,
                    g: 115,
                    b: 172,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 127,
                    g: 59,
                    b: 8,
                },
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 224,
                    g: 130,
                    b: 20,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 128,
                    g: 115,
                    b: 172,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
                RGB { r: 45, g: 0, b: 75 },
            ]),
            11 => Some(vec![
                RGB {
                    r: 127,
                    g: 59,
                    b: 8,
                },
                RGB {
                    r: 179,
                    g: 88,
                    b: 6,
                },
                RGB {
                    r: 224,
                    g: 130,
                    b: 20,
                },
                RGB {
                    r: 253,
                    g: 184,
                    b: 99,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 182,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 216,
                    g: 218,
                    b: 235,
                },
                RGB {
                    r: 178,
                    g: 171,
                    b: 210,
                },
                RGB {
                    r: 128,
                    g: 115,
                    b: 172,
                },
                RGB {
                    r: 84,
                    g: 39,
                    b: 136,
                },
                RGB { r: 45, g: 0, b: 75 },
            ]),
            _ => None,
        },
        Palette::BrBG => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 216,
                    g: 179,
                    b: 101,
                },
                RGB {
                    r: 245,
                    g: 245,
                    b: 245,
                },
                RGB {
                    r: 90,
                    g: 180,
                    b: 172,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 166,
                    g: 97,
                    b: 26,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 1,
                    g: 133,
                    b: 113,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 166,
                    g: 97,
                    b: 26,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 245,
                    g: 245,
                    b: 245,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 1,
                    g: 133,
                    b: 113,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 216,
                    g: 179,
                    b: 101,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 90,
                    g: 180,
                    b: 172,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 216,
                    g: 179,
                    b: 101,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 245,
                    g: 245,
                    b: 245,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 90,
                    g: 180,
                    b: 172,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 191,
                    g: 129,
                    b: 45,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 53,
                    g: 151,
                    b: 143,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 191,
                    g: 129,
                    b: 45,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 245,
                    g: 245,
                    b: 245,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 53,
                    g: 151,
                    b: 143,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
            ]),
            10 => Some(vec![
                RGB { r: 84, g: 48, b: 5 },
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 191,
                    g: 129,
                    b: 45,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 53,
                    g: 151,
                    b: 143,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
                RGB { r: 0, g: 60, b: 48 },
            ]),
            11 => Some(vec![
                RGB { r: 84, g: 48, b: 5 },
                RGB {
                    r: 140,
                    g: 81,
                    b: 10,
                },
                RGB {
                    r: 191,
                    g: 129,
                    b: 45,
                },
                RGB {
                    r: 223,
                    g: 194,
                    b: 125,
                },
                RGB {
                    r: 246,
                    g: 232,
                    b: 195,
                },
                RGB {
                    r: 245,
                    g: 245,
                    b: 245,
                },
                RGB {
                    r: 199,
                    g: 234,
                    b: 229,
                },
                RGB {
                    r: 128,
                    g: 205,
                    b: 193,
                },
                RGB {
                    r: 53,
                    g: 151,
                    b: 143,
                },
                RGB {
                    r: 1,
                    g: 102,
                    b: 94,
                },
                RGB { r: 0, g: 60, b: 48 },
            ]),
            _ => None,
        },
        Palette::PRGn => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 175,
                    g: 141,
                    b: 195,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 127,
                    g: 191,
                    b: 123,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 123,
                    g: 50,
                    b: 148,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 0,
                    g: 136,
                    b: 55,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 123,
                    g: 50,
                    b: 148,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 0,
                    g: 136,
                    b: 55,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 175,
                    g: 141,
                    b: 195,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 127,
                    g: 191,
                    b: 123,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 175,
                    g: 141,
                    b: 195,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 127,
                    g: 191,
                    b: 123,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 153,
                    g: 112,
                    b: 171,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 90,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 153,
                    g: 112,
                    b: 171,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 90,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
            ]),
            10 => Some(vec![
                RGB { r: 64, g: 0, b: 75 },
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 153,
                    g: 112,
                    b: 171,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 90,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
                RGB { r: 0, g: 68, b: 27 },
            ]),
            11 => Some(vec![
                RGB { r: 64, g: 0, b: 75 },
                RGB {
                    r: 118,
                    g: 42,
                    b: 131,
                },
                RGB {
                    r: 153,
                    g: 112,
                    b: 171,
                },
                RGB {
                    r: 194,
                    g: 165,
                    b: 207,
                },
                RGB {
                    r: 231,
                    g: 212,
                    b: 232,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 217,
                    g: 240,
                    b: 211,
                },
                RGB {
                    r: 166,
                    g: 219,
                    b: 160,
                },
                RGB {
                    r: 90,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 27,
                    g: 120,
                    b: 55,
                },
                RGB { r: 0, g: 68, b: 27 },
            ]),
            _ => None,
        },
        Palette::PiYG => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 233,
                    g: 163,
                    b: 201,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 161,
                    g: 215,
                    b: 106,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 208,
                    g: 28,
                    b: 139,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 77,
                    g: 172,
                    b: 38,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 208,
                    g: 28,
                    b: 139,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 77,
                    g: 172,
                    b: 38,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 233,
                    g: 163,
                    b: 201,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 161,
                    g: 215,
                    b: 106,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 233,
                    g: 163,
                    b: 201,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 161,
                    g: 215,
                    b: 106,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 222,
                    g: 119,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 127,
                    g: 188,
                    b: 65,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 222,
                    g: 119,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 127,
                    g: 188,
                    b: 65,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 142,
                    g: 1,
                    b: 82,
                },
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 222,
                    g: 119,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 127,
                    g: 188,
                    b: 65,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
                RGB {
                    r: 39,
                    g: 100,
                    b: 25,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 142,
                    g: 1,
                    b: 82,
                },
                RGB {
                    r: 197,
                    g: 27,
                    b: 125,
                },
                RGB {
                    r: 222,
                    g: 119,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 182,
                    b: 218,
                },
                RGB {
                    r: 253,
                    g: 224,
                    b: 239,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 208,
                },
                RGB {
                    r: 184,
                    g: 225,
                    b: 134,
                },
                RGB {
                    r: 127,
                    g: 188,
                    b: 65,
                },
                RGB {
                    r: 77,
                    g: 146,
                    b: 33,
                },
                RGB {
                    r: 39,
                    g: 100,
                    b: 25,
                },
            ]),
            _ => None,
        },
        Palette::RdBu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 202,
                    g: 0,
                    b: 32,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 5,
                    g: 113,
                    b: 176,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 202,
                    g: 0,
                    b: 32,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 5,
                    g: 113,
                    b: 176,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 103,
                    g: 169,
                    b: 207,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 67,
                    g: 147,
                    b: 195,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 67,
                    g: 147,
                    b: 195,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 103,
                    g: 0,
                    b: 31,
                },
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 67,
                    g: 147,
                    b: 195,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
                RGB { r: 5, g: 48, b: 97 },
            ]),
            11 => Some(vec![
                RGB {
                    r: 103,
                    g: 0,
                    b: 31,
                },
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 247,
                    g: 247,
                    b: 247,
                },
                RGB {
                    r: 209,
                    g: 229,
                    b: 240,
                },
                RGB {
                    r: 146,
                    g: 197,
                    b: 222,
                },
                RGB {
                    r: 67,
                    g: 147,
                    b: 195,
                },
                RGB {
                    r: 33,
                    g: 102,
                    b: 172,
                },
                RGB { r: 5, g: 48, b: 97 },
            ]),
            _ => None,
        },
        Palette::RdGy => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 153,
                    g: 153,
                    b: 153,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 202,
                    g: 0,
                    b: 32,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 64,
                    g: 64,
                    b: 64,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 202,
                    g: 0,
                    b: 32,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 64,
                    g: 64,
                    b: 64,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 153,
                    g: 153,
                    b: 153,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 239,
                    g: 138,
                    b: 98,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 153,
                    g: 153,
                    b: 153,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 135,
                    g: 135,
                    b: 135,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 135,
                    g: 135,
                    b: 135,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 103,
                    g: 0,
                    b: 31,
                },
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 135,
                    g: 135,
                    b: 135,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
                RGB {
                    r: 26,
                    g: 26,
                    b: 26,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 103,
                    g: 0,
                    b: 31,
                },
                RGB {
                    r: 178,
                    g: 24,
                    b: 43,
                },
                RGB {
                    r: 214,
                    g: 96,
                    b: 77,
                },
                RGB {
                    r: 244,
                    g: 165,
                    b: 130,
                },
                RGB {
                    r: 253,
                    g: 219,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                RGB {
                    r: 224,
                    g: 224,
                    b: 224,
                },
                RGB {
                    r: 186,
                    g: 186,
                    b: 186,
                },
                RGB {
                    r: 135,
                    g: 135,
                    b: 135,
                },
                RGB {
                    r: 77,
                    g: 77,
                    b: 77,
                },
                RGB {
                    r: 26,
                    g: 26,
                    b: 26,
                },
            ]),
            _ => None,
        },
        Palette::RdYlBu => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 145,
                    g: 191,
                    b: 219,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 44,
                    g: 123,
                    b: 182,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 44,
                    g: 123,
                    b: 182,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 145,
                    g: 191,
                    b: 219,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 145,
                    g: 191,
                    b: 219,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 116,
                    g: 173,
                    b: 209,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 116,
                    g: 173,
                    b: 209,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 165,
                    g: 0,
                    b: 38,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 116,
                    g: 173,
                    b: 209,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
                RGB {
                    r: 49,
                    g: 54,
                    b: 149,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 165,
                    g: 0,
                    b: 38,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 144,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 224,
                    g: 243,
                    b: 248,
                },
                RGB {
                    r: 171,
                    g: 217,
                    b: 233,
                },
                RGB {
                    r: 116,
                    g: 173,
                    b: 209,
                },
                RGB {
                    r: 69,
                    g: 117,
                    b: 180,
                },
                RGB {
                    r: 49,
                    g: 54,
                    b: 149,
                },
            ]),
            _ => None,
        },
        Palette::Spectral => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 153,
                    g: 213,
                    b: 148,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 43,
                    g: 131,
                    b: 186,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 43,
                    g: 131,
                    b: 186,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 153,
                    g: 213,
                    b: 148,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 153,
                    g: 213,
                    b: 148,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 158,
                    g: 1,
                    b: 66,
                },
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
                RGB {
                    r: 94,
                    g: 79,
                    b: 162,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 158,
                    g: 1,
                    b: 66,
                },
                RGB {
                    r: 213,
                    g: 62,
                    b: 79,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 152,
                },
                RGB {
                    r: 171,
                    g: 221,
                    b: 164,
                },
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 50,
                    g: 136,
                    b: 189,
                },
                RGB {
                    r: 94,
                    g: 79,
                    b: 162,
                },
            ]),
            _ => None,
        },
        Palette::RdYlGn => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 145,
                    g: 207,
                    b: 96,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 26,
                    g: 150,
                    b: 65,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 215,
                    g: 25,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 26,
                    g: 150,
                    b: 65,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 145,
                    g: 207,
                    b: 96,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 89,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 145,
                    g: 207,
                    b: 96,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 102,
                    g: 189,
                    b: 99,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 102,
                    g: 189,
                    b: 99,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 165,
                    g: 0,
                    b: 38,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 102,
                    g: 189,
                    b: 99,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
                RGB {
                    r: 0,
                    g: 104,
                    b: 55,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 165,
                    g: 0,
                    b: 38,
                },
                RGB {
                    r: 215,
                    g: 48,
                    b: 39,
                },
                RGB {
                    r: 244,
                    g: 109,
                    b: 67,
                },
                RGB {
                    r: 253,
                    g: 174,
                    b: 97,
                },
                RGB {
                    r: 254,
                    g: 224,
                    b: 139,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 191,
                },
                RGB {
                    r: 217,
                    g: 239,
                    b: 139,
                },
                RGB {
                    r: 166,
                    g: 217,
                    b: 106,
                },
                RGB {
                    r: 102,
                    g: 189,
                    b: 99,
                },
                RGB {
                    r: 26,
                    g: 152,
                    b: 80,
                },
                RGB {
                    r: 0,
                    g: 104,
                    b: 55,
                },
            ]),
            _ => None,
        },
        Palette::Accent => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
                RGB {
                    r: 56,
                    g: 108,
                    b: 176,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
                RGB {
                    r: 56,
                    g: 108,
                    b: 176,
                },
                RGB {
                    r: 240,
                    g: 2,
                    b: 127,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
                RGB {
                    r: 56,
                    g: 108,
                    b: 176,
                },
                RGB {
                    r: 240,
                    g: 2,
                    b: 127,
                },
                RGB {
                    r: 191,
                    g: 91,
                    b: 23,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 127,
                    g: 201,
                    b: 127,
                },
                RGB {
                    r: 190,
                    g: 174,
                    b: 212,
                },
                RGB {
                    r: 253,
                    g: 192,
                    b: 134,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
                RGB {
                    r: 56,
                    g: 108,
                    b: 176,
                },
                RGB {
                    r: 240,
                    g: 2,
                    b: 127,
                },
                RGB {
                    r: 191,
                    g: 91,
                    b: 23,
                },
                RGB {
                    r: 102,
                    g: 102,
                    b: 102,
                },
            ]),
            _ => None,
        },
        Palette::Dark2 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 102,
                    g: 166,
                    b: 30,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 102,
                    g: 166,
                    b: 30,
                },
                RGB {
                    r: 230,
                    g: 171,
                    b: 2,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 102,
                    g: 166,
                    b: 30,
                },
                RGB {
                    r: 230,
                    g: 171,
                    b: 2,
                },
                RGB {
                    r: 166,
                    g: 118,
                    b: 29,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 27,
                    g: 158,
                    b: 119,
                },
                RGB {
                    r: 217,
                    g: 95,
                    b: 2,
                },
                RGB {
                    r: 117,
                    g: 112,
                    b: 179,
                },
                RGB {
                    r: 231,
                    g: 41,
                    b: 138,
                },
                RGB {
                    r: 102,
                    g: 166,
                    b: 30,
                },
                RGB {
                    r: 230,
                    g: 171,
                    b: 2,
                },
                RGB {
                    r: 166,
                    g: 118,
                    b: 29,
                },
                RGB {
                    r: 102,
                    g: 102,
                    b: 102,
                },
            ]),
            _ => None,
        },
        Palette::Paired => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 202,
                    g: 178,
                    b: 214,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 202,
                    g: 178,
                    b: 214,
                },
                RGB {
                    r: 106,
                    g: 61,
                    b: 154,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 202,
                    g: 178,
                    b: 214,
                },
                RGB {
                    r: 106,
                    g: 61,
                    b: 154,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
            ]),
            12 => Some(vec![
                RGB {
                    r: 166,
                    g: 206,
                    b: 227,
                },
                RGB {
                    r: 31,
                    g: 120,
                    b: 180,
                },
                RGB {
                    r: 178,
                    g: 223,
                    b: 138,
                },
                RGB {
                    r: 51,
                    g: 160,
                    b: 44,
                },
                RGB {
                    r: 251,
                    g: 154,
                    b: 153,
                },
                RGB {
                    r: 227,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 253,
                    g: 191,
                    b: 111,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 202,
                    g: 178,
                    b: 214,
                },
                RGB {
                    r: 106,
                    g: 61,
                    b: 154,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 153,
                },
                RGB {
                    r: 177,
                    g: 89,
                    b: 40,
                },
            ]),
            _ => None,
        },
        Palette::Pastel1 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 166,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 166,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 166,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 229,
                    g: 216,
                    b: 189,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 166,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 229,
                    g: 216,
                    b: 189,
                },
                RGB {
                    r: 253,
                    g: 218,
                    b: 236,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 251,
                    g: 180,
                    b: 174,
                },
                RGB {
                    r: 179,
                    g: 205,
                    b: 227,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 222,
                    g: 203,
                    b: 228,
                },
                RGB {
                    r: 254,
                    g: 217,
                    b: 166,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 204,
                },
                RGB {
                    r: 229,
                    g: 216,
                    b: 189,
                },
                RGB {
                    r: 253,
                    g: 218,
                    b: 236,
                },
                RGB {
                    r: 242,
                    g: 242,
                    b: 242,
                },
            ]),
            _ => None,
        },
        Palette::Pastel2 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
                RGB {
                    r: 244,
                    g: 202,
                    b: 228,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
                RGB {
                    r: 244,
                    g: 202,
                    b: 228,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 201,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
                RGB {
                    r: 244,
                    g: 202,
                    b: 228,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 201,
                },
                RGB {
                    r: 255,
                    g: 242,
                    b: 174,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
                RGB {
                    r: 244,
                    g: 202,
                    b: 228,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 201,
                },
                RGB {
                    r: 255,
                    g: 242,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 226,
                    b: 204,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205,
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172,
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232,
                },
                RGB {
                    r: 244,
                    g: 202,
                    b: 228,
                },
                RGB {
                    r: 230,
                    g: 245,
                    b: 201,
                },
                RGB {
                    r: 255,
                    g: 242,
                    b: 174,
                },
                RGB {
                    r: 241,
                    g: 226,
                    b: 204,
                },
                RGB {
                    r: 204,
                    g: 204,
                    b: 204,
                },
            ]),
            _ => None,
        },
        Palette::Set1 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 51,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 51,
                },
                RGB {
                    r: 166,
                    g: 86,
                    b: 40,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 51,
                },
                RGB {
                    r: 166,
                    g: 86,
                    b: 40,
                },
                RGB {
                    r: 247,
                    g: 129,
                    b: 191,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 228,
                    g: 26,
                    b: 28,
                },
                RGB {
                    r: 55,
                    g: 126,
                    b: 184,
                },
                RGB {
                    r: 77,
                    g: 175,
                    b: 74,
                },
                RGB {
                    r: 152,
                    g: 78,
                    b: 163,
                },
                RGB {
                    r: 255,
                    g: 127,
                    b: 0,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 51,
                },
                RGB {
                    r: 166,
                    g: 86,
                    b: 40,
                },
                RGB {
                    r: 247,
                    g: 129,
                    b: 191,
                },
                RGB {
                    r: 153,
                    g: 153,
                    b: 153,
                },
            ]),
            _ => None,
        },
        Palette::Set2 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
                RGB {
                    r: 231,
                    g: 138,
                    b: 195,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
                RGB {
                    r: 231,
                    g: 138,
                    b: 195,
                },
                RGB {
                    r: 166,
                    g: 216,
                    b: 84,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
                RGB {
                    r: 231,
                    g: 138,
                    b: 195,
                },
                RGB {
                    r: 166,
                    g: 216,
                    b: 84,
                },
                RGB {
                    r: 255,
                    g: 217,
                    b: 47,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
                RGB {
                    r: 231,
                    g: 138,
                    b: 195,
                },
                RGB {
                    r: 166,
                    g: 216,
                    b: 84,
                },
                RGB {
                    r: 255,
                    g: 217,
                    b: 47,
                },
                RGB {
                    r: 229,
                    g: 196,
                    b: 148,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 102,
                    g: 194,
                    b: 165,
                },
                RGB {
                    r: 252,
                    g: 141,
                    b: 98,
                },
                RGB {
                    r: 141,
                    g: 160,
                    b: 203,
                },
                RGB {
                    r: 231,
                    g: 138,
                    b: 195,
                },
                RGB {
                    r: 166,
                    g: 216,
                    b: 84,
                },
                RGB {
                    r: 255,
                    g: 217,
                    b: 47,
                },
                RGB {
                    r: 229,
                    g: 196,
                    b: 148,
                },
                RGB {
                    r: 179,
                    g: 179,
                    b: 179,
                },
            ]),
            _ => None,
        },
        Palette::Set3 => match nb_value {
            3 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
            ]),
            4 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
            ]),
            5 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
            ]),
            6 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
            ]),
            7 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
            ]),
            8 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
                RGB {
                    r: 252,
                    g: 205,
                    b: 229,
                },
            ]),
            9 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
                RGB {
                    r: 252,
                    g: 205,
                    b: 229,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
            ]),
            10 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
                RGB {
                    r: 252,
                    g: 205,
                    b: 229,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 188,
                    g: 128,
                    b: 189,
                },
            ]),
            11 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
                RGB {
                    r: 252,
                    g: 205,
                    b: 229,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 188,
                    g: 128,
                    b: 189,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
            ]),
            12 => Some(vec![
                RGB {
                    r: 141,
                    g: 211,
                    b: 199,
                },
                RGB {
                    r: 255,
                    g: 255,
                    b: 179,
                },
                RGB {
                    r: 190,
                    g: 186,
                    b: 218,
                },
                RGB {
                    r: 251,
                    g: 128,
                    b: 114,
                },
                RGB {
                    r: 128,
                    g: 177,
                    b: 211,
                },
                RGB {
                    r: 253,
                    g: 180,
                    b: 98,
                },
                RGB {
                    r: 179,
                    g: 222,
                    b: 105,
                },
                RGB {
                    r: 252,
                    g: 205,
                    b: 229,
                },
                RGB {
                    r: 217,
                    g: 217,
                    b: 217,
                },
                RGB {
                    r: 188,
                    g: 128,
                    b: 189,
                },
                RGB {
                    r: 204,
                    g: 235,
                    b: 197,
                },
                RGB {
                    r: 255,
                    g: 237,
                    b: 111,
                },
            ]),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_color_ramp, Palette};
    use rgb::RGB;

    #[test]
    fn test_get_color_ramp() {
        let ramp = get_color_ramp(Palette::Pastel2, 3);
        assert_eq!(
            ramp,
            Some(vec![
                RGB {
                    r: 179,
                    g: 226,
                    b: 205
                },
                RGB {
                    r: 253,
                    g: 205,
                    b: 172
                },
                RGB {
                    r: 203,
                    g: 213,
                    b: 232
                }
            ])
        );
    }

    #[test]
    fn test_palette_enum_from_string() {
        let palette_pastel2: Palette = "Pastel2".parse().unwrap();
        assert_eq!(palette_pastel2, Palette::Pastel2);
    }
}
