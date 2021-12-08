type Board = [[u32; 5]; 5];

fn parse_input(input: String) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();
    let calls = lines.next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(_) = lines.next() {
        let mut board: Board = [[0; 5]; 5];
        for i in 0..5 {
            let row: Vec<u32> = lines.next()
                .unwrap()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect();
            for (j, &e) in row.iter().enumerate() {
                board[i][j] = e;     
            }
        }
        boards.push(board);
    }
    (calls, boards)

}

fn check_win(calls: &[u32], board: &Board) -> bool {
    let row_win = (0..5).map(|i| {
        (0..5)
            .map(|j| calls.contains(&board[i][j]))
            .all(|x| x)
    }).any(|x| x);

    let col_win = (0..5).map(|j| {
        (0..5)
            .map(|i| calls.contains(&board[i][j]))
            .all(|x| x)
    }).any(|x| x);

    row_win || col_win
}

fn score_win(calls: &[u32], board: &Board) -> u32 {
    let raw_score: u32 = (0..5).map(|i| {
        (0..5).map(move |j| {
            if calls.contains(&board[i][j]) {
                0
            } else {
                board[i][j]
            }
        })
    }).flatten().sum();
    raw_score * calls.last().unwrap()
}

pub fn giant_squid(input: String) {
    let (calls, boards) = parse_input(input);

    // PART 1
    for t in 0..calls.len() {
        let states: Vec<bool> = boards.iter()
            .map(|b| check_win(&calls[..t], b))
            .collect();

        if let Some(i) = states.iter().position(|&x| x) {
            let score = score_win(&calls[..t], &boards[i]);
            println!("Winning board scores {} points.", score);
            break
        }
    }

    // PART 2
    let mut losing_board = 0;
    for t in 0..calls.len() {
        let states: Vec<bool> = boards.iter()
            .map(|b| check_win(&calls[..t], b))
            .collect();

        let remaining = states.iter().filter(|&s| !s).count();
        if remaining == 1 {
            losing_board = states.iter()
                .position(|&s| !s)
                .unwrap();
        } else if remaining == 0 {
            let score = score_win(&calls[..t], &boards[losing_board]);
            println!("Losing board scores {} points.", score);
            break
        }
    }

}
