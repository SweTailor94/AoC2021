use crate::input_reader::input_list::get_vector_from_file;

pub fn run_day_3() {
    let mut diag = DiagnosticReportInterpreter::new();
    let values = get_vector_from_file("src/day_3/input.txt", |s| {
        diag.parse_line(s);
    });

    let (g, e) = diag.calculate_gamma_and_epsilon();

    println!("Day 3 - 1 g {}, e{} product {}", g, e, g * e);

    //Part 2
}

struct DiagnosticReportInterpreter {
    ones_count: Vec<i32>,
    total_count: i32,
    numbers: Vec<i32>,
}

impl DiagnosticReportInterpreter {
    pub fn new() -> DiagnosticReportInterpreter {
        DiagnosticReportInterpreter {
            ones_count: Vec::new(),
            total_count: 0,
            numbers: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let bits = line.as_bytes();
        if self.total_count == 0 {
            for b in 0..bits.len() {
                self.ones_count.push(0);
            }
        }
        self.total_count += 1;
        let mut current = 0;
        for (i, val) in bits.iter().enumerate() {
            current = current << 1;
            if *val == '1' as u8 {
                self.ones_count[i] += 1;
                current += 1;
            }
        }
        self.numbers.push(current);
    }

    pub fn calculate_gamma_and_epsilon(&mut self) -> (i32, i32) {
        let mut gamma = 0;
        let mut epsilon = 0;
        for count in self.ones_count.iter() {
            gamma = gamma << 1;
            epsilon = epsilon << 1;
            if *count > self.total_count - *count {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        }
        (gamma, epsilon)
    }

    // pub fn calculate_part_2_values(&mut self) -> (i32, i32)
    // {
    //     let mut o_candidates = self.numbers.clone();
    //     let mut co2_candidates = self.numbers.clone();

    //     let bits = self.ones_count.len();
    //     let mask = 1 << (bits-1);
    //     for bit in 0..bits{
    //         if is_1_most_common(self.numbers,mask){
    //             o_candidates = o_candidates.drain_filter()
    //         }
    //         else{

    //         }
    //     }
    //     (o_candidates[0],co2_candidates[0])
    // }
}

fn is_1_most_common(numbers: &Vec<i32>, mask: &i32) {
    let mut ones = 0;
    let mut zeros = 0;
    for val in numbers.iter() {
        if (val & mask) > 0 {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    ones > zeros;
}
