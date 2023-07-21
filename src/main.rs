// 2023 © Miguel Gargallo. All rights reserved.

use std::io; // Importamos las funciones de E/S

const SIZE: usize = 3; // Definimos el tamaño del tablero

type Board = [[char; SIZE]; SIZE]; // Nuestro tablero será una matriz 3x3

fn main() {
    // Creamos un tablero vacío
    let mut board: Board = [['.'; SIZE]; SIZE];
    // Definimos nuestros jugadores, 'X' y 'O'.
    let players = ['X', 'O'];
    // Cada jugador hace su movimiento por turnos
    for i in 0.. {
        print_board(&board); // Mostramos el tablero
        let player = players[i % players.len()]; // Seleccionamos el jugador
        let (row, col) = request_move(player); // El jugador hace su movimiento
        board[row][col] = player; // Actualizamos el tablero
        // Verificamos si hay un ganador
        if check_winner(player, &board) {
            print_board(&board);
            // Anunciamos al ganador
            println!("¡El jugador {} gana!", player);
            break;
        }
    }
}

// Esta función imprime el tablero en la consola
fn print_board(board: &Board) {
    println!("  1 2 3");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", (b'A' + (i as u8)) as char);
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// Esta función solicita al jugador su movimiento
fn request_move(player: char) -> (usize, usize) {
    loop {
        println!("Jugador {}, introduzca su movimiento (fila y número de columna como 'A1'): ", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la línea");
        let input = input.trim().to_uppercase();
        let chars: Vec<char> = input.chars().collect();
        if let (Some(&row_char), Some(&col_char)) = (chars.get(0), chars.get(1)) {
            if let Some(row) = (row_char as u8).checked_sub(b'A') {
                if let Some(col) = (col_char as u8).checked_sub(b'1') {
                    if row < (SIZE as u8) && col < (SIZE as u8) {
                        return (row as usize, col as usize);
                    }
                }
            }
        }
        println!("Movimiento inválido. Inténtalo de nuevo.");
    }
}

// Esta función verifica si hay un ganador
fn check_winner(player: char, board: &Board) -> bool {
    let win_row = board.iter().any(|row| row.iter().all(|&cell| cell == player));
    let win_col = (0..SIZE).any(|col| board.iter().all(|row| row[col] == player));
    let win_diag1 = (0..SIZE).all(|i| board[i][i] == player);
    let win_diag2 = (0..SIZE).all(|i| board[i][SIZE - 1 - i] == player);
    win_row || win_col || win_diag1 || win_diag2
}
