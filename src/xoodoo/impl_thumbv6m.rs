use super::{Xoodoo, ROUND_KEYS};

#[cfg(all(target_arch = "arm", not(target_feature = "thumb2")))]
impl Xoodoo {
    pub fn permute(&mut self) {
        // Since Xoodoo is #[repr(align(4))], self.st is guaranteed to be 4-byte aligned.
        // On Little Endian, we can safely treat it as a [u32; 12] array in-place.
        let mut st_buf = [0u32; 12];
        let st = if cfg!(target_endian = "little") {
            unsafe { &mut *(self.st.as_mut_ptr() as *mut [u32; 12]) }
        } else {
            use core::convert::TryInto;
            for i in 0..12 {
                st_buf[i] = u32::from_le_bytes(self.st[i * 4..i * 4 + 4].try_into().unwrap());
            }
            &mut st_buf
        };

        for &round_key in &ROUND_KEYS {
            let p = [
                st[0] ^ st[4] ^ st[8],
                st[1] ^ st[5] ^ st[9],
                st[2] ^ st[6] ^ st[10],
                st[3] ^ st[7] ^ st[11],
            ];

            let e = [
                p[3].rotate_right(27) ^ p[3].rotate_right(18),
                p[0].rotate_right(27) ^ p[0].rotate_right(18),
                p[1].rotate_right(27) ^ p[1].rotate_right(18),
                p[2].rotate_right(27) ^ p[2].rotate_right(18),
            ];

            let mut tmp = [0u32; 12];

            tmp[0] = e[0] ^ st[0] ^ round_key;
            tmp[1] = e[1] ^ st[1];
            tmp[2] = e[2] ^ st[2];
            tmp[3] = e[3] ^ st[3];

            tmp[4] = e[3] ^ st[7];
            tmp[5] = e[0] ^ st[4];
            tmp[6] = e[1] ^ st[5];
            tmp[7] = e[2] ^ st[6];

            tmp[8] = (e[0] ^ st[8]).rotate_right(21);
            tmp[9] = (e[1] ^ st[9]).rotate_right(21);
            tmp[10] = (e[2] ^ st[10]).rotate_right(21);
            tmp[11] = (e[3] ^ st[11]).rotate_right(21);

            st[0] = (!tmp[4] & tmp[8]) ^ tmp[0];
            st[1] = (!tmp[5] & tmp[9]) ^ tmp[1];
            st[2] = (!tmp[6] & tmp[10]) ^ tmp[2];
            st[3] = (!tmp[7] & tmp[11]) ^ tmp[3];

            st[4] = ((!tmp[8] & tmp[0]) ^ tmp[4]).rotate_right(31);
            st[5] = ((!tmp[9] & tmp[1]) ^ tmp[5]).rotate_right(31);
            st[6] = ((!tmp[10] & tmp[2]) ^ tmp[6]).rotate_right(31);
            st[7] = ((!tmp[11] & tmp[3]) ^ tmp[7]).rotate_right(31);

            st[8] = ((!tmp[2] & tmp[6]) ^ tmp[10]).rotate_right(24);
            st[9] = ((!tmp[3] & tmp[7]) ^ tmp[11]).rotate_right(24);
            st[10] = ((!tmp[0] & tmp[4]) ^ tmp[8]).rotate_right(24);
            st[11] = ((!tmp[1] & tmp[5]) ^ tmp[9]).rotate_right(24);
        }

        // Sync back only for Big Endian targets
        // (Little Endian targets operated in-place)
        if !cfg!(target_endian = "little") {
            for i in 0..12 {
                self.st[i * 4..i * 4 + 4].copy_from_slice(&st[i].to_le_bytes());
            }
        }
    }
}
