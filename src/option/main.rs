struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(name: &String, db: &Vec<Student>) -> Option<u32> {
    for student in db {
        if student.name == *name {
            return student.grade;
        }
    }

    None
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alexandre"),
            grade: Some(95),
        },
        Student {
            name: String::from("Helena"),
            grade: None,
        },
    ];

    let student_grade = get_grade(&String::from("Alexandre"), &student_db);

    match student_grade {
        Some(grade) => println!("Grade is {}", grade),
        None => (),
    }

    if let Some(grade) = student_grade {
        println!("Ignoring other variants of enum and printing grade, which is {grade}")
    }
}
