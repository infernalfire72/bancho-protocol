use crate::serde::macros::BinaryDeserialize;
use crate::structures::ReplayFrameBundle;

#[derive(Debug, BinaryDeserialize)]
#[crate_root(crate)]
pub struct SpectateFrames {
    pub frames: ReplayFrameBundle,
}
