// static BOARD_OFFSET: &'static [(i32, i32)] = &[
//     (-1, -1), (0, -1), (1, -1),
//     (-1,  0),          (1,  0),
//     (-1,  1), (0,  1), (1,  1),
// ];
//
//
// pub fn annotate(minefield: &[&str]) -> Vec<String> {
//     let height = minefield.len() as i32;
//     (0..height).map( |y| {
//         let width = minefield[y as usize].len() as i32;
//         (0..width).map( |x| {
//             if minefield[y as usize].as_bytes()[x as usize] == b'*' { '*'
//         } else {
//             match BOARD_OFFSET.iter()
//                 .map( |&(ox, oy)| (x + ox, y + oy) )
//                 .filter( |&(x,y)| (0 <= x && x < width) && (0 <= y && y < height) )
//                 .filter( |&(x,y)| minefield[y as usize].as_bytes()[x as usize] == b'*' )
//                 .count() {
//                     0 => ' ',
//                     n => (n as u8 + '0' as u8) as char
//                 }
//         }
//         }).collect()
//
//     }).collect()
// }

use std::{char::from_digit, convert::TryFrom};

const MINE_BYTE: u8 = b'*';
const MINE_CHAR: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // dbg!(minefield);
    minefield
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    MINE_CHAR => MINE_CHAR,
                    _ => match count_mines(minefield, x, y) {
                        0 => ' ',
                        x => from_digit(u32::try_from(x).unwrap(), 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect()
}

fn count_mines(minefield: &[&str], x: usize, y: usize) -> usize {
    (y.saturating_sub(1)..=y + 1)
        .filter_map(|y| minefield.get(y))
        .flat_map(|line| (x.saturating_sub(1)..=x + 1).filter_map(move |x| line.as_bytes().get(x)))
        .filter(|&&c| c == MINE_BYTE)
        .count()
}

