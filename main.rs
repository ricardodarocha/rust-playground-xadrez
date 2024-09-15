const SIZE: usize = 8_usize;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, )]
pub struct Coord(pub usize, pub usize);

impl From<usize> for Coord {
    fn from(index: usize) -> Self {
        let a = index / SIZE;
        let b = index % SIZE;
        Coord(a, b)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TargetKind {
    Opposite,
    Same,
    Empty,
}

#[derive(Clone)]
pub enum PieceKind{
    KindPawn (Pawn),
    KindNight (Night),
    KindBishop (Bishop),
    KindRook (Rook),
    KindQueen (Queen),
    KindKing (King),
    
} 

use PieceKind::*; 

impl From<Night> for PieceKind {
   fn from(piece: Night) -> Self {
        KindNight(piece)
    }  
}
impl From<Bishop> for PieceKind {
   fn from(piece: Bishop) -> Self {
        KindBishop(piece)
    }  
}

use core::fmt;

#[derive(Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    fn new(kind: PieceKind, color: PieceColor,) -> Piece {
        Piece {
            kind,
            color
        }
    }
}

impl fmt::Display for Piece {
        
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            
            match (self.kind.clone(), self.color) {
                (_, _) =>  write!(f, "❌" ),
            }
        }
    }

impl Piece {
    fn can_move(&self, from: &Coord, to: &Coord) -> bool {
        
            match self.kind {
             PieceKind::KindPawn (_) => { Pawn::can_move(from, to) }, 
             PieceKind::KindNight (_) => { Night::can_move(from, to) }, 
             PieceKind::KindBishop (_) => { Bishop::can_move(from, to) }, 
             PieceKind::KindRook (_) => { Rook::can_move(from, to) }, 
             PieceKind::KindQueen (_) => { Queen::can_move(from, to) }, 
             PieceKind::KindKing (_) => { King::can_move(from, to) }, 
       
            }
        }
}

#[derive(Clone)]
pub enum SquareKind {
    Empty,
    SquarePiece(Piece),
}


impl From<Piece> for SquareKind {
     fn from(piece: Piece) -> Self {
        SquareKind::SquarePiece (piece)
    }
} 
    
impl fmt::Display for SquareKind {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self {
            SquareKind::Empty =>  write!(f, "⬛"),
            piece => write!(f, "❌"),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    squares: HashMap<Coord, SquareKind>,
}

impl Board {

    fn empty(&mut self) -> Self {
        for i in 0..SIZE * SIZE {
            let target = Coord::from(i);
            self.squares.entry(target).or_insert(SquareKind::Empty);
        }
        self.clone()
    }
    
    fn new() -> Self {
        let mut result = Board {squares: HashMap::new()};
        Board::empty(&mut result) 
    }
    
    fn put(&mut self, coord: Coord, piece: SquareKind) -> Self {
        self.squares.entry(coord).or_insert(piece);
        self.clone()
    }
    
    fn get(&mut self, coord: Coord) -> &SquareKind {
        match self.squares.get(&coord) {
            Some(square) => square,
            None => &SquareKind::Empty
        }
    }
    
    fn mover(&mut self, from: Coord, to: Coord ) -> bool {
        let source = self.squares.get(&from);
        match source {
            None => false,
            Some(piece) => {
                match piece {
                    SquareKind::Empty => false, 
                    SquareKind::SquarePiece(piece) => {
                        piece.can_move(&from, &to)
                        // match piece.kind { 
                        //     KindPawn (_) => Pawn::can_move(&from, &to),
                        //     KindNight (_) => Night::can_move(&from, &to),
                        //     KindBishop (_) => Bishop::can_move(&from, &to),
                        //     KindRook (_) => Rook::can_move(&from, &to),
                        //     KindQueen (_) => Queen::can_move(&from, &to),
                        //     KindKing (_) => King::can_move(&from, &to),
                        // }
                    }
                }
            }
        }
        
    }
}
    
    impl fmt::Display for Board {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_string();
        for (_, value) in &self.squares {
            result.push_str(&format!("{}", value));
            if result.len() % 8 == 0 {result.push_str(&format!("\n"))};
        }
        write!(f, "{}", result)
    }
}

//walking strategies

#[derive(Clone)]
pub struct Pawn {}

#[derive(Clone)]
pub struct Night {}

#[derive(Clone)]
pub struct Bishop {}

#[derive(Clone)]
pub struct Rook {}

#[derive(Clone)]
pub struct Queen {}

