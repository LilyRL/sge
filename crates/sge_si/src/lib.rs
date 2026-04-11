use sge_si_macros::gen_types;

gen_types!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_conversions() {
        let m = Length::meters(1.0);
        assert_eq!(m.as_centimeters(), 100.0);
        assert_eq!(m.as_kilometers(), 0.001);
        assert_eq!(m.as_millimeters(), 1000.0);

        let km = Length::kilometers(2.5);
        assert_eq!(km.as_meters(), 2500.0);
    }

    #[test]
    fn mass_conversions() {
        let kg = Mass::kilograms(1.0);
        assert_eq!(kg.as_grams(), 1000.0);
        assert_eq!(kg.as_milligrams(), 1_000_000.0);

        let mg = Mass::milligrams(5000.0);
        assert_eq!(mg.as_grams(), 5.0);
    }

    #[test]
    fn time_conversions() {
        let sec = Time::seconds(60.0);
        assert_eq!(sec.as_milliseconds(), 60_000.0);
        assert_eq!(sec.as_microseconds(), 60_000_000.0);
    }

    #[test]
    fn length_addition() {
        let result = Length::meters(1.0) + Length::centimeters(50.0);
        assert_eq!(result.as_meters(), 1.5);
    }

    #[test]
    fn length_subtraction() {
        let result = Length::kilometers(2.0) - Length::meters(500.0);
        assert_eq!(result.as_meters(), 1500.0);
    }

    #[test]
    fn scalar_multiplication() {
        let distance = Length::meters(10.0);
        let doubled = distance * 2.0;
        assert_eq!(doubled.as_meters(), 20.0);
    }

    #[test]
    fn scalar_division() {
        let distance = Length::meters(100.0);
        let halved = distance / 2.0;
        assert_eq!(halved.as_meters(), 50.0);
    }

    #[test]
    fn speed_from_distance_and_time() {
        let distance = Length::meters(100.0);
        let time = Time::seconds(10.0);
        let speed = distance / time;
        assert_eq!(speed.as_meters_per_second(), 10.0);
    }

    #[test]
    fn acceleration_from_speed_and_time() {
        let speed = Speed::meters_per_second(50.0);
        let time = Time::seconds(5.0);
        let accel = speed / time;
        assert_eq!(accel.as_meters_per_second_squared(), 10.0);
    }

    #[test]
    fn distance_from_speed_and_time() {
        let speed = Speed::meters_per_second(25.0);
        let time = Time::seconds(4.0);
        let distance = speed * time;
        assert_eq!(distance.as_meters(), 100.0);
    }

    #[test]
    fn speed_from_acceleration_and_time() {
        let accel = Acceleration::meters_per_second_squared(9.8);
        let time = Time::seconds(3.0);
        let speed = accel * time;
        assert!((speed.as_meters_per_second() - 29.4).abs() < 0.01);
    }

    #[test]
    fn force_from_mass_and_acceleration() {
        let mass = Mass::kilograms(10.0);
        let accel = Acceleration::meters_per_second_squared(9.8);
        let force = mass * accel;
        assert!((force.as_newtons() - 98.0).abs() < 0.01);
    }

    #[test]
    fn mass_from_force_and_acceleration() {
        let force = Force::newtons(100.0);
        let accel = Acceleration::meters_per_second_squared(10.0);
        let mass = force / accel;
        assert_eq!(mass.as_kilograms(), 10.0);
    }

    #[test]
    fn momentum_from_mass_and_velocity() {
        let mass = Mass::kilograms(5.0);
        let velocity = Speed::meters_per_second(20.0);
        let momentum = mass * velocity;
        assert_eq!(momentum.as_newton_seconds(), 100.0);
    }

    #[test]
    fn impulse_from_force_and_time() {
        let force = Force::newtons(50.0);
        let time = Time::seconds(2.0);
        let impulse = force * time;
        assert_eq!(impulse.as_newton_seconds(), 100.0);
    }

    #[test]
    fn energy_from_force_and_distance() {
        let force = Force::newtons(100.0);
        let distance = Length::meters(5.0);
        let energy = force * distance;
        assert_eq!(energy.as_joules(), 500.0);
    }

    #[test]
    fn torque_from_force_and_distance() {
        let force = Force::newtons(50.0);
        let distance = Length::meters(2.0);
        let torque = distance * force;
        assert_eq!(torque.as_newton_meters(), 100.0);
    }

    #[test]
    fn power_from_energy_and_time() {
        let energy = Energy::joules(1000.0);
        let time = Time::seconds(10.0);
        let power = energy / time;
        assert_eq!(power.as_watts(), 100.0);
    }

    #[test]
    fn power_from_force_and_speed() {
        let force = Force::newtons(200.0);
        let speed = Speed::meters_per_second(5.0);
        let power = force * speed;
        assert_eq!(power.as_watts(), 1000.0);
    }

    #[test]
    fn energy_from_power_and_time() {
        let power = Power::watts(500.0);
        let time = Time::seconds(20.0);
        let energy = power * time;
        assert_eq!(energy.as_joules(), 10_000.0);
    }

    #[test]
    fn area_from_length_squared() {
        let side = Length::meters(5.0);
        let area = side * side;
        assert_eq!(area.as_meters_squared(), 25.0);
    }

    #[test]
    fn volume_from_area_and_length() {
        let area = Area::meters_squared(10.0);
        let height = Length::meters(3.0);
        let volume = area * height;
        assert_eq!(volume.as_meters_cubed(), 30.0);
    }

    #[test]
    fn pressure_from_force_and_area() {
        let force = Force::newtons(1000.0);
        let area = Area::meters_squared(2.0);
        let pressure = force / area;
        assert_eq!(pressure.as_pascals(), 500.0);
    }

    #[test]
    fn force_from_pressure_and_area() {
        let pressure = Pressure::pascals(1000.0);
        let area = Area::meters_squared(5.0);
        let force = pressure * area;
        assert_eq!(force.as_newtons(), 5000.0);
    }

    #[test]
    fn density_from_mass_and_volume() {
        let mass = Mass::kilograms(800.0);
        let volume = Volume::meters_cubed(1.0);
        let density = mass / volume;
        assert_eq!(density.as_kilograms_per_meter_cubed(), 800.0);
    }

    #[test]
    fn mass_from_density_and_volume() {
        let density = Density::kilograms_per_meter_cubed(1000.0);
        let volume = Volume::meters_cubed(0.5);
        let mass = density * volume;
        assert_eq!(mass.as_kilograms(), 500.0);
    }

    #[test]
    fn charge_from_current_and_time() {
        let current = Current::amperes(5.0);
        let time = Time::seconds(10.0);
        let charge = current * time;
        assert_eq!(charge.as_coulombs(), 50.0);
    }

    #[test]
    fn current_from_charge_and_time() {
        let charge = Charge::coulombs(100.0);
        let time = Time::seconds(20.0);
        let current = charge / time;
        assert_eq!(current.as_amperes(), 5.0);
    }

    #[test]
    fn power_from_voltage_and_current() {
        let voltage = Voltage::volts(12.0);
        let current = Current::amperes(5.0);
        let power = voltage * current;
        assert_eq!(power.as_watts(), 60.0);
    }

    #[test]
    fn resistance_from_voltage_and_current() {
        let voltage = Voltage::volts(24.0);
        let current = Current::amperes(2.0);
        let resistance = voltage / current;
        assert_eq!(resistance.as_ohms(), 12.0);
    }

    #[test]
    fn voltage_from_current_and_resistance() {
        let current = Current::amperes(3.0);
        let resistance = Resistance::ohms(10.0);
        let voltage = current * resistance;
        assert_eq!(voltage.as_volts(), 30.0);
    }

    #[test]
    fn energy_from_voltage_and_charge() {
        let voltage = Voltage::volts(100.0);
        let charge = Charge::coulombs(5.0);
        let energy = voltage * charge;
        assert_eq!(energy.as_joules(), 500.0);
    }

    #[test]
    fn charge_from_capacitance_and_voltage() {
        let capacitance = Capacitance::farads(0.001);
        let voltage = Voltage::volts(12.0);
        let charge = capacitance * voltage;
        assert_eq!(charge.as_coulombs(), 0.012);
    }

    #[test]
    fn angular_velocity_from_angle_and_time() {
        let angle = Angle::radians(6.28);
        let time = Time::seconds(1.0);
        let angular_vel = angle / time;
        assert!((angular_vel.as_radians_per_second() - 6.28).abs() < 0.01);
    }

    #[test]
    fn angular_acceleration_from_angular_velocity_and_time() {
        let angular_vel = AngularVelocity::radians_per_second(10.0);
        let time = Time::seconds(2.0);
        let angular_accel = angular_vel / time;
        assert_eq!(angular_accel.as_radians_per_second_squared(), 5.0);
    }

    #[test]
    fn kinetic_energy_calculation() {
        let mass = Mass::kilograms(10.0);
        let speed = Speed::meters_per_second(5.0);
        let momentum = mass * speed;

        let energy_value = momentum.as_newton_seconds() * speed.as_meters_per_second() * 0.5;
        assert_eq!(energy_value, 125.0);
    }

    #[test]
    fn gravity_potential_energy() {
        let mass = Mass::kilograms(5.0);
        let gravity = Acceleration::meters_per_second_squared(9.8);
        let height = Length::meters(10.0);
        let force = mass * gravity;
        let energy = force * height;
        assert!((energy.as_joules() - 490.0).abs() < 0.1);
    }

    #[test]
    fn electrical_power_consumption() {
        let voltage = Voltage::volts(220.0);
        let current = Current::amperes(10.0);
        let power = voltage * current;
        let time = Time::seconds(3600.0);
        let energy = power * time;
        assert_eq!(energy.as_joules(), 7_920_000.0);
        assert_eq!(energy.as_kilojoules(), 7_920.0);
    }

    #[test]
    fn length_comparison() {
        let m1 = Length::meters(1.0);
        let cm100 = Length::centimeters(100.0);
        assert_eq!(m1, cm100);

        let m2 = Length::meters(2.0);
        assert!(m2 > m1);
    }

    #[test]
    fn force_comparison() {
        let n1000 = Force::newtons(1000.0);
        let kn1 = Force::kilonewtons(1.0);
        assert_eq!(n1000, kn1);
    }

    #[test]
    fn extreme_scale_conversions() {
        let nm = Length::nanometers(1_000_000.0);
        assert_eq!(nm.as_millimeters(), 1.0);

        let gw = Power::gigawatts(0.001);
        assert_eq!(gw.as_megawatts(), 1.0);
    }

    #[test]
    fn mixed_prefix_operations() {
        let result = Length::kilometers(1.0) + Length::millimeters(500.0);
        assert_eq!(result.as_meters(), 1000.5);
    }
}
