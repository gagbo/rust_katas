use rust_katas::graph::ChessBoard;

fn main() {
    let mut board = ChessBoard::new(8, 9);
    println!("{}", board.bfs((4, 4), (5, 5)));
}
