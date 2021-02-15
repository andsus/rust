use std::cmp;
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
/*
    0  1  2
  |---------
0 | 9  8  7
1 | 5  3  2     <--- saddle point at (1,0)
2 | 6  6  7
*/
    
    // let mut result : Vec<(usize,usize)> = Vec::new();
    // let row_len = input.len();


    // for (row_idx, row) in input.iter().enumerate() {
    //     for (col_idx, cell) in row.iter().enumerate() {
    //         // find max cell among row, and min among col
    //         if row.iter().all( |it| it <= cell)
    //             && (0..row_len).all( |it| input[it][col_idx] >= *cell ) {
    //                 result.push((row_idx,col_idx)); 
    //             }
    //     }
    // }

    // result



    input
    .iter()
    .enumerate()
    .flat_map(|(ix, row)| {
        let row_max = row.iter().max().unwrap_or(&u64::max_value());
        row.iter()
            .enumerate()
            .filter_map(|(j, elem)| {
                let col_min =
                    (0..input.len()).fold(row_max, |acc, curr| cmp::min(acc, &input[curr][j]));
                match elem == row_max && elem == col_min {
                    true => Some((ix, j)),
                    false => None,
                }
            })
            .collect::<Vec<(usize, usize)>>()
    })
    .collect()
}
