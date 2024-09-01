use crate::input_reader::input_list::get_vector_from_file;

pub fn run_day_2(){
    let mut sub: SubmarineMovment = SubmarineMovment::new();
    let values = get_vector_from_file(
        "src/day_2/input.txt",
        |s| {
            sub.parse_line(s);

        }
    );

    println!("Day2 - 1 product = {}",sub.get_product());

    let mut sub: SubmarineMovment = SubmarineMovment::new();
    let values = get_vector_from_file(
        "src/day_2/input.txt",
        |s| {
            sub.parse_line_part2(s);

        }
    );

    println!("Day2 - 2 product = {}",sub.get_product());

}

struct SubmarineMovment{
    aim:i32,
    depth: i32,
    horisontal: i32
}

impl SubmarineMovment{

    pub fn new() -> SubmarineMovment{
        SubmarineMovment{
            aim:0,
            depth:0,
            horisontal:0
        }
    }

    pub fn parse_line_part2(&mut self,line: &str){
        let parts:Vec::<&str> = line.split(" ").collect();
        let steps = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => {
                self.horisontal += steps;
                self.depth += self.aim * steps;
            }
            "up" => {
                self.aim -= steps;
            }
            "down" => {
                self.aim += steps
            }
            _ => {}
        }
    }
    pub fn parse_line(&mut self,line: &str){
        let parts:Vec::<&str> = line.split(" ").collect();
        let steps = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => {
                self.horisontal += steps;
            }
            "up" => {
                self.depth -= steps;
            }
            "down" => {
                self.depth += steps
            }
            _ => {}
        }
    }
    pub fn get_product(&mut self) -> i32{
        self.depth * self.horisontal
    }

}