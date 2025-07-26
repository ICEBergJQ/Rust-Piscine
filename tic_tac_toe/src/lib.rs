pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut players: Vec<char> = Vec::new();
    
    for row in &table {
        for c in row {
            players.push(*c)
        }
    }

    for &player in &players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            return format!("player {} won", player);
        }
    }

    "tie".to_string()
}

pub fn diagonals(player: char, table: &[[char; 3]; 3]) -> bool {
    let mut main_diag = (0..3).all(|i| table[i][i] == player);
    let mut anti_diag = (0..3).all(|i| table[i][2 - i] == player);
    main_diag || anti_diag
}

pub fn horizontal(player: char, table: &[[char; 3]; 3]) -> bool {
    table.iter().any(|row| row.iter().all(|&c| c == player))
}

pub fn vertical(player: char, table: &[[char; 3]; 3]) -> bool {
    (0..3).any(|col| (0..3).all(|row| table[row][col] == player))
}
