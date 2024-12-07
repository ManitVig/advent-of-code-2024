use crate::part1::check_if_safe;

fn check_with_dampner(report: &mut Vec<i32>) -> bool {
    let dampner_available = true;
    for i in 0..report.len()-2 {
        let curr = report[i];
        let next = report[i+1];
        let next2 = report[i+2];

        if !((curr < next && next < next2) || (curr > next && next > next2)) {
            if dampner_available {
                let mut report2 = report.clone();
                let mut report3 = report.clone();
                report.remove(i);
                report2.remove(i+1);
                report3.remove(i+2);
                return check_if_safe(report) || check_if_safe(&report2) || check_if_safe(&report3)
            } else {
                return false
            }
        }
        let diff = (next - curr).abs();
        if !(diff >=1 && diff <= 3) {
            if dampner_available {
                let mut report2 = report.clone();
                report.remove(i);
                report2.remove(i+1);
                return check_if_safe(report) || check_if_safe(&report2)
            } else {
                return false
            }
        }
    }
    //edge case
    let diff = (report[report.len() - 2] - report[report.len() - 1]).abs();
    if !(diff >=1 && diff <= 3) {
            if dampner_available {
                report.remove(report.len() - 1);
                return check_if_safe(report) 
            } else {
                return false
            }
    }
    return true
}

pub fn play_part2(mut reports: Vec<Vec<i32>>) -> usize {
    let total_safe = reports.iter_mut().map(|report| check_with_dampner(report))
        .filter(|checked| *checked == true).count();

    return total_safe
}