#[derive(Clone)]
pub struct King {}

#[allow(dead_code)]
#[allow(unused_variables)]
trait Movable {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        true
    }

    fn available_target(source: &Coord) -> impl Iterator<Item = Coord> {
        let mut jumps = Vec::new();
        for i in 0..SIZE * SIZE {
            let target = Coord::from(i);
            if Self::can_move(&source, &target) {
                jumps.push(target)
            }
        }
        jumps.into_iter()
    }
}

impl Movable for Pawn {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        let Coord(lin_orig, col_orig) = from;
        let Coord(lin_dest, col_dest) = to;

        let primeira_fila = lin_orig >= &1_usize;
        let andou_1 = *lin_dest == lin_orig + &1_usize;
        let andou_2 = lin_orig == &1_usize && lin_dest == &3_usize;
        let mesma_col = col_orig == col_dest;
        let uma_col = ((col_orig - col_dest) as i32).abs() == 1;

        //valid row
        (primeira_fila && andou_1 || andou_2 && mesma_col )
            &&
            //valid col
            (mesma_col || uma_col )
    }
}

impl Movable for Night {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        let Coord(lin_orig, col_orig) = from;
        let Coord(lin_dest, col_dest) = to;
        let delta_x = ((col_orig - col_dest) as i32).abs();
        let delta_y = ((lin_orig - lin_dest) as i32).abs();

        delta_x + delta_y == 2 + 1 && delta_x * delta_y == 2 * 1
    }
}

impl Movable for Bishop {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        let Coord(lin_orig, col_orig) = from;
        let Coord(lin_dest, col_dest) = to;
        let delta_x = ((col_orig - col_dest) as i32).abs();
        let delta_y = ((lin_orig - lin_dest) as i32).abs();

        delta_x == delta_y
    }
}
impl Movable for Rook {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        let Coord(lin_orig, col_orig) = from;
        let Coord(lin_dest, col_dest) = to;
        lin_orig == lin_dest || col_orig == col_dest
    }
}

impl Movable for Queen {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        Bishop::can_move(&from, &to) || Rook::can_move(&from, &to)
    }
}

impl Movable for King {
    fn can_move(from: &Coord, to: &Coord) -> bool {
        let Coord(lin_orig, col_orig) = from;
        let Coord(lin_dest, col_dest) = to;
        let delta_x = ((col_orig - col_dest) as i32).abs();
        let delta_y = ((lin_orig - lin_dest) as i32).abs();

        delta_x <= 1 && delta_y <= 1
    }
}



/// macro


use std::str::FromStr;

#[derive(Debug)]
struct Fen {
    casas: Vec<char>,
    active_color: char,
    castling: String,
    en_passant: String,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl FromStr for Fen {
    type Err = String;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Formato FEN inválido".to_string());
        }

        let board_part = parts[0];
        let active_color = parts[1].chars().next().ok_or("Cor ativa inválida")?;
        let castling = parts[2].to_string();
        let en_passant = parts[3].to_string();
        let halfmove_clock = parts[4].parse::<u32>().map_err(|_| "Erro no meio-movimento".to_string())?;
        let fullmove_number = parts[5].parse::<u32>().map_err(|_| "Erro no número de jogadas".to_string())?;

        // Parse da disposição das peças no tabuleiro
        let mut casas = Vec::with_capacity(64);
        for row in board_part.split('/') {
            for c in row.chars() {
                if c.is_digit(10) {
                    let empty_spaces = c.to_digit(10).unwrap();
                    // Preenche as casas vazias com ' '
                    for _ in 0..empty_spaces {
                        casas.push(' ');
                    }
                } else {
                    // Adiciona as peças ao vetor
                    casas.push(c);
                }
            }
        }

        if casas.len() != 64 {
            return Err("Tabuleiro FEN inválido".to_string());
        }

