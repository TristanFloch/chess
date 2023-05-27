use chess::engine;

fn perft(board: engine::board::Board, depth: u32) -> usize {
    let moves = board.generate_legal_moves();

    if depth == 1 {
        return moves.len();
    }

    let mut nodes = 0;
    for m in moves {
        let mut bis = board.clone();
        bis.do_move(&m);
        nodes += perft(bis, depth - 1);
    }

    nodes
}

#[ignore = "Move generation with blockers not yet implemented"]
#[test]
fn initial_pos() {
    let b = engine::board::Board::new();
    assert_eq!(perft(b.clone(), 1), 20);
    assert_eq!(perft(b.clone(), 2), 400);
    assert_eq!(perft(b.clone(), 3), 8902);
    assert_eq!(perft(b.clone(), 4), 197281);
}
