use super::game::{MazeGame, Room};

#[derive(Clone)]
pub struct MagicRoom{
    title: String
}

impl MagicRoom{
    pub fn new(title: String) -> Self {
        Self {title}
    }
}

impl Room for MagicRoom {
    fn render(&self) -> String{
        let mut output: String = "Magic Room: ".to_owned();
        output.push_str(&self.title);
        output.push_str(&" ".to_owned());
        return output;
    }
}

pub struct MagicMaze {
    rooms: Vec<MagicRoom>
}

impl MagicMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                MagicRoom::new("Infinite Room". into()),
                MagicRoom::new("Red Room".into())
            ]
        }
    }
}

impl MazeGame for MagicMaze {
    type RoomImpl = MagicRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        self.rooms.clone()
    }
}