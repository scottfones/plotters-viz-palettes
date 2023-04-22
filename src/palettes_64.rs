use plotters::style::Palette;

pub struct PaletteBentCoolWarm64;
pub struct PaletteBlackBody64;
pub struct PaletteExtendedKindlmann64;
pub struct PaletteInferno64;
pub struct PaletteKindlmann64;
pub struct PalettePlasma64;
pub struct PaletteSmoothCoolWarm64;
pub struct PaletteTurbo64;
pub struct PaletteViridis64;

impl Palette for PaletteBentCoolWarm64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (63, 82, 195),
        (67, 87, 197),
        (71, 93, 200),
        (75, 98, 202),
        (80, 104, 205),
        (84, 109, 207),
        (89, 115, 209),
        (94, 120, 211),
        (99, 125, 213),
        (104, 131, 215),
        (109, 136, 217),
        (115, 141, 218),
        (120, 147, 220),
        (126, 152, 221),
        (131, 157, 223),
        (137, 163, 224),
        (143, 168, 226),
        (149, 173, 227),
        (155, 178, 229),
        (162, 184, 230),
        (168, 189, 231),
        (175, 194, 232),
        (181, 199, 233),
        (188, 204, 234),
        (195, 209, 236),
        (202, 214, 237),
        (209, 220, 238),
        (216, 225, 239),
        (223, 230, 240),
        (231, 235, 241),
        (238, 240, 242),
        (242, 239, 238),
        (240, 233, 229),
        (239, 227, 220),
        (238, 221, 212),
        (236, 215, 203),
        (235, 209, 195),
        (234, 202, 187),
        (232, 196, 179),
        (230, 190, 171),
        (229, 184, 163),
        (227, 177, 156),
        (226, 171, 148),
        (224, 165, 141),
        (222, 158, 134),
        (220, 152, 127),
        (218, 146, 121),
        (216, 139, 114),
        (214, 133, 108),
        (212, 126, 101),
        (210, 119, 95),
        (208, 113, 90),
        (206, 106, 84),
        (203, 99, 79),
        (201, 92, 73),
        (199, 84, 68),
        (196, 77, 63),
        (194, 69, 59),
        (191, 60, 54),
        (188, 51, 50),
        (186, 40, 46),
        (183, 27, 42),
        (180, 4, 38),
    ];
}

impl Palette for PaletteBlackBody64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (13, 4, 2),
        (22, 8, 4),
        (29, 12, 7),
        (34, 15, 9),
        (40, 17, 11),
        (46, 18, 13),
        (53, 20, 15),
        (59, 21, 16),
        (65, 23, 18),
        (72, 24, 19),
        (79, 25, 20),
        (86, 26, 21),
        (93, 27, 22),
        (99, 28, 23),
        (107, 29, 24),
        (114, 30, 25),
        (121, 31, 26),
        (128, 31, 27),
        (135, 32, 28),
        (143, 32, 29),
        (150, 33, 30),
        (158, 33, 31),
        (165, 34, 32),
        (173, 34, 33),
        (179, 37, 34),
        (183, 44, 33),
        (188, 51, 32),
        (192, 57, 30),
        (196, 63, 29),
        (200, 68, 28),
        (204, 74, 26),
        (208, 80, 24),
        (212, 85, 21),
        (216, 90, 18),
        (220, 95, 15),
        (224, 101, 10),
        (227, 106, 5),
        (228, 114, 8),
        (229, 121, 10),
        (230, 128, 13),
        (230, 135, 15),
        (231, 141, 18),
        (232, 148, 21),
        (232, 155, 23),
        (232, 161, 26),
        (233, 168, 28),
        (233, 174, 31),
        (233, 180, 33),
        (233, 187, 36),
        (233, 193, 38),
        (233, 199, 41),
        (232, 206, 43),
        (232, 212, 46),
        (231, 218, 48),
        (231, 224, 51),
        (230, 230, 56),
        (237, 234, 92),
        (242, 237, 122),
        (246, 240, 150),
        (250, 244, 176),
        (253, 248, 203),
        (254, 251, 229),
        (255, 255, 255),
    ];
}

