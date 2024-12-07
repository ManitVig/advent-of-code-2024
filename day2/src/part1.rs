pub fn check_if_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len()-2 {
        let curr = report[i];
        let next = report[i+1];
        let next2 = report[i+2];

        if !((curr < next && next < next2) || (curr > next && next > next2)) {
            return false
        }
        let diff = (next - curr).abs();
        if !(diff >=1 && diff <= 3) {
            return false
        }
    }
    //edge case
    let diff = (report[report.len() - 2] - report[report.len() - 1]).abs();
    if !(diff >=1 && diff <= 3) {
        return false
    }
    return true
}

pub fn play_part1(reports: Vec<Vec<i32>>) -> usize {
    let total_safe = reports.iter().map(|report| check_if_safe(report)).filter(|checked| *checked == true).count();

    return total_safe
}
