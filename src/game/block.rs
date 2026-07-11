#[derive(Copy, Clone)]
pub enum Block {
    Purple,
    Orange,
    Green,
    PaleBlue,
    Gold,
    DarkBlue,
    Pink
}

pub fn sprite(block: &Block) -> &'static str {
    match block {
        Block::Purple => "purple",
        Block::Orange => "orange",
        Block::Green => "green",
        Block::PaleBlue => "pale_blue",
        Block::Gold => "gold",
        Block::DarkBlue => "dark_blue",
        Block::Pink => "pink"
    }
}