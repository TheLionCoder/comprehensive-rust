#[allow(dead_code)]
pub struct User {
    name: String,
    age: u16,
    height: f32,
    visit_count: u16,
    last_blood_pressure: Option<(i32, i32)>
}

pub struct Measurements {
    height: f32,
    blood_pressure: (i32, i32)
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u16,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>
}

impl User {
    pub fn new(name: String, age: u16, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }
    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;
        let blood_pressure: (i32, i32) = measurements.blood_pressure;
        let report: HealthReport = HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count,
            height_change: measurements.height - self.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(lbp) => {
                    Some((blood_pressure.0 - lbp.0, blood_pressure.1 - lbp.1))
                }
                None => None
            },
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(blood_pressure);
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_doctor() {
        let mut bob = User::new(String::from("Bob"), 15, 1.73);
        // validate that there are no visits because is a new use
        assert_eq!(bob.visit_count, 0);
        let report = bob.visit_doctor(
            Measurements {
                height: 1.76,
                blood_pressure: (120, 80)
            }
        );
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, None);
        assert!(report.height_change < 0.03);

        let second_report = bob.visit_doctor(
            Measurements {
                height: 1.76,
                blood_pressure: (125, 75)
            }
        );
        assert_eq!(second_report.visit_count, 2);
        assert_eq!(second_report.height_change, 0.0);
        assert_eq!(second_report.blood_pressure_change, Some((5, -5)));
    }
}