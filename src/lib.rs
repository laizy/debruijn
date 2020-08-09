use once_cell::sync::Lazy;

//debruijn seq: 0000 0111 0111 1100 1011 0101 0011 0001 
const DEBRUIJN32: u32 = 0x077cb531;

static INDEX32: Lazy<[u8; 32]> = Lazy::new(|| {
    let mut index = [0; 32];
    for i in 0..32 {
        let idx = (DEBRUIJN32 << i) >> 27;
        index[idx as usize] = i as u8;
    }

    index
});

/// find first one
pub fn ffo(mut b: u32) -> u32 {
    if b == 0 {
        return 32;
    }
    b &= -(b as i32) as u32;
    b = b.wrapping_mul(DEBRUIJN32) >> 27;
    INDEX32[b as usize] as u32
}

#[test]
fn it_works() {
    assert_eq!(ffo(0u32), 0u32.trailing_zeros());
    for _ in 0..10000 {
        let val = fastrand::u32(..);
        assert_eq!(ffo(val), val.trailing_zeros());
    }
}
