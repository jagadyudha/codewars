pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut positif = 0;
    let mut negatif = 0;
    for i in input.iter() {
        if i > &0 {
            positif = positif + 1;
        } else {
            negatif = negatif + i;
        }
    }
    if input.len() == 0 {
        return [].to_vec();
    } else {
        return [positif, negatif].to_vec();
    }
}
