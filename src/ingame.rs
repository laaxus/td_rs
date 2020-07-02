use queues::*;
use crate::blocs;
use std::io::{Result};

pub fn find_path(board : &mut Vec<Vec<blocs::Bloc>>) -> Result<()> {
	let mut q: Queue<(usize,usize)> = queue![];
	let board_height = board.len();
	let board_width = board[0].len();
	
	for i in 0..board_height {
		for j in 0..board_width {
			if board[i][j].tag == blocs::BlocType::Rouge {
				q.add((i,j)).expect("Error adding in queue : find_path");
			}
			board[i][j].parent = None;
		}
	}
	
	fn add_bloc(k:usize, l:usize, i:usize, j:usize, q:&mut Queue<(usize,usize)>,board : &mut Vec<Vec<blocs::Bloc>> ) {
		q.add((k,l)).expect("Error adding in queue : find_path");
		board[k][l].parent = Some((i,j));
	}
	
	
	while let Ok((i,j)) = q.remove() {
		if i > 0 {
			if (board[i-1][j].tag == blocs::BlocType::Gris || board[i-1][j].tag == blocs::BlocType::Bleu)  && board[i-1][j].parent == None {
				add_bloc(i-1,j,i,j,&mut q,board);
			}
		}
		if i < board_height - 1 {
			if (board[i+1][j].tag == blocs::BlocType::Gris || board[i+1][j].tag == blocs::BlocType::Bleu) && board[i+1][j].parent == None {
				add_bloc(i+1,j,i,j,&mut q,board);
			}
		}if j > 0 {
			if (board[i][j-1].tag == blocs::BlocType::Gris || board[i][j-1].tag == blocs::BlocType::Bleu) && board[i][j-1].parent == None {
				add_bloc(i,j-1,i,j,&mut q,board);
			}
		}
		if j < board_width - 1 {
			if (board[i][j+1].tag == blocs::BlocType::Gris || board[i][j+1].tag == blocs::BlocType::Bleu) && board[i][j+1].parent == None {
				add_bloc(i,j+11,i,j,&mut q,board);
			}
		}
	}
	Ok(())
}
