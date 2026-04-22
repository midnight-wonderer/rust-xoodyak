use super::{SparkleP, RCON};

#[cfg(all(target_arch = "arm", target_has_atomic = "32"))]
impl SparkleP {
    #[allow(clippy::many_single_char_names)]
    pub fn permute(&mut self, steps: usize) {
        use core::arch::asm;
        let st_ptr = self.st.as_mut_ptr() as *mut u32;
        let rcon_ptr = RCON.as_ptr();

        unsafe {
            asm!(
                "push {{r4-r6, r8-r11, lr}}",
                "mov r12, r0", // st_ptr
                "mov lr, r1",  // steps
                "push {{r2, r12, lr}}", // [sp, #0]=rcon_ptr, [sp, #4]=st_ptr, [sp, #8]=steps
                "sub sp, sp, #48",      // [sp, #0]=i, [sp, #4..19]=x4,y4,x5,y5, [sp, #20..43]=shuffle_scratch
                "mov r12, #0",
                "str r12, [sp, #0]",

                // Load initial state
                "ldr r12, [sp, #52]", // st_ptr
                "ldm r12!, {{r0-r5, r8-r11}}", // x0..y2, x3..y3 (r8, r9), x4..y4 (r10, r11)
                "ldrd r6, lr, [r12]", // x5, y5
                "strd r6, lr, [sp, #12]", // [sp, #12]=x5, [sp, #16]=y5
                "strd r10, r11, [sp, #4]", // [sp, #4]=x4, [sp, #8]=y4

                "2:", // .Lround_loop
                "ldr r12, [sp, #0]",
                "ldr lr, [sp, #56]", // steps
                "cmp r12, lr",
                "beq 4f",

                // state[1] ^= RCON[i % 8]
                "ldr lr, [sp, #48]", // rcon_ptr
                "and r12, r12, #7",
                "ldr r12, [lr, r12, lsl #2]",
                "eors r1, r1, r12",
                // state[3] ^= i
                "ldr r12, [sp, #0]",
                "eors r3, r3, r12",

                // === Alzette rounds ===
                "ldr lr, [sp, #48]", // rcon_ptr

                // Br 0 (r0, r1)
                "ldr r12, [lr, #0]",
                "add r0, r0, r1, ror #31", "eor r1, r1, r0, ror #24", "eor r0, r0, r12",
                "add r0, r0, r1, ror #17", "eor r1, r1, r0, ror #17", "eor r0, r0, r12",
                "add r0, r0, r1",           "eor r1, r1, r0, ror #31", "eor r0, r0, r12",
                "add r0, r0, r1, ror #24", "eor r1, r1, r0, ror #16", "eor r0, r0, r12",

                // Br 1 (r2, r3)
                "ldr r12, [lr, #4]",
                "add r2, r2, r3, ror #31", "eor r3, r3, r2, ror #24", "eor r2, r2, r12",
                "add r2, r2, r3, ror #17", "eor r3, r3, r2, ror #17", "eor r2, r2, r12",
                "add r2, r2, r3",           "eor r3, r3, r2, ror #31", "eor r2, r2, r12",
                "add r2, r2, r3, ror #24", "eor r3, r3, r2, ror #16", "eor r2, r2, r12",

                // Br 2 (r4, r5)
                "ldr r12, [lr, #8]",
                "add r4, r4, r5, ror #31", "eor r5, r5, r4, ror #24", "eor r4, r4, r12",
                "add r4, r4, r5, ror #17", "eor r5, r5, r4, ror #17", "eor r4, r4, r12",
                "add r4, r4, r5",           "eor r5, r5, r4, ror #31", "eor r4, r4, r12",
                "add r4, r4, r5, ror #24", "eor r5, r5, r4, ror #16", "eor r4, r4, r12",

                // Br 3 (r8, r9)
                "ldr r12, [lr, #12]",
                "add r8, r8, r9, ror #31", "eor r9, r9, r8, ror #24", "eor r8, r8, r12",
                "add r8, r8, r9, ror #17", "eor r9, r9, r8, ror #17", "eor r8, r8, r12",
                "add r8, r8, r9",           "eor r9, r9, r8, ror #31", "eor r8, r8, r12",
                "add r8, r8, r9, ror #24", "eor r9, r9, r8, ror #16", "eor r8, r8, r12",

                // Br 4 (stack)
                "ldrd r6, r10, [sp, #4]", // x4, y4
                "ldr r12, [lr, #16]",
                "add r6, r6, r10, ror #31", "eor r10, r10, r6, ror #24", "eor r6, r6, r12",
                "add r6, r6, r10, ror #17", "eor r10, r10, r6, ror #17", "eor r6, r6, r12",
                "add r6, r6, r10",           "eor r10, r10, r6, ror #31", "eor r6, r6, r12",
                "add r6, r6, r10, ror #24", "eor r10, r10, r6, ror #16", "eor r6, r6, r12",
                "strd r6, r10, [sp, #4]",

                // Br 5 (stack)
                "ldrd r6, r10, [sp, #12]", // x5, y5
                "ldr r12, [lr, #20]",
                "add r6, r6, r10, ror #31", "eor r10, r10, r6, ror #24", "eor r6, r6, r12",
                "add r6, r6, r10, ror #17", "eor r10, r10, r6, ror #17", "eor r6, r6, r12",
                "add r6, r6, r10",           "eor r10, r10, r6, ror #31", "eor r6, r6, r12",
                "add r6, r6, r10, ror #24", "eor r10, r10, r6, ror #16", "eor r6, r6, r12",
                "strd r6, r10, [sp, #12]",

                // === Linear Layer ===
                // tmpx = x0 ^ x1 ^ x2 = r0 ^ r2 ^ r4
                "eors r12, r0, r2", "eors r12, r12, r4",
                "eor lr, r12, r12, lsl #16", "ror r12, lr, #16", // r12 = tmpx_ell
                // tmpy = y0 ^ y1 ^ y2 = r1 ^ r3 ^ r5
                "eors lr, r1, r3", "eors lr, lr, r5",
                "eor lr, lr, lr, lsl #16", "ror lr, lr, #16", // lr = tmpy_ell

                "add r6, sp, #20", "stm r6, {{r0-r5}}", // save old x0..y2 to [sp, #20..43]

                // Shuffle logic
                // x0' = x4 ^ x1 ^ tmpy
                "ldr r6, [sp, #4]", "eors r0, r6, r2", "eors r0, r0, lr",
                // y0' = y4 ^ y1 ^ tmpx
                "ldr r6, [sp, #8]", "eors r1, r6, r3", "eors r1, r1, r12",
                // x1' = x5 ^ x2 ^ tmpy
                "ldr r6, [sp, #12]", "eors r2, r6, r4", "eors r2, r2, lr",
                // y1' = y5 ^ y2 ^ tmpx
                "ldr r6, [sp, #16]", "eors r3, r6, r5", "eors r3, r3, r12",
                // x2' = x3 ^ x0_old ^ tmpy
                "ldr r6, [sp, #20]", "eors r4, r8, r6", "eors r4, r4, lr",
                // y2' = y3 ^ y0_old ^ tmpx
                "ldr r6, [sp, #24]", "eors r5, r9, r6", "eors r5, r5, r12",

                "ldr r8, [sp, #20]", "ldr r9, [sp, #24]", // x3' = x0_old, y3' = y0_old
                "ldr r6, [sp, #28]", "str r6, [sp, #4]",  // x4' = x1_old
                "ldr r6, [sp, #32]", "str r6, [sp, #8]",  // y4' = y1_old
                "ldr r6, [sp, #36]", "str r6, [sp, #12]", // x5' = x2_old
                "ldr r6, [sp, #40]", "str r6, [sp, #16]", // y5' = y2_old

                "ldr r12, [sp, #0]", "adds r12, #1", "str r12, [sp, #0]",
                "b 2b",

                "4:",
                "add sp, sp, #48",
                "pop {{r2, r12, lr}}", // r2=rcon_ptr, r12=st_ptr, lr=steps
                "stm r12!, {{r0-r5}}", // Write x0..y2
                "stm r12!, {{r8-r9}}", // Write x3, y3
                "ldr r0, [sp, #-44]", "ldr r1, [sp, #-40]", // x4, y4 from [sp, #4], [sp, #8]
                "stm r12!, {{r0-r1}}",
                "ldr r0, [sp, #-36]", "ldr r1, [sp, #-32]", // x5, y5 from [sp, #12], [sp, #16]
                "stm r12!, {{r0-r1}}",
                "pop {{r4-r6, r8-r11, lr}}",

                in("r0") st_ptr,
                in("r1") steps,
                in("r2") rcon_ptr,
                out("r3") _,
                out("r12") _,
            );
        }
    }
}
