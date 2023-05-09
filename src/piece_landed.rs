use crate::{
    dot::{Dot, DotColor},
    prelude::{COLS, ROWS},
    util::map_idx,
};

pub fn get_indexes_to_remove(squares: &[Option<Dot>]) -> Vec<usize> {
    let mut indexes_to_remove: Vec<usize> = Vec::new();
    let mut current_chain: Vec<usize> = Vec::new();

    let mut chain_color: Option<DotColor> = None;
    let mut chain_count = 0;

    for x in 0..COLS {
        for y in 0..ROWS {
            let idx = map_idx(x, y);

            let color = squares[idx].as_ref().map(|dot| dot.color.clone());

            if color.is_some() && color == chain_color {
                chain_count += 1;
                current_chain.push(idx);
            } else {
                if chain_count >= 3 {
                    for i in &current_chain {
                        indexes_to_remove.push(*i);
                    }
                }

                chain_count = 0;
                chain_color = color.clone();
                current_chain.clear();
            }
        }
    }

    current_chain.clear();
    chain_count = 0;
    chain_color = None;

    for y in 0..ROWS {
        for x in 0..COLS {
            let idx = map_idx(x, y);

            let color = squares[idx].as_ref().map(|dot| dot.color.clone());

            if color.is_some() && color == chain_color {
                chain_count += 1;
                current_chain.push(idx);
            } else {
                if chain_count >= 3 {
                    for i in &current_chain {
                        indexes_to_remove.push(*i);
                    }
                }

                chain_color = color.clone();
                chain_count = 0;
                current_chain.clear();
            }
        }
    }

    indexes_to_remove
}