impl Palette for PaletteExtendedKindlmann64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (17, 1, 17),
        (26, 1, 31),
        (29, 2, 46),
        (30, 3, 62),
        (27, 4, 76),
        (24, 4, 89),
        (20, 5, 102),
        (16, 5, 113),
        (6, 13, 120),
        (5, 26, 113),
        (5, 37, 103),
        (4, 45, 92),
        (4, 52, 84),
        (4, 58, 76),
        (3, 63, 70),
        (3, 68, 65),
        (3, 73, 60),
        (4, 78, 53),
        (4, 82, 45),
        (4, 87, 36),
        (4, 92, 26),
        (5, 96, 15),
        (5, 101, 6),
        (12, 105, 5),
        (20, 109, 5),
        (33, 113, 5),
        (48, 116, 6),
        (65, 119, 6),
        (81, 121, 6),
        (98, 122, 6),
        (115, 123, 6),
        (132, 123, 6),
        (152, 122, 7),
        (174, 119, 8),
        (200, 111, 10),
        (227, 99, 11),
        (245, 92, 35),
        (246, 99, 62),
        (246, 107, 76),
        (247, 114, 90),
        (248, 120, 112),
        (249, 125, 135),
        (249, 130, 159),
        (249, 134, 181),
        (249, 139, 202),
        (250, 143, 222),
        (250, 147, 240),
        (244, 156, 250),
        (233, 169, 251),
        (227, 179, 251),
        (223, 187, 252),
        (222, 194, 252),
        (223, 200, 252),
        (225, 206, 253),
        (228, 211, 253),
        (231, 217, 253),
        (232, 223, 253),
        (233, 229, 254),
        (234, 236, 254),
        (235, 242, 254),
        (238, 247, 254),
        (243, 252, 254),
        (255, 255, 255),
    ];
}

impl Palette for PaletteInferno64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 4),
        (2, 1, 10),
        (4, 3, 19),
        (7, 5, 27),
        (11, 7, 36),
        (16, 9, 46),
        (21, 11, 56),
        (27, 12, 65),
        (34, 12, 75),
        (40, 11, 84),
        (48, 10, 91),
        (55, 9, 97),
        (62, 9, 102),
        (69, 10, 105),
        (75, 12, 107),
        (82, 14, 109),
        (88, 16, 110),
        (95, 19, 110),
        (101, 21, 110),
        (108, 24, 110),
        (114, 26, 110),
        (120, 28, 109),
        (127, 30, 108),
        (133, 33, 107),
        (140, 35, 105),
        (146, 37, 104),
        (153, 40, 101),
        (159, 42, 99),
        (165, 45, 96),
        (172, 47, 93),
        (178, 50, 90),
        (184, 53, 87),
        (190, 56, 83),
        (196, 60, 79),
        (201, 64, 75),
        (207, 68, 70),
        (212, 72, 66),
        (217, 77, 61),
        (221, 82, 57),
        (226, 87, 52),
        (230, 93, 47),
        (234, 99, 42),
        (237, 105, 37),
        (240, 111, 32),
        (243, 118, 27),
        (245, 125, 21),
        (247, 132, 16),
        (249, 139, 11),
        (250, 146, 7),
        (251, 154, 6),
        (252, 161, 8),
        (252, 169, 14),
        (252, 177, 21),
        (251, 185, 30),
        (250, 193, 39),
        (249, 201, 49),
        (247, 209, 60),
        (246, 217, 72),
        (244, 224, 85),
        (242, 232, 100),
        (241, 239, 116),
        (243, 245, 133),
        (246, 250, 150),
        (252, 255, 164),
    ];
}

