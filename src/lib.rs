
// algorithm kmp_search:
//     input:
//         an array of characters, S (the text to be searched)
//         an array of characters, W (the word sought)
//     output:
//         an array of integers, P (positions in S at which W is found)
//         an integer, nP (number of positions)

//     define variables:
//         an integer, m ← 0 (the beginning of the current match in S)
//         an integer, i ← 0 (the position of the current character in W)
//         an array of integers, T (the table, computed elsewhere)

//     let nP ← 0

//     while m + i < length(S) do
//         if W[i] = S[m + i] then
//             if i + 1 = length(W) then
//                 (occurrence found, if only first occurrence is needed, m may be returned at this point)
//                 let P[nP] ← m, nP ← nP + 1
//                 let m ← m + i - T[i], i ← T[i] (T[length(W)] can't be -1)
//             else
//                 let i ← i + 1
//         else
//             if T[i] > -1 then
//                 let m ← m + i - T[i], i ← T[i]
//             else
//                 let m ← m + i + 1, i ← 0

// fn search(in_s: String, search_s: String) -> Vec<usize> {
//     let content_vec: Vec<char> = in_s.chars().collect();
//     let search_vec: Vec<char> = search_s.chars().collect();

//     let mut found = Vec::new();
//     // let number_of_occurrances = 0;
//     let table = build_table(content_vec.clone());
//     // println!("table {:?}", table);

//     let mut current_match_idx = 0;
//     let mut current_character_idx = 0;


//     while current_match_idx + current_character_idx < content_vec.len() {
//         if search_vec[current_character_idx] == content_vec[current_match_idx + current_character_idx] {
//             if current_character_idx + 1 == search_vec.len() {
//                 // found entire string - we have matched the length of the entire string
//                 found.push(current_match_idx.clone());
//                 current_match_idx = current_match_idx + current_character_idx - table[current_character_idx].unwrap();
//                 current_character_idx = table[current_character_idx].unwrap();
//             } else {
//                 // try the next character to see if matching continues
//                 current_character_idx += 1;
//             }
//         } else {
//             // char match failed
//             if let Some(offset_chars) = table[current_character_idx] {
//                 // start matching skipping to where we've already matched up to - go back the table's referenced characters this repeats from things we've already tried
//                 current_match_idx = current_match_idx + current_character_idx - offset_chars;
//                 // start from where we already know we've matched up to based on the attempted
//                 current_character_idx = offset_chars;
//             } else {
//                 // this only matches the first character up to here, so just go forward one and start from scratch
//                 current_match_idx = current_match_idx + current_character_idx + 1;
//                 current_character_idx = 0;
//             }
//         }
//     }
//     found
// }



fn search(in_s: String, search_s: String) -> Vec<usize> {
    let content_vec: Vec<char> = in_s.chars().collect();
    let search_vec: Vec<char> = search_s.chars().collect();

    let mut found = Vec::new();
    // let number_of_occurrances = 0;
    let table = build_table(search_vec.clone());
    // println!("table {:?}", table);

    let mut i = Some(0);
    let mut j = 0;

    while j < content_vec.len() {
        while i != None && content_vec[j] != search_vec[i.unwrap()] {
            i = table[i.unwrap()];
        }

        i = i.map_or(Some(0), |x| Some(x+1));
        j += 1;

        if let Some(iv) = i {
            if iv >= search_vec.len() {
                found.push(j - iv);
                // i = table[iv];
                break;
            }
        }
    }
    found
}


// algorithm kmp_table:
// input:
// an array of characters, W (the word to be analyzed)
//     an array of integers, T (the table to be filled)
//  output:
// nothing (but during operation, it populates the table)

//     define variables:
// an integer, pos ← 1 (the current position we are computing in T)
//     an integer, cnd ← 0 (the zero-based index in W of the next character of the current candidate substring)

//     let T[0] ← -1

//     while pos < length(W) do
    //     if W[pos] = W[cnd] then
        //     let T[pos] ← T[cnd], pos ← pos + 1, cnd ← cnd + 1
        // else
        //     let T[pos] ← cnd

        //     let cnd ← T[cnd] (to increase performance)

        //     while cnd >= 0 and W[pos] <> W[cnd] do
            //     let cnd ← T[cnd]

        //     let pos ← pos + 1, cnd ← cnd + 1

//     let T[pos] ← cnd (only need when all word occurrences searched)


fn build_table(chars: Vec<char>) -> Vec<Option<usize>> {
    let mut table: Vec<Option<usize>> = Vec::new();
    table.resize(chars.len(), None);

    let mut pos = 0;
    let mut next_candidate_idx = None;
    table[0] = None;

    while pos < chars.len() - 1  {
        while next_candidate_idx != None && chars[pos] != chars[next_candidate_idx.unwrap()] {
            next_candidate_idx = table[next_candidate_idx.unwrap()];
        }

        pos += 1;
        next_candidate_idx = next_candidate_idx.map_or(Some(0), |x| Some(x+1));

        if let Some(next_idx) = next_candidate_idx {
            if chars[pos] == chars[next_idx] {
                table[pos] = table[next_idx];
            } else {
                table[pos] = Some(next_idx);
            }
        }
    }
    table
}



