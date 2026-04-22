use super::internal::{Mode, Phase};
use super::*;

#[derive(Clone, Debug)]
pub struct XoosparkHash {
    state: SparkleP,
    phase: Phase,
}

impl XoosparkHash {
    pub fn new() -> Self {
        XoosparkHash {
            state: SparkleP::default(),
            phase: Phase::Up,
        }
    }
}

impl Default for XoosparkHash {
    #[inline]
    fn default() -> Self {
        XoosparkHash::new()
    }
}

impl internal::XoosparkCommon for XoosparkHash {
    #[inline(always)]
    fn state(&mut self) -> &mut SparkleP {
        &mut self.state
    }

    #[inline(always)]
    fn mode(&self) -> Mode {
        Mode::Hash
    }

    #[inline(always)]
    fn phase(&self) -> Phase {
        self.phase
    }

    #[inline(always)]
    fn set_phase(&mut self, phase: Phase) {
        self.phase = phase
    }

    #[inline(always)]
    fn absorb_rate(&self) -> usize {
        HASH_ABSORB_RATE
    }

    #[inline(always)]
    fn squeeze_rate(&self) -> usize {
        HASH_SQUEEZE_RATE
    }
}

impl XoosparkCommon for XoosparkHash {}
