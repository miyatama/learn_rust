#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellValue {
    // 未入力
    None,

    // User
    Player,

    // AI
    Enemy,
}