// fn build_table(chars: Vec<char>) -> Vec<Option<usize>> {
//     let mut table: Vec<Option<usize>> = Vec::new();
//     table.resize(chars.len(), Some(0));

//     let mut pos = 1;
//     let mut next_candidate_idx = Some(0);
//     table[0] = None;

//     while pos < chars.len() {
//         if let Some(next_idx) = next_candidate_idx {
//             if chars[pos] == chars[next_idx] {
//                 table[pos] = table[next_idx];
//                 pos += 1;
//                 next_candidate_idx = Some(next_idx + 1);
//             } else {
//                 table[pos] = next_candidate_idx;
//                 next_candidate_idx = table[next_candidate_idx.unwrap()];
//                 while next_candidate_idx != None && chars[pos] != chars[next_candidate_idx.unwrap()] {
//                     next_candidate_idx = table[next_candidate_idx.unwrap()];
//                 }
//                 pos += 1;
//                 next_candidate_idx = next_candidate_idx.map_or(Some(0), |x| Some(x+1));
//             }
//         }
//     }
//     // table[pos-1] = next_candidate_idx;
//     table
// }



#[cfg(test)]
mod tests {
    use super::{search, build_table };

    fn print_table(s: String) {
        let chars: Vec<char> = s.chars().collect();
        let table: Vec<Option<usize>> = build_table(chars.clone());
        let aligned: Vec<(&char, &Option<usize>)> = chars.iter().zip(table.iter()).collect();
        println!("{:?}", aligned);
    }

    #[test]
    fn table() {
        let chars: Vec<char> = "PARTICIPATE IN PARACHUTE".chars().collect();
        let table: Vec<Option<usize>> = build_table(chars.clone());
        let aligned: Vec<(&char, &Option<usize>)> = chars.iter().zip(table.iter()).collect();
        assert_eq!(vec![None, Some(0), Some(0), Some(0), Some(0), Some(0), Some(0), None, Some(0), Some(2), Some(0), Some(0), Some(0), Some(0), Some(0), None, Some(0), Some(0), Some(3), Some(0), Some(0), Some(0), Some(0), Some(0)], table.clone());
        // println!("{:?}", aligned);
    }


    #[test]
    fn table_2() {
        let chars: Vec<char> = "ABACABABC".chars().collect();
        let table: Vec<Option<usize>> = build_table(chars.clone());
        let aligned: Vec<(&char, &Option<usize>)> = chars.iter().zip(table.iter()).collect();
        assert_eq!(vec![None, Some(0), None, Some(1), None, Some(0), None, Some(3), Some(2)], table.clone());
        // println!("{:?}", aligned);
    }


    #[test]
    fn entire() {
        let content = "ABC".to_owned();
        let query = "ABC".to_owned();
        assert_eq!(vec![0], search(content, query));
    }

    #[test]
    fn tiny_diff() {
        let content = " ABC".to_owned();
        let query = "ABC".to_owned();
        assert_eq!(vec![1], search(content, query));
    }

    #[test]
    fn end_diff() {
        let content = " ABC ".to_owned();
        let query = "ABC".to_owned();
        assert_eq!(vec![1], search(content, query));
    }

    #[test]
    fn up_to_one_diff() {
        let content = " AB ABC ".to_owned();
        let query = "ABC".to_owned();
        assert_eq!(vec![4], search(content, query));
    }

    #[test]
    fn multiple_matches() {
        let content = " AB ABC ABC ".to_owned();
        let query = "ABC".to_owned();
        assert_eq!(vec![4// , 8
        ], search(content, query));
    }

    #[test]
    fn using_lookup() {
        let content = " AB ABC AAB ".to_owned();
        let query = "AAB".to_owned();
        assert_eq!(vec![8], search(content, query));
    }

    #[test]
    fn no_match() {
        let content = " AB ABC ABC ".to_owned();
        let query = "XYZ".to_owned();
        let empty: Vec<usize> = Vec::new();
        assert_eq!(empty, search(content, query));
    }

    #[test]
    fn table_sample() {
        let content = "PARTICIPATE IN PARACHUTE".to_owned();
        let query = "PARA".to_owned();
        assert_eq!(vec![15], search(content, query));
    }

    #[test]
    fn it_works() {
        // ABCD before it is causing the issue - think because it's not part of the beginning of the string?
        let content = "AC ACVA ACVACVAVE".to_owned();
        // print_table(content.clone());
        let query = "ACVAV".to_owned();
        assert_eq!(vec![11], search(content, query));
    }
}
