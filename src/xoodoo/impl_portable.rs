use super::{Xoodoo, RCON};

#[inline(always)]
fn rot(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

#[inline(always)]
fn ell(x: u32) -> u32 {
    rot(x ^ (x << 16), 16)
}

impl Xoodoo {
    pub fn permute(&mut self, steps: usize) {
        let mut state = self.to_words();
        let brans = 6;
        for i in 0..steps {
            state[1] ^= RCON[i % 8];
            state[3] ^= i as u32;

            let mut j = 0;
            while j < 2 * brans {
                let rc = RCON[j >> 1];
                state[j] = state[j].wrapping_add(rot(state[j + 1], 31));
                state[j + 1] ^= rot(state[j], 24);
                state[j] ^= rc;
                state[j] = state[j].wrapping_add(rot(state[j + 1], 17));
                state[j + 1] ^= rot(state[j], 17);
                state[j] ^= rc;
                state[j] = state[j].wrapping_add(state[j + 1]);
                state[j + 1] ^= rot(state[j], 31);
                state[j] ^= rc;
                state[j] = state[j].wrapping_add(rot(state[j + 1], 24));
                state[j + 1] ^= rot(state[j], 16);
                state[j] ^= rc;
                j += 2;
            }

            let mut tmpx = state[0];
            let x0 = state[0];
            let mut tmpy = state[1];
            let y0 = state[1];

            let mut j = 2;
            while j < brans {
                tmpx ^= state[j];
                tmpy ^= state[j + 1];
                j += 2;
            }

            tmpx = ell(tmpx);
            tmpy = ell(tmpy);

            let mut j = 2;
            while j < brans {
                let sj = state[j];
                let sjp1 = state[j + 1];
                let sjb = state[j + brans];
                let sjbp1 = state[j + brans + 1];

                state[j - 2] = sjb ^ sj ^ tmpy;
                state[j + brans] = sj;
                state[j - 1] = sjbp1 ^ sjp1 ^ tmpx;
                state[j + brans + 1] = sjp1;

                j += 2;
            }

            state[brans - 2] = state[brans] ^ x0 ^ tmpy;
            state[brans] = x0;
            state[brans - 1] = state[brans + 1] ^ y0 ^ tmpx;
            state[brans + 1] = y0;
        }
        self.init_from_words(state);
    }
}
