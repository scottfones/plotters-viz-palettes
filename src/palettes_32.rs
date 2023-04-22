use plotters::style::Palette;

pub struct PaletteBentCoolWarm32;
pub struct PaletteBlackBody32;
pub struct PaletteExtendedKindlmann32;
pub struct PaletteInferno32;
pub struct PaletteKindlmann32;
pub struct PalettePlasma32;
pub struct PaletteSmoothCoolWarm32;
pub struct PaletteTurbo32;
pub struct PaletteViridis32;

impl Palette for PaletteBentCoolWarm32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (67, 88, 198),
        (76, 99, 202),
        (85, 110, 207),
        (95, 121, 211),
        (105, 132, 215),
        (116, 142, 219),
        (127, 153, 222),
        (139, 164, 225),
        (151, 175, 228),
        (164, 185, 230),
        (177, 196, 233),
        (191, 206, 235),
        (205, 217, 237),
        (219, 227, 239),
        (235, 237, 241),
        (241, 236, 233),
        (238, 224, 215),
        (236, 211, 198),
        (233, 199, 182),
        (229, 186, 166),
        (226, 173, 151),
        (223, 160, 136),
        (219, 147, 122),
        (215, 134, 109),
        (211, 121, 97),
        (206, 107, 85),
        (201, 93, 74),
        (196, 77, 64),
        (191, 61, 54),
        (186, 41, 46),
        (180, 4, 38),
    ];
}

impl Palette for PaletteBlackBody32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (23, 8, 4),
        (35, 15, 9),
        (47, 19, 13),
        (60, 21, 16),
        (73, 24, 19),
        (87, 26, 21),
        (101, 28, 23),
        (115, 30, 25),
        (130, 31, 27),
        (145, 33, 30),
        (160, 33, 32),
        (176, 34, 34),
        (185, 47, 32),
        (193, 59, 30),
        (202, 71, 27),
        (210, 82, 22),
        (218, 93, 16),
        (226, 104, 6),
        (229, 118, 9),
        (230, 132, 15),
        (231, 146, 20),
        (232, 159, 25),
        (233, 172, 30),
        (233, 185, 35),
        (233, 198, 40),
        (232, 211, 45),
        (231, 223, 50),
        (236, 233, 89),
        (246, 240, 148),
        (253, 247, 202),
        (255, 255, 255),
    ];
}

impl Palette for PaletteExtendedKindlmann32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (26, 1, 31),
        (29, 3, 63),
        (23, 4, 91),
        (14, 5, 115),
        (5, 28, 112),
        (4, 47, 91),
        (4, 59, 75),
        (3, 69, 64),
        (4, 79, 51),
        (4, 89, 33),
        (5, 98, 11),
        (14, 107, 5),
        (39, 114, 5),
        (72, 120, 6),
        (107, 123, 6),
        (142, 123, 7),
        (188, 115, 9),
        (243, 90, 12),
        (246, 104, 72),
        (248, 118, 104),
        (249, 128, 151),
        (249, 138, 196),
        (250, 146, 235),
        (235, 166, 251),
        (224, 185, 252),
        (223, 199, 252),
        (228, 211, 253),
        (232, 223, 253),
        (234, 235, 254),
        (238, 247, 254),
        (255, 255, 255),
    ];
}

impl Palette for PaletteInferno32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 4),
        (4, 3, 19),
        (11, 7, 37),
        (22, 11, 57),
        (34, 12, 76),
        (49, 10, 92),
        (63, 10, 102),
        (77, 12, 107),
        (90, 17, 110),
        (103, 22, 110),
        (116, 27, 110),
        (129, 31, 108),
        (142, 36, 105),
        (155, 41, 100),
        (168, 46, 95),
        (181, 51, 88),
        (193, 58, 81),
        (204, 66, 72),
        (215, 75, 63),
        (224, 85, 54),
        (232, 97, 44),
        (239, 109, 34),
        (245, 123, 23),
        (248, 137, 12),
        (251, 152, 6),
        (252, 168, 13),
        (251, 184, 28),
        (249, 200, 48),
        (246, 216, 71),
        (242, 231, 99),
        (243, 245, 133),
        (252, 255, 164),
    ];
}

