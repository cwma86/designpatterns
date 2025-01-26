pub trait Room {
    fn render(&self) -> String;
}

pub trait MazeGame {
    type RoomImpl: Room;

    fn rooms(&self) -> Vec<Self::RoomImpl>;

    fn play(&self) -> String {
        let mut output: String = "".to_owned();
        for room in self.rooms() {
            output.push_str(&room.render());
        }
        return output;
    }
}

pub fn run (maze_game: impl MazeGame) -> String {
    let mut output: String = "".to_owned();
    output.push_str("Loading resource. ");
    output.push_str("Starting Game. ");
    output.push_str(&maze_game.play());
    return output;
}