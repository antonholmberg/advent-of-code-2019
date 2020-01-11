pub trait Module {
    fn fuel_consumption(&self) -> i32;
    fn fuel_consumption_including_fuel(&self) -> i32;
}

impl Module for i32 {
    fn fuel_consumption(&self) -> i32 {
        let result = self / 3 - 2;

        if result < 0 {
            return 0;
        }

        result
    }

    fn fuel_consumption_including_fuel(&self) -> i32 {
        let mut current_fuel = self.fuel_consumption();
        let mut total_fuel = 0;
        while current_fuel > 0 {
            total_fuel += current_fuel;
            current_fuel = current_fuel.fuel_consumption();
        }

        total_fuel
    }
}

#[cfg(test)]
mod test {

    use super::Module;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(14.fuel_consumption(), 2);
    }

    #[test]
    fn test_calculate_fuel_including_fuel() {
        assert_eq!(14.fuel_consumption_including_fuel(), 2);
        assert_eq!(1969.fuel_consumption_including_fuel(), 966);
    }
}
