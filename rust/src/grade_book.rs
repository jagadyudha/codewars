// complete the function so that it finds the average of the three scores passed to it and returns the letter value associated with that grade.

// Numerical Score	Letter Grade
// 90 <= score <= 100	'A'
// 80 <= score < 90	'B'
// 70 <= score < 80	'C'
// 60 <= score < 70	'D'
// 0 <= score < 60	'F'

pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let average: u16 = (s1 + s2 + s3) / 3;
    match average {
        average if average >= 90 && average <= 100 => 'A',
        average if average >= 80 && average < 90 => 'B',
        average if average >= 70 && average < 80 => 'C',
        average if average >= 60 && average < 70 => 'D',
        _ => 'F',
    }
}
