#[derive(Clone)]
pub struct Day1Inputs{
    day1_input: Vec<i32>,
}


impl Day1Inputs {
    fn day1_1(self) -> i32 {
        let total_fuel_reqs: i32 = self.day1_input.iter().map(|x| ((x / 3) as i32) - 2).sum();
        total_fuel_reqs
        }

    fn day1_2_fuel_calc(mass: i32) -> i32 {
        let new_mass = ((mass / 3) as i32) -2;
        new_mass
    }

    fn day1_2(self) -> i32 {
        let mut total_fuel_mass: i32 = 0;
        for mass in self.day1_input {
            let mut total_fuel_mass_for_object: i32 = 0;
            let mut fuel_req = mass;
            loop {
                fuel_req = Day1Inputs::day1_2_fuel_calc(fuel_req);
                if fuel_req <= 0 {
                    break;
                }
                total_fuel_mass_for_object = total_fuel_mass_for_object + fuel_req;
            }
            total_fuel_mass = total_fuel_mass + total_fuel_mass_for_object;
        }
        total_fuel_mass
    }
}

pub fn day_one() {
    let day1 = Day1Inputs {day1_input: vec![88093, 102524, 75875, 62024, 86072, 106670, 105440, 51371, 148951, 123704, 92364, 50848, 117125, 95022, 131085, 129886, 145084, 123077, 69219, 84366, 51344, 65604, 140383, 53606, 132685, 83550, 76648, 120937, 137498, 84167, 94438, 54178, 106306, 80802, 98524, 70214, 114108, 118782, 75444, 76449, 144233, 56747, 93663, 137969, 99981, 110442, 106873, 93708, 114085, 53655, 78096, 137640, 50775, 72563, 135043, 146136, 147244, 105601, 106293, 63048, 104864, 93044, 118222, 107110, 92725, 57424, 94602, 87898, 51668, 137651, 55070, 67255, 103823, 83059, 61150, 82029, 56060, 56702, 85486, 114522, 94121, 104870, 53014, 111776, 63615, 78378, 113830, 80059, 123427, 73545, 93688, 122410, 93174, 131464, 137014, 114304, 138703, 54128, 111698, 84299]};
    let fuel_req = day1.clone().day1_1();
    (println!("Day 1.1: {:?}", fuel_req));
    
    let total_fuel_req = day1 .day1_2();
    (println!("Day 1.2: {:?}", total_fuel_req));
}