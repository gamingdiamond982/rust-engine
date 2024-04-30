use std::fmt;


const first_rank: u64 = 0xFF;
const second_rank: u64 = 0xFF_00;


const h_file: u64 = 0x80_80_80_80_80_80_80_80;
const g_file: u64 = 0x40_40_40_40_40_40_40_40;


const a_file: u64 = 0x01_01_01_01_01_01_01_01;
const b_file: u64 = 0x02_02_02_02_02_02_02_02;


const seventh_rank: u64 = 0xFF_00_00_00_00_00_00;
const back_rank: u64 = 0xFF_00_00_00_00_00_00_00;




fn print_bitboard(bitboard: u64) {
	println!("");
	for row in (0..8).rev() {
		print!("{} ", row+1);
		for column in 0..8 {
			let index = column + row*8;
			print!("{} ", if bitboard & (1 << index) == 0 {0} else {1} );

		}

		println!("");
	}
	println!("  A B C D E F G H");

}

const fn mask_bishop_attacks(square: u8) -> u64 {
	let mut attacks: u64 = 0;
	
	let r: i8 = (square / 8) as i8;
	let f: i8 = (square % 8) as i8;
	
	let mut tr: i8 = r+1;
	let mut tf: i8 = f+1;
	while tr <= 6 && tf <= 6 {
		attacks |= 1 << (tr*8 + tf);

		tr += 1;
		tf += 1;
	}

	tr = r-1;
	tf = f+1;
	while tr >= 1 && tf <= 6 {
		attacks |= 1 << (tr*8+tf);
		tr -= 1;
		tf += 1;
	}

	tr = r-1;
	tf = f-1;

	while tr>=1 && tf >=1 {
		attacks |= 1 << (tr*8+tf);
		tr -= 1;
		tf -= 1;
	}

	tr = r+1;
	tf = f-1;

	while tr <= 6 && tf >= 1 {
		attacks |= 1 << (tr*8+tf);
		tr += 1;
		tf -= 1;
	}	
	return attacks
	
}


const fn init_bishop_mask_table() -> [u64;64] {
	let mut result: [u64;64] = [0;64];
	let mut i: u8 = 0;
	
	while i < 64 {
		result[i as usize] = mask_bishop_attacks(i);
		i+=1;
	}
	return result;
}

const BISHOP_MASKS: [u64;64] = init_bishop_mask_table();

const fn blockers_permutation(permutation: u32, mask: u64) -> u64 {
	let mut blockers: u64 = 0;
	let mut permutation = permutation;
	let mut mask = mask;
	while permutation != 0 {
		if (permutation & 1) != 0 {
			let shift = mask.trailing_zeros();
			blockers |= 1 << shift;
		}
		
		permutation >>= 1; 
		mask &= mask-1; // remove the LSB
	}
	return blockers;
}





// fuck generating magics stole these off sebastian lague's bot - https://github.com/SebLague/Chess-Coding-Adventure/


const ROOK_SHIFTS: [u8;64] = [52, 52, 52, 52, 52, 52, 52, 52, 53, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 52, 54, 53, 53, 53, 53, 54, 53, 52, 53, 54, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52];
const ROOK_MAGICS: [u64; 64] = [468374916371625120, 18428729537625841661, 2531023729696186408, 6093370314119450896, 13830552789156493815, 16134110446239088507, 12677615322350354425, 5404321144167858432, 2111097758984580, 18428720740584907710, 17293734603602787839, 4938760079889530922, 7699325603589095390, 9078693890218258431, 578149610753690728, 9496543503900033792, 1155209038552629657, 9224076274589515780, 1835781998207181184, 509120063316431138, 16634043024132535807, 18446673631917146111, 9623686630121410312, 4648737361302392899, 738591182849868645, 1732936432546219272, 2400543327507449856, 5188164365601475096, 10414575345181196316, 1162492212166789136, 9396848738060210946, 622413200109881612, 7998357718131801918, 7719627227008073923, 16181433497662382080, 18441958655457754079, 1267153596645440, 18446726464209379263, 1214021438038606600, 4650128814733526084, 9656144899867951104, 18444421868610287615, 3695311799139303489, 10597006226145476632, 18436046904206950398, 18446726472933277663, 3458977943764860944, 39125045590687766, 9227453435446560384, 6476955465732358656, 1270314852531077632, 2882448553461416064, 11547238928203796481, 1856618300822323264, 2573991788166144, 4936544992551831040, 13690941749405253631, 15852669863439351807, 18302628748190527413, 12682135449552027479, 13830554446930287982, 18302628782487371519, 7924083509981736956, 4734295326018586370];