        Ok(Fen {
            casas,
            active_color,
            castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
}

// Macro para encapsular o parsing e retornar a struct Fen
macro_rules! parse_fen {
    ($fen:expr) => {{
        let fen_str = $fen;
        match fen_str.parse::<Fen>() {
            Ok(fen) => fen,
            Err(e) => panic!("Erro ao parsear a FEN: {}", e),
        }
    }};
}


/// macro 2

use regex::Regex;

#[derive(Debug)]
struct Lance(String, String);

#[derive(Debug)]
struct Game {
    fields: HashMap<String, String>,
    lances: Vec<Lance>,
}

macro_rules! Pgn {
    ($pgn:expr) => {{
        let pgn_text = $pgn;

        // Regex para capturar campos do cabeçalho
        let header_regex = Regex::new(r#"\[(\w+)\s+"([^"]+)"\]"#).unwrap();
        // Regex para capturar os lances (movimentos brancas e pretas)
        let moves_regex = Regex::new(r#"(\d+)\.\s+([^\s]+)\s+([^\s]+)"#).unwrap();

        let mut fields = HashMap::new();
        let mut lances = Vec::new();

        // Parse do cabeçalho
        for caps in header_regex.captures_iter(pgn_text) {
            let key = caps[1].to_string();
            let value = caps[2].to_string();
            fields.insert(key, value);
        }

        // Parse dos lances
        for caps in moves_regex.captures_iter(pgn_text) {
            let white_move = caps[2].to_string();
            let black_move = caps[3].to_string();
            lances.push(Lance(white_move, black_move));
        }

        Game { fields, lances }
    }};
}

#[cfg(test)]
mod tests {
    use super::Coord;

    //Converte coordenadas (x, y) em id e vice versa
    //sendo id = 0..63, coord = (0, 0)  .. (7, 7)
    
    //Coord podem ser úteis para cálculos em determinadas posições, id em outras
    #[test]
    fn it_works() {
        assert!(Coord::from(0) == Coord(0, 0));
        assert!(Coord::from(1) == Coord(0, 1));
        assert!(Coord::from(2) == Coord(0, 2));
        assert!(Coord::from(3) == Coord(0, 3));
        assert!(Coord::from(4) == Coord(0, 4));
        assert!(Coord::from(5) == Coord(0, 5));
        assert!(Coord::from(6) == Coord(0, 6));
        assert!(Coord::from(7) == Coord(0, 7));
        assert!(Coord::from(8) == Coord(1, 0));
        assert!(Coord::from(9) == Coord(1, 1));
        assert!(Coord::from(10) == Coord(1, 2));
        assert!(Coord::from(11) == Coord(1, 3));
        assert!(Coord::from(12) == Coord(1, 4));
        assert!(Coord::from(13) == Coord(1, 5));
        assert!(Coord::from(14) == Coord(1, 6));
    }
}

#[allow(dead_code)]
fn main() {

use PieceColor::*;

// #[derive(Clone)]
// pub enum SquareKind {
//     Empty,
//     SquarePiece(Piece),
// }

// #[derive(Clone)]
// pub struct Piece {
//     pub kind: PieceKind,
//     pub color: PieceColor,
// }

// impl Piece {
//     fn new(kind: PieceKind, color: PieceColor,) -> Piece {
//         Piece {
//             kind,
//             color
//         }
//     }
// }

// #[derive(Clone)]
// pub enum PieceKind{
//     KindPawn (Pawn),
//     KindNight (Night),
//     KindBishop (Bishop),
//     KindRook (Rook),
//     KindQueen (Queen),
//     KindKing (King),
    
// }

    let fen = parse_fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // Exibe a struct Fen parseada
    println!("{:?}", fen);

    // Exibe o tabuleiro linha por linha
    println!("Tabuleiro:");
    for row in fen.casas.chunks(8) {
        for casa in row {
            print!("{}", casa);
        }
        println!();
    }



//// exemplo 2 Parse pGN

  let pgn_text = r#"
    [Event "F/S Return Match"]
    [Site "Belgrade, Serbia JUG"]
    [Date "1992.11.04"]
    [Round "29"]
    [White "Fischer, Robert J."]
    [Black "Spassky, Boris V."]
    [Result "1/2-1/2"]

    1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 
    9. h3 Nb8 10. d4 Nbd7 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15.Nb1 h6 
    16. Bh4 c5 17. dxe5 Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21.Nc4 Nxc4 
    22. Bxc4 Nb6 23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 
    28. Qxg5 hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33.f3 Bc8 34. Kf2 Bf5 
    35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6 
    Nf2 42. g4 Bd3 43. Re6 1/2-1/2
    "#;

    let game = Pgn!(pgn_text);

    // Exibe os campos do cabeçalho
    println!("Campos:");
    for (key, value) in &game.fields {
        println!("{}: {}", key, value);
    }

    // Exibe os lances
    println!("\nLances:");
    for (i, Lance(white, black)) in game.lances.into_iter().enumerate() {
        println!("{}. {} {} ", i + 1, white, black);
    }
  
}
