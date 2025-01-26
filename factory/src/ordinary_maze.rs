use super::game::{MazeGame, Room};

#[derive(Clone)]
pub struct OrdinaryRoom {
    id: u32,
}

impl OrdinaryRoom {
    pub fn new(id: u32) -> Self {
        Self { id}
    }
}

impl Room for OrdinaryRoom {
    fn render(&self) -> String{
        let mut output: String = "OrdinaryRoom: ".to_owned();
        output.push_str(&self.id.to_string());
        output.push_str(&" ".to_owned());
        return output;
    }
}

pub struct OrdinaryMaze {
    rooms: Vec<OrdinaryRoom>
}

impl OrdinaryMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                OrdinaryRoom::new(1),
                OrdinaryRoom::new(2)
            ]
        }
    }
}

impl MazeGame for OrdinaryMaze {
    type RoomImpl = OrdinaryRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();
        rooms
    }
}