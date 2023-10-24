// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

pub fn main() {

    let student_0: Student = Student::new("Nadir", Some(2686));
    let student_1: Student = Student::new("Juliva", Some(1988));
    let student_2: Student = Student::new("Vanessa", Some(3872));
    let student_3: Student = Student::new("Jake", None);
    let student_4: Student = Student::new("Bill", None);

    let mut students:Vec<Student> = Vec::new();
    students.push(student_0); 
    students.push(student_1); 
    students.push(student_2); 
    students.push(student_3); 
    students.push(student_4); 
    println!("\nStudents with lockers assigned");
    for student in students {
        match student.locker {
            Some(locker) => println!("\nname: {}\nlocker assignment: {}\n", student.name, locker),
            None => (),
        }
    }


}



pub struct Student {
    name: String,
    locker: Option<i32>
}

impl Student {
    fn new(name: &str,locker: Option<i32> ) -> Self {
        Self {
        name: name.to_owned(),
        locker,
        }
    }
}
