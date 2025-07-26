fn to_rgb(index: u8) -> (u8, u8, u8) { crate::rgb_from_ansi256(index) }

fn to_ansi(rgb: (u8, u8, u8)) -> u8 { crate::ansi256_from_rgb(rgb) }

static CUBE_VALUES: [u8; 6] = [0, 95, 135, 175, 215, 255];

/// Tests that getting colour from the ANSI palette gives desired result.
#[test]
fn test_to_rgb() {
    #[rustfmt::skip]
    static SYSTEM_COLOURS: [(u8, u8, u8); 16] = [
        (  0,   0,   0), (205,   0,   0), (  0, 205,   0), (205, 205,   0),
        (  0,   0, 238), (205,   0, 205), (  0, 205, 205), (229, 229, 229),
        (127, 127, 127), (255,   0,   0), (  0, 255,   0), (255, 255,   0),
        ( 92,  92, 255), (255,   0, 255), (  0, 255, 255), (255, 255, 255),
    ];

    // System colours
    for (idx, rgb) in SYSTEM_COLOURS.iter().enumerate() {
        assert_eq!(*rgb, to_rgb(idx as u8));
    }

    // Colour cube
    for idx in 0..216 {
        assert_eq!(
            (
                CUBE_VALUES[idx / 36],
                CUBE_VALUES[(idx / 6) % 6],
                CUBE_VALUES[idx % 6]
            ),
            to_rgb(16 + idx as u8)
        );
    }

    // Greyscale ramp
    for idx in 0..24 {
        let y = idx * 10 + 8;
        assert_eq!((y, y, y), to_rgb(idx + 232));
    }
}

/// Tries all colours in the 256-colour ANSI palette and chooses one with
/// smallest ΔE*₀₀ to `rgb(y, y, y)`.
fn best_grey(y: u8) -> u8 {
    let reference = empfindung::ToLab::to_lab(&rgb::alt::Gray(y));
    CUBE_VALUES
        .iter()
        .enumerate()
        .map(|(idx, v)| (idx as u8 * (36 + 6 + 1) + 16, *v))
        .chain((0..24u8).map(|idx| (idx + 232, idx * 10 + 8)))
        .map(|(idx, grey)| (idx, rgb::alt::Gray(grey)))
        .map(|(idx, grey)| (empfindung::cie00::diff(reference, grey), idx))
        .reduce(|x, y| if x.0 < y.0 { x } else { y })
        .unwrap()
        .1
}

/// Tests that converting `(c, c, c)` colour gives the best possible result.
#[test]
fn test_from_rgb_grey() {
    for i in 0..256 {
        assert_eq!(best_grey(i as u8), to_ansi((i as u8, i as u8, i as u8)));
    }
}

/// Tests that getting value for grey colour given as RGB triple and one given
/// as just shade of grey produce the same result.
#[test]
fn test_greys_agree() {
    for i in 0..256 {
        assert_eq!(
            to_ansi((i as u8, i as u8, i as u8)),
            crate::ansi256_from_grey(i as u8)
        );
    }
}

/// Tests that converting colour which exists in the palette gives index of that
/// colour in the palette.
#[test]
fn test_to_ansi_exact() {
    for i in 16..256 {
        let rgb = to_rgb(i as u8);
        let got = to_ansi(rgb);
        assert_eq!(i as u8, got, "want {:?} but got {:?}", rgb, to_rgb(got));
    }
}

/// Tests a few approximations.
#[test]
#[rustfmt::skip]
fn test_to_ansi_approx() {
    assert_eq!( 16, to_ansi((  1,   1,   1)));
    assert_eq!(232, to_ansi((  7,   7,   7)));
    assert_eq!(232, to_ansi((  8,   7,   8)));
    assert_eq!( 64, to_ansi(( 97, 134,   8)));
}

/// Calculates RGB→ANSI for all colours and calculates a checksum of them
/// comparing it to known value.  This is meant to see whether refactoring of
/// the code does not change the behaviour.  If the computation is changed on
/// purpose simply update the checksum in this test.
#[test]
#[cfg_attr(miri, ignore = "runs too slow on Miri")]
fn from_rgb_checksum() {
    let mut buf = [0; 1 << 12];
    let mut checksum = 0;
    for rgb in 0..(1 << 24) {
        buf[rgb % buf.len()] = crate::ansi256_from_rgb(rgb as u32);
        if rgb % buf.len() == buf.len() - 1 {
            checksum = crc64::crc64(checksum, &buf);
        }
    }
    assert_eq!(3373856917329536106, checksum);
}