impl Palette for PaletteKindlmann32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (27, 1, 29),
        (36, 3, 55),
        (39, 4, 79),
        (38, 5, 105),
        (31, 6, 133),
        (25, 8, 158),
        (8, 21, 175),
        (8, 46, 165),
        (7, 64, 147),
        (6, 78, 131),
        (6, 90, 118),
        (5, 101, 109),
        (5, 112, 100),
        (6, 122, 89),
        (6, 132, 75),
        (7, 142, 57),
        (7, 152, 37),
        (8, 162, 15),
        (20, 172, 8),
        (43, 181, 9),
        (74, 188, 9),
        (107, 195, 9),
        (142, 200, 10),
        (177, 203, 10),
        (212, 205, 10),
        (247, 205, 83),
        (251, 211, 169),
        (252, 221, 203),
        (254, 232, 224),
        (254, 244, 240),
        (255, 255, 255),
    ];
}

impl Palette for PalettePlasma32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (13, 8, 135),
        (34, 6, 144),
        (50, 5, 151),
        (64, 4, 157),
        (78, 2, 162),
        (91, 1, 165),
        (104, 0, 168),
        (117, 1, 168),
        (129, 4, 167),
        (141, 11, 165),
        (152, 20, 160),
        (163, 29, 154),
        (173, 38, 147),
        (182, 48, 139),
        (191, 57, 132),
        (199, 66, 124),
        (207, 76, 116),
        (214, 85, 109),
        (221, 94, 102),
        (227, 104, 95),
        (233, 114, 88),
        (238, 124, 81),
        (243, 135, 74),
        (247, 146, 67),
        (250, 157, 59),
        (252, 169, 53),
        (253, 181, 46),
        (253, 194, 41),
        (252, 207, 37),
        (249, 221, 36),
        (245, 235, 39),
        (240, 249, 33),
    ];
}

impl Palette for PaletteSmoothCoolWarm32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (68, 91, 205),
        (78, 105, 216),
        (88, 118, 226),
        (99, 132, 235),
        (110, 144, 242),
        (121, 156, 248),
        (133, 168, 252),
        (144, 178, 254),
        (155, 188, 255),
        (167, 196, 254),
        (178, 204, 251),
        (188, 210, 247),
        (198, 215, 241),
        (208, 218, 234),
        (217, 220, 226),
        (225, 219, 215),
        (233, 214, 203),
        (239, 207, 190),
        (243, 200, 178),
        (246, 191, 165),
        (247, 180, 152),
        (247, 169, 139),
        (245, 157, 126),
        (242, 144, 113),
        (237, 130, 101),
        (230, 115, 89),
        (223, 99, 78),
        (214, 82, 67),
        (204, 63, 57),
        (192, 41, 47),
        (180, 4, 38),
    ];
}

impl Palette for PaletteTurbo32 {
    // calculated from https://gist.github.com/mikhailov-work/ee72ba4191942acecc03fe6da94fc73f
    const COLORS: &'static [(u8, u8, u8)] = &[
        (48, 18, 59),
        (57, 42, 116),
        (64, 65, 164),
        (68, 88, 202),
        (70, 110, 230),
        (70, 131, 249),
        (64, 151, 255),
        (51, 172, 248),
        (37, 192, 232),
        (26, 210, 210),
        (24, 225, 189),
        (35, 236, 169),
        (59, 245, 143),
        (89, 252, 115),
        (121, 255, 89),
        (151, 255, 67),
        (174, 251, 55),
        (195, 242, 52),
        (215, 230, 53),
        (232, 215, 56),
        (245, 198, 58),
        (252, 180, 54),
        (255, 159, 46),
        (253, 134, 36),
        (247, 109, 25),
        (238, 86, 16),
        (227, 67, 9),
        (212, 50, 5),
        (195, 36, 2),
        (174, 23, 1),
        (150, 12, 1),
        (122, 4, 2),
    ];
}

impl Palette for PaletteViridis32 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (68, 1, 84),
        (71, 13, 96),
        (72, 25, 107),
        (72, 36, 116),
        (71, 46, 124),
        (69, 56, 130),
        (65, 66, 134),
        (62, 75, 137),
        (58, 84, 140),
        (54, 93, 141),
        (50, 101, 142),
        (46, 109, 142),
        (43, 117, 142),
        (40, 125, 142),
        (37, 133, 142),
        (34, 140, 141),
        (32, 148, 140),
        (30, 156, 137),
        (32, 163, 134),
        (37, 171, 130),
        (45, 178, 125),
        (57, 186, 118),
        (72, 193, 110),
        (88, 199, 101),
        (106, 205, 91),
        (126, 211, 79),
        (146, 215, 66),
        (168, 219, 52),
        (190, 223, 38),
        (212, 226, 27),
        (233, 228, 26),
        (253, 231, 37),
    ];
}
