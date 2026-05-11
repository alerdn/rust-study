struct Student {
    name: String,
    grade: Option<u32>,
}

fn find_student<'a>(name: &String, db: &'a Vec<Student>) -> Result<&'a Student, String> {
    for student in db {
        if student.name == *name {
            return Ok(student);
        }
    }
    Err("Student not found".to_string())
}

fn get_grade(name: &String, db: &Vec<Student>) -> Result<Option<u32>, String> {
    let student = find_student(name, db)?;

    Ok(student.grade)
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

    let student_name = &String::from("Alexandreo");

    match get_grade(student_name, &student_db) {
        Ok(option_grade) => {
            if let Some(grade) = option_grade {
                println!("Grade: {}", grade)
            }
        }
        Err(message) => println!("Error: {}", message),
    }
}