const BISHOP_SHIFTS: [u8; 64] = [58, 60, 59, 59, 59, 59, 60, 58, 60, 59, 59, 59, 59, 59, 59, 60, 59, 59, 57, 57, 57, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59, 60, 60, 59, 59, 59, 59, 60, 60, 58, 60, 59, 59, 59, 59, 59, 58];
const BISHOP_MAGICS: [u64; 64] = [16509839532542417919, 14391803910955204223, 1848771770702627364, 347925068195328958, 5189277761285652493, 3750937732777063343, 18429848470517967340, 17870072066711748607, 16715520087474960373, 2459353627279607168, 7061705824611107232, 8089129053103260512, 7414579821471224013, 9520647030890121554, 17142940634164625405, 9187037984654475102, 4933695867036173873, 3035992416931960321, 15052160563071165696, 5876081268917084809, 1153484746652717320, 6365855841584713735, 2463646859659644933, 1453259901463176960, 9808859429721908488, 2829141021535244552, 576619101540319252, 5804014844877275314, 4774660099383771136, 328785038479458864, 2360590652863023124, 569550314443282, 17563974527758635567, 11698101887533589556, 5764964460729992192, 6953579832080335136, 1318441160687747328, 8090717009753444376, 16751172641200572929, 5558033503209157252, 17100156536247493656, 7899286223048400564, 4845135427956654145, 2368485888099072, 2399033289953272320, 6976678428284034058, 3134241565013966284, 8661609558376259840, 17275805361393991679, 15391050065516657151, 11529206229534274423, 9876416274250600448, 16432792402597134585, 11975705497012863580, 11457135419348969979, 9763749252098620046, 16960553411078512574, 15563877356819111679, 14994736884583272463, 9441297368950544394, 14537646123432199168, 9888547162215157388, 18140215579194907366, 18374682062228545019]; 






const fn transform(blockers: u64, magic: u64, shift: u8) -> usize {
	return ((blockers * magic) >> (64-shift)) as usize;
}

fn init_bishop_attack_table() -> [[u64;512];64]{
	let mut square: u8 = 0;
	let mut result: [[u64;512];64] = [[0; 512]; 64];
	while square < 64 {
		let mask = BISHOP_MASKS[square as usize];
		let permutation_count = 1 << mask.count_ones();
		let mut i = 0;

		while i < permutation_count {
			let blockers = blockers_permutation(i, mask);
			let mut attacks: u64 = 0;
			let rank:i8 = (square/8) as i8;
			let file:i8 = (square%8) as i8;

			let mut tr = rank + 1;
			let mut tf = file + 1;
			
			while tr<=7 && tf <= 7 {
				attacks |= 1 << (tr * 8 + tf);
				if blockers & (1 << (tr * 8 + tf)) != 0 {break}
				tr += 1;
				tf += 1;
			}

			tr = rank - 1;
			tf = file + 1;
			
			while tr >= 0 && tf <= 7 {
				attacks |= 1 << (tr * 8 + tf);
				if blockers & (1 << (tr * 9 + tf))  != 0 {break}
				tr -= 1;
				tf += 1;
			}
			
			tr = rank - 1;
			tf = file - 1;

			while tr >= 0 && tf >= 0 {
				attacks |= 1 << (tr * 8 + tf);
				if blockers & (1 << (tr * 8 + tf)) != 0 {break}
				tr -= 1;
				tf -= 1;
			}

			while tr <=7 && tf >= 0 {
				attacks |= 1<< (tr * 8 + tf);
				if blockers & (1 << (tr * 8 + tf)) != 0 {break}
				tr += 1;
				tf -= 1;
			}
			print_bitboard(blockers);		
			let key = transform(blockers, BISHOP_MAGICS[square as usize], BISHOP_SHIFTS[square as usize]);
			result[square as usize][key] = attacks;
				
			i+=1;
		}

		square+=1;
	}
	return result;
}


// const BISHOP_ATTACK_TABLE: [[u64;512];64] = init_bishop_attack_table();


const fn mask_rook_attacks(square: u8) -> u64 {
	let mut attacks: u64 = 0;

	let r: u8 = square / 8;
	let f: u8 = square % 8;
	
	let mut tr = r+1;
	let mut tf = f;
	while tr <= 6 {
		attacks |= 1<< (tr*8+tf);
		tr += 1;
	}
	tr = r-1;
	tf = f;
	while tr >= 1 {
		attacks |= 1<<(tr*8+tf);
		tr -= 1;
	}

	tr = r;
	tf = f+1;
	
	while tf <= 6 {
		attacks |= 1<<(tr*8+tf);
		tf += 1;
	}

	tr = r;
	tf = f-1;
	while tf >= 1 {
		attacks |= 1<<(tr*8+tf);
		tf -= 1;
	}


	return attacks
		
}



