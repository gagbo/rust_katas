use std::collections::{HashMap, VecDeque};

/// A chess position (x, y)
pub type ChessPos = (isize, isize);

/// A Chessboard, including a cached view of moves
#[derive(Debug, Clone, Default)]
pub struct ChessBoard {
    pub width: isize,
    pub height: isize,
    /// Nodes is a map from Position to (Parent position, total number of moves)
    pub nodes: HashMap<ChessPos, (ChessPos, u8)>,
}

impl ChessBoard {
    /// Initializes a new board
    pub fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    /// Return the number of moves from start_pos to dest
    ///
    /// mut because the "nodes" cache is in the structure
    pub fn bfs(&mut self, start_pos: ChessPos, dest: ChessPos) -> usize {
        // Initialization
        self.nodes.insert(start_pos, (start_pos, 0));
        let mut nodes_to_visit: VecDeque<(ChessPos, ChessPos)> = self
            .next_moves(start_pos)
            .iter()
            .cloned()
            .map(|pos| (pos, start_pos))
            .collect();
        while let Some((pos, parent)) = nodes_to_visit.pop_front() {
            // Already visited
            if self.nodes.contains_key(&pos) {
                continue;
            }

            let moves = self.nodes.get(&parent).unwrap().1;
            self.nodes.insert(pos, (parent, moves + 1));

            // Stop at destination
            if pos == dest {
                break;
            }

            // Else add all neighbours
            self.next_moves(pos)
                .iter()
                .cloned()
                .for_each(|next_pos| nodes_to_visit.push_back((next_pos, pos)))
        }

        // Print the result directly on console
        let mut cursor = dest;
        while let Some(new_cursor) = self.nodes.get(&cursor) {
            if new_cursor.1 == 0 {
                break;
            }
            println!("{:?} -> {:?}", new_cursor.0, cursor);
            cursor = new_cursor.0;
        }
        self.nodes
            .get(&dest)
            .map(|it| it.1)
            .unwrap_or_else(|| {
                println!("Impossible!!!");
                0
            })
            .into()
    }

    /// Return true if m is a valid position on the board
    pub fn is_valid(&self, m: &ChessPos) -> bool {
        m.0 > 0 && m.1 > 0 && m.0 < self.width + 1 && m.1 < self.height + 1
    }

    /// Return the valid moves from pos for this board
    ///
    /// This part is specific to knight moves.
    pub fn next_moves(&self, pos: ChessPos) -> Vec<ChessPos> {
        // The knight has a finite set of moves so it easier
        [
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
        ]
        .iter()
        .filter_map(|(dx, dy)| {
            let new_pos: ChessPos = (pos.0 + dx, pos.1 + dy);
            if self.is_valid(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        })
        .collect()
    }

}
