fn main() {
    let mut part_one = 0;
    let mut part_two = 0;

    for passport in Passport::load_from_file("input.txt").unwrap() {
        if passport.is_valid_p1() {
            part_one += 1;
        }

        if passport.is_valid_p2() {
            part_two += 1;
        }
    }

    println!("{}", part_one);
    println!("{}", part_two);
}

struct Field {
    name: String,
    value: String,
}

impl Field {
    fn is_valid_p1(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].contains(&self.name.as_str())
    }

    fn is_valid_p2(&self) -> bool {
        match self.name.as_str() {
            "byr" => self.is_valid_birth_year(),
            "iyr" => self.is_valid_issue_year(),
            "eyr" => self.is_valid_expiration_year(),
            "hgt" => self.is_valid_height(),
            "hcl" => self.is_valid_hair_color(),
            "ecl" => self.is_valid_eye_color(),
            "pid" => self.is_valid_passport_id(),
            _ => false,
        }
    }

    fn is_valid_birth_year(&self) -> bool {
        self.is_value_between(1920, 2002)
    }

    fn is_valid_issue_year(&self) -> bool {
        self.is_value_between(2010, 2020)
    }

    fn is_valid_expiration_year(&self) -> bool {
        self.is_value_between(2020, 2030)
    }

    fn is_value_between(&self, min: u32, max: u32) -> bool {
        match self.value.parse::<u32>() {
            Ok(value) => (min..=max).contains(&value),
            Err(_) => false,
        }
    }

    fn is_valid_height(&self) -> bool {
        let (num, unit) = self.value.split_at(self.value.len() - 2);

        let num = match num.parse::<u32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        match unit {
            "cm" => (150..=193).contains(&num),
            "in" => (59..=76).contains(&num),
            _ => false,
        }
    }

    fn is_valid_hair_color(&self) -> bool {
        self.value.len() == 7
            && self.value.starts_with('#')
            && (self.value[1..]
                .chars()
                .all(|c| ('a'..='z').contains(&c) || ('0'..='9').contains(&c)))
    }

    fn is_valid_eye_color(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.value.as_str())
    }

    fn is_valid_passport_id(&self) -> bool {
        self.value.len() == 9 && self.value.parse::<u32>().is_ok()
    }
}

impl From<(&str, &str)> for Field {
    fn from(val: (&str, &str)) -> Field {
        Field {
            name: val.0.to_string(),
            value: val.1.to_string(),
        }
    }
}

struct Passport {
    fields: Vec<Field>,
}

impl Passport {
    fn load_from_file(name: &str) -> Result<Vec<Passport>, std::io::Error> {
        let passports = std::fs::read_to_string(name)?
            .replace("\n", " ")
            .split("  ")
            .map(|item| Passport {
                fields: item
                    .split(' ')
                    .filter_map(|a| a.split_once(":").map(|field| field.into()))
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();

        Ok(passports)
    }

    fn is_valid_p1(&self) -> bool {
        self.fields.iter().filter(|f| f.is_valid_p1()).count() == 7
    }

    fn is_valid_p2(&self) -> bool {
        self.fields.iter().filter(|f| f.is_valid_p2()).count() == 7
    }
}