impl Palette for PaletteKindlmann64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (0, 0, 0),
        (17, 1, 17),
        (27, 1, 29),
        (33, 2, 42),
        (36, 3, 54),
        (38, 3, 66),
        (39, 4, 78),
        (39, 4, 90),
        (38, 5, 104),
        (36, 6, 117),
        (32, 6, 131),
        (28, 7, 144),
        (25, 7, 156),
        (17, 8, 169),
        (8, 18, 175),
        (8, 31, 173),
        (8, 43, 167),
        (8, 53, 159),
        (7, 62, 150),
        (7, 70, 141),
        (6, 76, 133),
        (6, 83, 127),
        (6, 88, 120),
        (6, 94, 115),
        (5, 99, 110),
        (5, 104, 106),
        (5, 109, 102),
        (5, 115, 97),
        (6, 120, 92),
        (6, 125, 86),
        (6, 130, 78),
        (6, 135, 71),
        (7, 140, 62),
        (7, 145, 53),
        (7, 150, 43),
        (7, 155, 32),
        (8, 159, 21),
        (8, 164, 11),
        (16, 169, 8),
        (24, 174, 8),
        (35, 178, 9),
        (48, 182, 9),
        (63, 186, 9),
        (79, 190, 9),
        (95, 193, 9),
        (112, 196, 9),
        (129, 198, 10),
        (146, 200, 10),
        (164, 202, 10),
        (181, 204, 10),
        (198, 205, 10),
        (216, 205, 10),
        (235, 205, 11),
        (247, 205, 97),
        (250, 208, 146),
        (251, 212, 172),
        (252, 217, 190),
        (253, 222, 204),
        (253, 227, 215),
        (254, 233, 225),
        (254, 238, 233),
        (254, 244, 241),
        (255, 249, 248),
        (255, 255, 255),
    ];
}

impl Palette for PalettePlasma64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (13, 8, 135),
        (25, 6, 140),
        (34, 6, 144),
        (42, 5, 147),
        (50, 5, 151),
        (57, 4, 154),
        (64, 4, 156),
        (70, 3, 159),
        (77, 2, 161),
        (84, 2, 163),
        (90, 1, 165),
        (96, 1, 166),
        (103, 0, 167),
        (109, 0, 168),
        (115, 1, 168),
        (121, 2, 168),
        (127, 3, 168),
        (133, 6, 167),
        (139, 10, 165),
        (145, 14, 163),
        (150, 19, 161),
        (156, 23, 158),
        (161, 28, 155),
        (166, 32, 152),
        (171, 37, 148),
        (176, 41, 145),
        (180, 46, 141),
        (185, 50, 137),
        (189, 55, 133),
        (193, 59, 130),
        (197, 64, 126),
        (201, 69, 122),
        (205, 73, 118),
        (209, 78, 115),
        (212, 82, 111),
        (216, 87, 108),
        (219, 92, 104),
        (222, 96, 101),
        (225, 101, 97),
        (228, 106, 94),
        (231, 111, 90),
        (234, 116, 87),
        (237, 121, 83),
        (239, 126, 80),
        (242, 131, 76),
        (244, 136, 73),
        (246, 141, 69),
        (248, 147, 66),
        (249, 153, 62),
        (250, 158, 59),
        (252, 164, 55),
        (253, 170, 52),
        (253, 176, 49),
        (254, 182, 46),
        (254, 188, 43),
        (253, 195, 40),
        (253, 201, 38),
        (252, 208, 37),
        (251, 214, 36),
        (249, 221, 37),
        (247, 228, 37),
        (245, 235, 39),
        (242, 242, 39),
        (240, 249, 33),
    ];
}

impl Palette for PaletteSmoothCoolWarm64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (59, 76, 192),
        (63, 83, 199),
        (68, 90, 205),
        (73, 97, 210),
        (78, 104, 216),
        (83, 111, 221),
        (88, 118, 226),
        (93, 124, 230),
        (98, 131, 234),
        (104, 137, 238),
        (109, 143, 242),
        (115, 149, 245),
        (120, 155, 247),
        (126, 161, 250),
        (131, 167, 252),
        (137, 172, 253),
        (142, 177, 254),
        (148, 182, 255),
        (154, 186, 255),
        (159, 191, 255),
        (165, 195, 254),
        (170, 199, 253),
        (176, 202, 252),
        (181, 206, 250),
        (186, 209, 248),
        (191, 211, 246),
        (196, 214, 243),
        (201, 216, 240),
        (206, 217, 236),
        (210, 219, 232),
        (215, 220, 228),
        (219, 220, 223),
        (223, 220, 218),
        (227, 218, 212),
        (231, 215, 206),
        (234, 212, 200),
        (237, 209, 194),
        (240, 206, 188),
        (242, 202, 182),
        (244, 198, 175),
        (245, 194, 169),
        (246, 189, 163),
        (247, 184, 156),
        (247, 179, 150),
        (247, 173, 143),
        (247, 168, 137),
        (246, 162, 131),
        (245, 155, 124),
        (243, 149, 118),
        (241, 142, 112),
        (239, 135, 106),
        (236, 128, 100),
        (233, 121, 94),
        (230, 113, 88),
        (226, 106, 83),
        (222, 98, 77),
        (218, 89, 72),
        (213, 81, 67),
        (208, 72, 61),
        (203, 62, 57),
        (198, 52, 52),
        (192, 41, 47),
        (186, 27, 43),
        (180, 4, 38),
    ];
}

