#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    // integer sort
    let mut vec1: Vec<i32> = vec![1, 5, 10, 2, 15];

    vec1.sort();

    assert_eq!(vec1, vec![1, 2, 5, 10, 15]);
    println!("{:?}", vec1); // [1, 2, 5, 10, 15]

    // float sort
    let mut vec2: Vec<f32> = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec2.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec2, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    println!("{:?}", vec2); // [1.1, 1.123, 1.15, 2.0, 5.5]

    // structure sort
    let mut people: Vec<Person> = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (Name and age)
    people.sort();

    println!("{:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age).reverse());

    /*
    assert_eq!(
    people,
    vec![
        Person::new("Al".to_string(), 60),
        Person::new("Zoe".to_string(), 25),
        Person::new("John".to_string(), 1),
    ]);
    */
    println!("{:?}", people);
}
