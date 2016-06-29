enum GameElement {
    Empty,
    Snake{isHeader: bool},
    Food,
}

type Canvas = Vec<Vec<GameElement>>;

pub struct Game {
    main_panel: Canvas,
}

impl Game {
    pub fn new() -> Self {
        let width = 10;
        let height = 16;

        Game {
            main_panel: vec![vec![GameElement::Empty; width]; height],
        }
    }

    fn main_panel(&self) -> Canvas {
        self.main_panel
    }
}
