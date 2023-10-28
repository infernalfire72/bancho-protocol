use bancho_protocol_macros::{BinaryDeserialize, BinarySerialize, ByteSized};

#[derive(Debug, BinarySerialize, BinaryDeserialize, ByteSized)]
#[crate_root(crate)]
pub struct ScoreFrame {
    pub time: i32,
    pub slot_id: u8,
    pub num300: u16,
    pub num100: u16,
    pub num50: u16,
    pub num_geki: u16,
    pub num_katu: u16,
    pub misses: u16,
    pub total_score: i32,
    pub current_combo: u16,
    pub max_combo: u16,
    pub perfect: bool,
    pub current_hp: u8,
    pub tag_byte: u8,
    pub score_v2: bool,

    #[depends(score_v2)]
    pub combo_portion: Option<f64>,
    #[depends(score_v2)]
    pub bonus_portion: Option<f64>,
}