impl Palette for PaletteTurbo64 {
    // calculated from https://gist.github.com/mikhailov-work/ee72ba4191942acecc03fe6da94fc73f
    const COLORS: &'static [(u8, u8, u8)] = &[
        (48, 18, 59),
        (53, 30, 88),
        (57, 42, 116),
        (61, 53, 140),
        (64, 65, 163),
        (66, 76, 183),
        (68, 87, 200),
        (70, 98, 216),
        (70, 108, 229),
        (70, 119, 239),
        (70, 129, 248),
        (69, 139, 253),
        (65, 149, 255),
        (60, 160, 254),
        (53, 170, 249),
        (46, 180, 243),
        (39, 190, 234),
        (32, 199, 224),
        (27, 208, 214),
        (24, 216, 203),
        (23, 223, 192),
        (26, 229, 183),
        (32, 234, 172),
        (42, 239, 161),
        (53, 244, 148),
        (67, 247, 134),
        (82, 250, 121),
        (98, 253, 107),
        (114, 254, 94),
        (130, 255, 82),
        (144, 255, 72),
        (157, 254, 63),
        (168, 252, 57),
        (179, 249, 54),
        (190, 245, 52),
        (200, 240, 52),
        (210, 234, 52),
        (219, 227, 54),
        (227, 220, 55),
        (235, 212, 57),
        (241, 204, 58),
        (247, 195, 58),
        (250, 186, 56),
        (253, 177, 53),
        (254, 167, 50),
        (255, 155, 45),
        (254, 143, 40),
        (252, 131, 35),
        (250, 119, 29),
        (246, 107, 24),
        (242, 95, 19),
        (237, 84, 15),
        (232, 74, 11),
        (226, 65, 9),
        (219, 57, 7),
        (211, 49, 5),
        (203, 42, 3),
        (194, 35, 2),
        (184, 29, 1),
        (173, 23, 1),
        (161, 17, 1),
        (149, 12, 1),
        (136, 8, 1),
        (122, 4, 2),
    ];
}

impl Palette for PaletteViridis64 {
    // from https://www.kennethmoreland.com/color-advice/
    const COLORS: &'static [(u8, u8, u8)] = &[
        (68, 1, 84),
        (70, 7, 90),
        (71, 13, 96),
        (71, 19, 101),
        (72, 24, 107),
        (72, 30, 111),
        (72, 35, 116),
        (72, 40, 120),
        (71, 45, 124),
        (70, 50, 127),
        (69, 55, 130),
        (67, 60, 132),
        (66, 65, 134),
        (64, 70, 136),
        (62, 74, 137),
        (60, 79, 138),
        (58, 83, 139),
        (56, 87, 140),
        (54, 92, 141),
        (52, 96, 141),
        (50, 100, 142),
        (49, 104, 142),
        (47, 108, 142),
        (45, 112, 142),
        (44, 116, 142),
        (42, 119, 142),
        (41, 123, 142),
        (39, 127, 142),
        (38, 131, 142),
        (36, 135, 142),
        (35, 138, 141),
        (33, 142, 141),
        (32, 146, 140),
        (31, 150, 139),
        (31, 154, 138),
        (31, 157, 137),
        (31, 161, 135),
        (33, 165, 134),
        (35, 169, 132),
        (38, 172, 129),
        (42, 176, 127),
        (47, 180, 124),
        (53, 183, 121),
        (59, 187, 117),
        (66, 190, 113),
        (74, 194, 109),
        (82, 197, 105),
        (90, 200, 100),
        (99, 203, 95),
        (108, 206, 89),
        (118, 208, 84),
        (128, 211, 77),
        (138, 213, 71),
        (148, 216, 65),
        (159, 218, 58),
        (169, 220, 51),
        (180, 221, 44),
        (191, 223, 37),
        (202, 224, 31),
        (212, 226, 26),
        (223, 227, 24),
        (233, 228, 26),
        (243, 230, 30),
        (253, 231, 37),
    ];
}
