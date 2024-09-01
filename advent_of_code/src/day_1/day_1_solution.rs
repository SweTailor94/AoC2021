use crate::input_reader::input_list::get_vector_from_file;


    pub fn run_day_1() {
        let values = get_vector_from_file(
            "src/day_1/input.txt",
            |s| s.parse::<i32>().unwrap()
        );

        // part one, count higher than last value
        let mut last = values[0];
        let mut count = 0;

        for val in values.iter().skip(1) {
            if val > &last {
                count += 1;
            }
            last = *val;
        }

        println!("Day 1 - 1 count is {}", count);

        // part two

        let mut counter: SlidingWindow = SlidingWindow::new();
        for i in values{
            counter.add_value(i);
        }
        println!("Day 1 - 2 count is {}", counter.get_count())
    }

enum SlidingWindowState {
    Start,
    OneValue,
    TwoValues,
    ThreeValues
}

    struct SlidingWindow
    {
        state: SlidingWindowState,
        sum_a:i32,
        sum_b:i32,
        sum_c:i32,
        sum_oldest:i32,
        count_increasing:i32
    }

impl SlidingWindow {
    fn new() -> SlidingWindow
    {
        SlidingWindow {state: SlidingWindowState::Start, sum_a: 0, sum_b: 0, sum_c: 0, sum_oldest: 0, count_increasing: 0 }
    }

    fn add_value(&mut self, val:i32)
    {
        match self.state {
            SlidingWindowState::Start =>{
                self.sum_a += val;
                self.state = SlidingWindowState::OneValue;
            }
            SlidingWindowState::OneValue => {
                self.sum_a += val;
                self.sum_b += val;
                self.state = SlidingWindowState::TwoValues;
            }
            SlidingWindowState::TwoValues => {
                self.sum_a += val;
                self.sum_b += val;
                self.sum_c += val;
                self.state = SlidingWindowState::ThreeValues
            }
            SlidingWindowState::ThreeValues => {
                self.sum_oldest  = self.sum_a;
                self.sum_a = self.sum_b;
                self.sum_b = self.sum_c;
                self.sum_a += val;
                self.sum_b += val;
                self.sum_c = val;
                if self.sum_a > self.sum_oldest {
                    self.count_increasing += 1;
                }
            }
        }
    }

    pub fn get_count(&self) -> i32 {
        self.count_increasing
    }
}