const fn mask_pawn_attacks(square:u8, is_white: bool) -> u64 {
	let mut attacks: u64 = 0;
	let bitboard: u64 = 0 | 1 << square;
	
	if is_white {
		if a_file & bitboard == 0 {
			attacks |= bitboard << 7;
		}
		if h_file & bitboard == 0 {
			attacks |= bitboard << 9;
		}
	} else {
		if a_file & bitboard == 0 {
			attacks |= bitboard >> 9;
		}
		if h_file & bitboard == 0 {
			attacks |= bitboard >> 7
		}
	}
	
	return attacks;
}


const fn mask_pawn_moves(square: u8, is_white: bool) -> u64 {
	let mut attacks = 0;
	let bitboard: u64 = 0 | 1 << square;

	if is_white {
		attacks |= bitboard << 8;
	} else {
		attacks |= bitboard >> 8;
	}
	
	return attacks;
	
}

const fn generate_pawn_attack_table(is_white: bool) -> [u64;64] {
	let mut result: [u64;64] = [0; 64];
	let mut i: usize = 0;
	while i < 64 {
		result[i] = mask_pawn_attacks(i as u8, is_white);
		i+=1
	}
	return result;
}

const fn generate_pawn_move_table(is_white: bool) -> [u64;64] {
	let mut result: [u64;64] = [0; 64];
	let mut i: usize = 0;
	while i < 64 {
		result[i] = mask_pawn_moves(i as u8, is_white);
		i+=2
	}

	return result;
}


const WHITE_PAWN_MOVES: [u64;64] = generate_pawn_move_table(true);
const BLACK_PAWN_MOVES: [u64;64] = generate_pawn_move_table(false);

const WHITE_PAWN_ATTACKS: [u64;64] = generate_pawn_attack_table(true);
const BLACK_PAWN_ATTACKS: [u64;64] = generate_pawn_attack_table(false);


const fn mask_knight_attacks(square: u8) -> u64 {
	let mut attacks: u64 = 0;
	let bitboard: u64 = 0 | 1 << square;
	
	if first_rank & bitboard == 0 {
		if (a_file | b_file) & bitboard == 0 {
			attacks |= bitboard >> 6;
		}
		if (g_file | h_file) & bitboard == 0 {
			attacks |= bitboard >> 10;
		}

		if second_rank & bitboard == 0 {
			if a_file & bitboard == 0 {
				attacks |= bitboard >> 15;
			}

			if h_file & bitboard == 0 {
				attacks |= bitboard >> 17;
			}
		}
	}

	if back_rank & bitboard == 0 {
		if (a_file | b_file) & bitboard == 0 {
			attacks |= bitboard << 6;
		}

		if (g_file | h_file) & bitboard == 0 {
			attacks |= bitboard << 10;
		}

		if seventh_rank & bitboard == 0 {
			if h_file & bitboard == 0 {
				attacks |= bitboard << 15;
			}

			if a_file & bitboard == 0 {
				attacks |= bitboard << 17
			}
		}
	}
	return attacks;
}




const fn generate_knight_attack_table() -> [u64;64] {
	let mut result: [u64;64] = [0; 64];
	let mut i: usize = 0;
	while i < 64 {
		result[i] = mask_knight_attacks(i as u8);
		i += 1;
	}
	return result;
}

const KNIGHT_ATTACKERS: [u64;64] = generate_knight_attack_table();






const fn mask_king_attacks(square: u8) -> u64 {
	let mut attacks: u64 = 0;
	let bitboard: u64 = 0 | 1 << square;
		
	if first_rank & bitboard == 0 {
		if a_file & bitboard == 0 {
			attacks |= bitboard >> 9;
		}

		attacks |= bitboard >> 8;

		if h_file & bitboard == 0 {
			attacks |= bitboard >> 7;
		}
	}

	if a_file & bitboard == 0 {
		attacks |= bitboard >> 1;
	}

	if h_file & bitboard == 0 {
		attacks |= bitboard << 1;
	}


	if back_rank & bitboard == 0 {
		if a_file & bitboard == 0 {
			attacks  |= bitboard << 7;
		}
		attacks |= bitboard << 8;

		if h_file & bitboard == 0 {
			attacks |= bitboard << 9;
		}
	}


	return attacks;
}

const fn generate_king_attack_table() -> [u64;64] {
	let mut result: [u64;64] =  [0; 64];

	let mut i: usize = 0;
	while i< 64{
		result[i] = mask_king_attacks(i as u8);
		i+=1;
	}

	return result;
}

