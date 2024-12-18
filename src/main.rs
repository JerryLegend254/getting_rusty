use std::{collections::HashMap, f64};
fn main() {
    let mut x = vec![1, 2, 3, 4, 5, 2];
    println!("{:?}", stats(&mut x));

    let s = "first";
    println!("{}", string_to_pig_latin(s));

    let mut company = Company {
        departments: HashMap::new(),
    };
    company.add_employee(Employee {
        name: "JerryLegend".to_string(),
        department: "Software".to_string(),
    });
    company.add_employee(Employee {
        name: "John Doe".to_string(),
        department: "HR".to_string(),
    });
    company.add_employee(Employee {
        name: "Jane Doe".to_string(),
        department: "IT".to_string(),
    });
    company.add_employee(Employee {
        name: "Man Claims".to_string(),
        department: "IT".to_string(),
    });

    println!("{:?}", company.get_employees("IT"));
    println!("{:?}", company.get_all_employees());
}

fn mean(x: &Vec<i32>) -> f64 {
    let sum: i32 = x.iter().sum();
    let len = x.len() as f64;
    sum as f64 / len
}

fn mode(x: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for &value in x.iter() {
        *counts.entry(value).or_insert(0) += 1;
    }
    let max = counts.values().max().unwrap();
    *counts.iter().find(|&(_, v)| v == max).unwrap().0
}

fn median(x: &mut Vec<i32>) -> f64 {
    x.sort();
    let mid = x.len() / 2;
    if x.len() % 2 == 0 {
        (x[mid - 1] + x[mid]) as f64 / 2 as f64
    } else {
        x[mid] as f64
    }
}

fn stats(x: &mut Vec<i32>) -> (f64, i32, f64) {
    (mean(x), mode(x), median(x))
}

fn string_to_pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let words = s.split_whitespace();
    let mut result = String::new();
    for word in words {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if vowels.contains(&first_char) {
            result.push_str(&format!("{}-hay ", word));
        } else {
            result.push_str(&format!("{}-{}ay ", chars.collect::<String>(), first_char));
        }
    }
    result
}

struct Employee {
    name: String,
    department: String,
}
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn add_employee(&mut self, employee: Employee) {
        self.departments
            .entry(employee.department)
            .or_insert(Vec::new())
            .push(employee.name);
    }

    fn get_employees(&self, department: &str) -> Vec<String> {
        let dpt = match self.departments.get(department) {
            Some(dpt) => dpt,
            None => return Vec::new(),
        };
        dpt.to_vec()
    }
    fn get_all_employees(&self) -> Vec<&String> {
        let mut employees = Vec::new();
        for (_, names) in self.departments.iter() {
            employees.extend(names);
        }
        employees
    }
}
