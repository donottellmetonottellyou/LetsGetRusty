#![allow(dead_code)]

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0",
    ];

    let good_students: Vec<Student> = students
        .into_iter()
        .filter_map(|student| {
            let mut student = student.split(' ');
            let name = student.next()?.to_owned();
            let gpa = student.next()?.parse().ok()?;

            if gpa < 3.5 {
                return None;
            }

            Some(Student { name, gpa })
        })
        .collect();

    // Bad!
    // for student in students {
    //     let mut student = student.split(' ');
    //     let name = student.next();
    //     let gpa = student.next();

    //     if let (Some(name), Some(gpa)) = (name, gpa) {
    //         let name = name.to_owned();
    //         let gpa = gpa.parse::<f32>();

    //         if let Ok(gpa) = gpa {
    //             if gpa >= 3.5 {
    //                 good_students.push(Student { name, gpa });
    //             }
    //         }
    //     }
    // }

    for student in good_students {
        println!("{student:?}");
    }
}
