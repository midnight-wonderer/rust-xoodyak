use super::{Xoodoo, ROUND_KEYS};
use core::arch::asm;
use core::convert::TryInto;

#[cfg(all(target_arch = "arm", not(target_feature = "thumb2")))]
impl Xoodoo {
    #[inline(always)]
    fn ror(val: u32, shift: u32) -> u32 {
        let out: u32;
        unsafe {
            asm!(
                "rors {0}, {1}",
                inout(reg) val => out,
                in(reg) shift,
            );
        }
        out
    }

    pub fn permute(&mut self) {
        let mut st = [0u32; 12];
        for i in 0..12 {
            st[i] = u32::from_le_bytes(self.st[i * 4..i * 4 + 4].try_into().unwrap());
        }

        for &round_key in &ROUND_KEYS {
            let p = [
                st[0] ^ st[4] ^ st[8],
                st[1] ^ st[5] ^ st[9],
                st[2] ^ st[6] ^ st[10],
                st[3] ^ st[7] ^ st[11],
            ];

            let e = [
                Self::ror(p[3], 27) ^ Self::ror(p[3], 18),
                Self::ror(p[0], 27) ^ Self::ror(p[0], 18),
                Self::ror(p[1], 27) ^ Self::ror(p[1], 18),
                Self::ror(p[2], 27) ^ Self::ror(p[2], 18),
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

            tmp[8] = Self::ror(e[0] ^ st[8], 21);
            tmp[9] = Self::ror(e[1] ^ st[9], 21);
            tmp[10] = Self::ror(e[2] ^ st[10], 21);
            tmp[11] = Self::ror(e[3] ^ st[11], 21);

            st[0] = (!tmp[4] & tmp[8]) ^ tmp[0];
            st[1] = (!tmp[5] & tmp[9]) ^ tmp[1];
            st[2] = (!tmp[6] & tmp[10]) ^ tmp[2];
            st[3] = (!tmp[7] & tmp[11]) ^ tmp[3];

            st[4] = Self::ror((!tmp[8] & tmp[0]) ^ tmp[4], 31);
            st[5] = Self::ror((!tmp[9] & tmp[1]) ^ tmp[5], 31);
            st[6] = Self::ror((!tmp[10] & tmp[2]) ^ tmp[6], 31);
            st[7] = Self::ror((!tmp[11] & tmp[3]) ^ tmp[7], 31);

            st[8] = Self::ror((!tmp[2] & tmp[6]) ^ tmp[10], 24);
            st[9] = Self::ror((!tmp[3] & tmp[7]) ^ tmp[11], 24);
            st[10] = Self::ror((!tmp[0] & tmp[4]) ^ tmp[8], 24);
            st[11] = Self::ror((!tmp[1] & tmp[5]) ^ tmp[9], 24);
        }

        for i in 0..12 {
            self.st[i * 4..i * 4 + 4].copy_from_slice(&st[i].to_le_bytes());
        }
    }
}
