extern crate image;

use image::{Pixel, Rgb};

pub struct XTermColor(u8, u8, u8, u8);

impl XTermColor {
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb::from_channels(self.0, self.1, self.2, 255)
    }

    pub fn id(&self) -> u8 {
        self.3
    }
}

pub const XTERM_COLORS: [XTermColor; 256] = [
    XTermColor(0, 0, 0, 0),
    XTermColor(128, 0, 0, 1),
    XTermColor(0, 128, 0, 2),
    XTermColor(128, 128, 0, 3),
    XTermColor(0, 0, 128, 4),
    XTermColor(128, 0, 128, 5),
    XTermColor(0, 128, 128, 6),
    XTermColor(192, 192, 192, 7),
    XTermColor(128, 128, 128, 8),
    XTermColor(255, 0, 0, 9),
    XTermColor(0, 255, 0, 10),
    XTermColor(255, 255, 0, 11),
    XTermColor(0, 0, 255, 12),
    XTermColor(255, 0, 255, 13),
    XTermColor(0, 255, 255, 14),
    XTermColor(255, 255, 255, 15),
    XTermColor(0, 0, 0, 16),
    XTermColor(0, 0, 95, 17),
    XTermColor(0, 0, 135, 18),
    XTermColor(0, 0, 175, 19),
    XTermColor(0, 0, 215, 20),
    XTermColor(0, 0, 255, 21),
    XTermColor(0, 95, 0, 22),
    XTermColor(0, 95, 95, 23),
    XTermColor(0, 95, 135, 24),
    XTermColor(0, 95, 175, 25),
    XTermColor(0, 95, 215, 26),
    XTermColor(0, 95, 255, 27),
    XTermColor(0, 135, 0, 28),
    XTermColor(0, 135, 95, 29),
    XTermColor(0, 135, 135, 30),
    XTermColor(0, 135, 175, 31),
    XTermColor(0, 135, 215, 32),
    XTermColor(0, 135, 255, 33),
    XTermColor(0, 175, 0, 34),
    XTermColor(0, 175, 95, 35),
    XTermColor(0, 175, 135, 36),
    XTermColor(0, 175, 175, 37),
    XTermColor(0, 175, 215, 38),
    XTermColor(0, 175, 255, 39),
    XTermColor(0, 215, 0, 40),
    XTermColor(0, 215, 95, 41),
    XTermColor(0, 215, 135, 42),
    XTermColor(0, 215, 175, 43),
    XTermColor(0, 215, 215, 44),
    XTermColor(0, 215, 255, 45),
    XTermColor(0, 255, 0, 46),
    XTermColor(0, 255, 95, 47),
    XTermColor(0, 255, 135, 48),
    XTermColor(0, 255, 175, 49),
    XTermColor(0, 255, 215, 50),
    XTermColor(0, 255, 255, 51),
    XTermColor(95, 0, 0, 52),
    XTermColor(95, 0, 95, 53),
    XTermColor(95, 0, 135, 54),
    XTermColor(95, 0, 175, 55),
    XTermColor(95, 0, 215, 56),
    XTermColor(95, 0, 255, 57),
    XTermColor(95, 95, 0, 58),
    XTermColor(95, 95, 95, 59),
    XTermColor(95, 95, 135, 60),
    XTermColor(95, 95, 175, 61),
    XTermColor(95, 95, 215, 62),
    XTermColor(95, 95, 255, 63),
    XTermColor(95, 135, 0, 64),
    XTermColor(95, 135, 95, 65),
    XTermColor(95, 135, 135, 66),
    XTermColor(95, 135, 175, 67),
    XTermColor(95, 135, 215, 68),
    XTermColor(95, 135, 255, 69),
    XTermColor(95, 175, 0, 70),
    XTermColor(95, 175, 95, 71),
    XTermColor(95, 175, 135, 72),
    XTermColor(95, 175, 175, 73),
    XTermColor(95, 175, 215, 74),
    XTermColor(95, 175, 255, 75),
    XTermColor(95, 215, 0, 76),
    XTermColor(95, 215, 95, 77),
    XTermColor(95, 215, 135, 78),
    XTermColor(95, 215, 175, 79),
    XTermColor(95, 215, 215, 80),
    XTermColor(95, 215, 255, 81),
    XTermColor(95, 255, 0, 82),
    XTermColor(95, 255, 95, 83),
    XTermColor(95, 255, 135, 84),
    XTermColor(95, 255, 175, 85),
    XTermColor(95, 255, 215, 86),
    XTermColor(95, 255, 255, 87),
    XTermColor(135, 0, 0, 88),
    XTermColor(135, 0, 95, 89),
    XTermColor(135, 0, 135, 90),
    XTermColor(135, 0, 175, 91),
    XTermColor(135, 0, 215, 92),
    XTermColor(135, 0, 255, 93),
    XTermColor(135, 95, 0, 94),
    XTermColor(135, 95, 95, 95),
    XTermColor(135, 95, 135, 96),
    XTermColor(135, 95, 175, 97),
    XTermColor(135, 95, 215, 98),
    XTermColor(135, 95, 255, 99),
    XTermColor(135, 135, 0, 100),
    XTermColor(135, 135, 95, 101),
    XTermColor(135, 135, 135, 102),
    XTermColor(135, 135, 175, 103),
    XTermColor(135, 135, 215, 104),
    XTermColor(135, 135, 255, 105),
    XTermColor(135, 175, 0, 106),
    XTermColor(135, 175, 95, 107),
    XTermColor(135, 175, 135, 108),
    XTermColor(135, 175, 175, 109),
    XTermColor(135, 175, 215, 110),
    XTermColor(135, 175, 255, 111),
    XTermColor(135, 215, 0, 112),
    XTermColor(135, 215, 95, 113),
    XTermColor(135, 215, 135, 114),
    XTermColor(135, 215, 175, 115),
    XTermColor(135, 215, 215, 116),
    XTermColor(135, 215, 255, 117),
    XTermColor(135, 255, 0, 118),
    XTermColor(135, 255, 95, 119),
    XTermColor(135, 255, 135, 120),
    XTermColor(135, 255, 175, 121),
    XTermColor(135, 255, 215, 122),
    XTermColor(135, 255, 255, 123),
    XTermColor(175, 0, 0, 124),
    XTermColor(175, 0, 95, 125),
    XTermColor(175, 0, 135, 126),
    XTermColor(175, 0, 175, 127),
    XTermColor(175, 0, 215, 128),
    XTermColor(175, 0, 255, 129),
    XTermColor(175, 95, 0, 130),
    XTermColor(175, 95, 95, 131),
    XTermColor(175, 95, 135, 132),
    XTermColor(175, 95, 175, 133),
    XTermColor(175, 95, 215, 134),
    XTermColor(175, 95, 255, 135),
    XTermColor(175, 135, 0, 136),
    XTermColor(175, 135, 95, 137),
    XTermColor(175, 135, 135, 138),
    XTermColor(175, 135, 175, 139),
    XTermColor(175, 135, 215, 140),
    XTermColor(175, 135, 255, 141),
    XTermColor(175, 175, 0, 142),
    XTermColor(175, 175, 95, 143),
    XTermColor(175, 175, 135, 144),
    XTermColor(175, 175, 175, 145),
    XTermColor(175, 175, 215, 146),
    XTermColor(175, 175, 255, 147),
    XTermColor(175, 215, 0, 148),
    XTermColor(175, 215, 95, 149),
    XTermColor(175, 215, 135, 150),
    XTermColor(175, 215, 175, 151),
    XTermColor(175, 215, 215, 152),
    XTermColor(175, 215, 255, 153),
    XTermColor(175, 255, 0, 154),
    XTermColor(175, 255, 95, 155),
    XTermColor(175, 255, 135, 156),
    XTermColor(175, 255, 175, 157),
    XTermColor(175, 255, 215, 158),
    XTermColor(175, 255, 255, 159),
    XTermColor(215, 0, 0, 160),
    XTermColor(215, 0, 95, 161),
    XTermColor(215, 0, 135, 162),
    XTermColor(215, 0, 175, 163),
    XTermColor(215, 0, 215, 164),
    XTermColor(215, 0, 255, 165),
    XTermColor(215, 95, 0, 166),
    XTermColor(215, 95, 95, 167),
    XTermColor(215, 95, 135, 168),
    XTermColor(215, 95, 175, 169),
    XTermColor(215, 95, 215, 170),
    XTermColor(215, 95, 255, 171),
    XTermColor(215, 135, 0, 172),
    XTermColor(215, 135, 95, 173),
    XTermColor(215, 135, 135, 174),
    XTermColor(215, 135, 175, 175),
    XTermColor(215, 135, 215, 176),
    XTermColor(215, 135, 255, 177),
    XTermColor(215, 175, 0, 178),
    XTermColor(215, 175, 95, 179),
    XTermColor(215, 175, 135, 180),
    XTermColor(215, 175, 175, 181),
    XTermColor(215, 175, 215, 182),
    XTermColor(215, 175, 255, 183),
    XTermColor(215, 215, 0, 184),
    XTermColor(215, 215, 95, 185),
    XTermColor(215, 215, 135, 186),
    XTermColor(215, 215, 175, 187),
    XTermColor(215, 215, 215, 188),
    XTermColor(215, 215, 255, 189),
    XTermColor(215, 255, 0, 190),
    XTermColor(215, 255, 95, 191),
    XTermColor(215, 255, 135, 192),
    XTermColor(215, 255, 175, 193),
    XTermColor(215, 255, 215, 194),
    XTermColor(215, 255, 255, 195),
    XTermColor(255, 0, 0, 196),
    XTermColor(255, 0, 95, 197),
    XTermColor(255, 0, 135, 198),
    XTermColor(255, 0, 175, 199),
    XTermColor(255, 0, 215, 200),
    XTermColor(255, 0, 255, 201),
    XTermColor(255, 95, 0, 202),
    XTermColor(255, 95, 95, 203),
    XTermColor(255, 95, 135, 204),
    XTermColor(255, 95, 175, 205),
    XTermColor(255, 95, 215, 206),
    XTermColor(255, 95, 255, 207),
    XTermColor(255, 135, 0, 208),
    XTermColor(255, 135, 95, 209),
    XTermColor(255, 135, 135, 210),
    XTermColor(255, 135, 175, 211),
    XTermColor(255, 135, 215, 212),
    XTermColor(255, 135, 255, 213),
    XTermColor(255, 175, 0, 214),
    XTermColor(255, 175, 95, 215),
    XTermColor(255, 175, 135, 216),
    XTermColor(255, 175, 175, 217),
    XTermColor(255, 175, 215, 218),
    XTermColor(255, 175, 255, 219),
    XTermColor(255, 215, 0, 220),
    XTermColor(255, 215, 95, 221),
    XTermColor(255, 215, 135, 222),
    XTermColor(255, 215, 175, 223),
    XTermColor(255, 215, 215, 224),
    XTermColor(255, 215, 255, 225),
    XTermColor(255, 255, 0, 226),
    XTermColor(255, 255, 95, 227),
    XTermColor(255, 255, 135, 228),
    XTermColor(255, 255, 175, 229),
    XTermColor(255, 255, 215, 230),
    XTermColor(255, 255, 255, 231),
    XTermColor(8, 8, 8, 232),
    XTermColor(18, 18, 18, 233),
    XTermColor(28, 28, 28, 234),
    XTermColor(38, 38, 38, 235),
    XTermColor(48, 48, 48, 236),
    XTermColor(58, 58, 58, 237),
    XTermColor(68, 68, 68, 238),
    XTermColor(78, 78, 78, 239),
    XTermColor(88, 88, 88, 240),
    XTermColor(98, 98, 98, 241),
    XTermColor(108, 108, 108, 242),
    XTermColor(118, 118, 118, 243),
    XTermColor(128, 128, 128, 244),
    XTermColor(138, 138, 138, 245),
    XTermColor(148, 148, 148, 246),
    XTermColor(158, 158, 158, 247),
    XTermColor(168, 168, 168, 248),
    XTermColor(178, 178, 178, 249),
    XTermColor(188, 188, 188, 250),
    XTermColor(198, 198, 198, 251),
    XTermColor(208, 208, 208, 252),
    XTermColor(218, 218, 218, 253),
    XTermColor(228, 228, 228, 254),
    XTermColor(238, 238, 238, 255),
];