const KING_ATTACKERS: [u64;64] = generate_king_attack_table();




#[derive(Debug)]
enum ParseError {
	InvalidAlgebraic,
	InvalidFEN,
}

impl fmt::Display for ParseError {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			match self {
				ParseError::InvalidAlgebraic => write!(f, "Invalid Algebraic Notation"),
				ParseError::InvalidFEN => write!(f, "Invalid FEN")
			}
		}

}




fn square_from_algebraic(algebraic: &str) -> Result<u8, ParseError>{
	// 8*(num-1) + letter_as_digit
	let mut chars = algebraic.chars();
	let row: u8 = match chars.next() {
		Some('a') => 0,
		Some('b') => 1,
		Some('c') => 2,
		Some('d') => 3,
		Some('e') => 4,
		Some('f') => 5,
		Some('g') => 6,
		Some('h') => 7,
		_ => return Err(ParseError::InvalidAlgebraic),
	};

	let column: u8 = match chars.next().expect("").to_digit(10) {
		None => return Err(ParseError::InvalidAlgebraic),
		Some(a) => (a-1).try_into().unwrap(),
	};
	
	return Ok(row+8*column)
	

}

#[derive(Debug)]
struct Board {
	white_king: u64,
	black_king: u64,

	white_pawns: u64,
	black_pawns: u64,
	
	white_rooks: u64,
	black_rooks: u64,

	white_knights: u64,
	black_knights: u64,

	white_bishops: u64,
	black_bishops: u64,

	white_queens: u64,
	black_queens: u64,

	white_to_move: bool,

	en_passant_square: u8,

	white_kingside: bool,
	white_queenside: bool,

	black_kingside: bool,
	black_queenside: bool,

	halfmove_clock: u8,
	fullmove_clock: u8,
}


impl Board {
	pub fn start_pos() -> Result<Self, ParseError> {
		Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	}

	pub fn from_fen(fen: &str) -> Result<Self, ParseError> {
		let mut result = Self { white_king: 0, black_king: 0, white_pawns: 0, black_pawns: 0, white_rooks: 0, black_rooks: 0, white_knights: 0, black_knights: 0, white_bishops: 0, black_bishops: 0, white_queens: 0, black_queens: 0, white_to_move: true, en_passant_square: 64, white_kingside: false, white_queenside: false, black_kingside: false, black_queenside: false, halfmove_clock: 0, fullmove_clock: 0};
		let parts: Vec<_> = fen.split(' ').collect();
		let pieces = parts[0];
		
		let mut i: i64 = 55;

		let mut r = 0;
		for c in pieces.chars() {
			let bitboard = match c {
				'r' => &mut result.black_rooks,
				'n' => &mut result.black_knights,
				'b' => &mut result.black_bishops,
				'q' => &mut result.black_queens,
				'k' => &mut result.black_king,
				'p' => &mut result.black_pawns,

				'R' => &mut result.white_rooks,
				'N' => &mut result.white_knights,
				'B' => &mut result.white_bishops,
				'Q' => &mut result.white_queens,
				'K' => &mut result.white_king,
				'P' => &mut result.white_pawns,


				'/' => {
					i -= 17;
					&mut r
				},
				
				a => {
					i += (match a.to_digit(10) {None => return Err(ParseError::InvalidFEN), Some(i) => i } -1) as i64;
					&mut r
				}
			};

			i += 1;

			if i >= 0 {
				*bitboard |= &1 << i
			}

		}
		result.white_to_move = match parts[1] {
			"w" => true,
			"b" => false,
			_ => return Err(ParseError::InvalidFEN)
		};

		let castling = parts[2];

		let mut nocastle = false;
		for c in castling.chars() {
			let castler = match c {
				'K' => &mut result.white_kingside,
				'Q' => &mut result.white_queenside,
				'k' => &mut result.black_kingside,
				'q' => &mut result.black_queenside,
				_ => &mut nocastle
			};
			*castler = true
		}
		result.en_passant_square = match parts[3] {
			"-" => 64,
			a => match square_from_algebraic(a) {
				Err(e) => return Err(e),
				Ok(i) => i,
			}
		};

		result.halfmove_clock = match parts[4].parse() {
			Err(_e) => return Err(ParseError::InvalidFEN),
			Ok(a) => a
		};

		result.fullmove_clock = match parts[5].parse() {
			Err(_e) => return Err(ParseError::InvalidFEN),
			Ok(a) => a
		};
		
		
		

		
		return Ok(result)
	}

}





fn main() {
	let _board = Board::start_pos().expect("");
	init_bishop_attack_table();
}
