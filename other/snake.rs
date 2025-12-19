// [dependencies]
// crossterm = "0.29.0"
// rand = "0.9.2"

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

const WIDTH: i16 = 30;
const HEIGHT: i16 = 20;

#[derive(Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

fn random_food(snake: &Vec<Point>) -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let p = Point {
            x: rng.gen_range(1..WIDTH - 1),
            y: rng.gen_range(1..HEIGHT - 1),
        };
        if !snake.iter().any(|s| s.x == p.x && s.y == p.y) {
            return p;
        }
    }
}

fn draw(snake: &Vec<Point>, food: Point) {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                print!("#");
            } else if snake.iter().any(|s| s.x == x && s.y == y) {
                print!("O");
            } else if food.x == x && food.y == y {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    stdout.flush().unwrap();
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide).unwrap();

    let mut snake = vec![
        Point { x: 5, y: 5 },
        Point { x: 4, y: 5 },
    ];

    let mut food = random_food(&snake);
    let mut dir = Point { x: 1, y: 0 };

    loop {
        // Управление
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                dir = match key.code {
                    KeyCode::Up => Point { x: 0, y: -1 },
                    KeyCode::Down => Point { x: 0, y: 1 },
                    KeyCode::Left => Point { x: -1, y: 0 },
                    KeyCode::Right => Point { x: 1, y: 0 },
                    KeyCode::Esc => break,
                    _ => dir,
                };
            }
        }

        let mut new_head = snake[0];
        new_head.x += dir.x;
        new_head.y += dir.y;

        // Столкновения
        if new_head.x == 0
            || new_head.x == WIDTH - 1
            || new_head.y == 0
            || new_head.y == HEIGHT - 1
            || snake.iter().any(|s| s.x == new_head.x && s.y == new_head.y)
        {
            break;
        }

        snake.insert(0, new_head);

        if new_head.x == food.x && new_head.y == food.y {
            food = random_food(&snake);
        } else {
            snake.pop();
        }

        draw(&snake, food);
        thread::sleep(Duration::from_millis(240));
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    println!("\nИгра окончена!");
}


/*#[derive(Debug, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

fn main() {
    let test =  vec![
        Point { x: 5, y: 5 },
        Point { x: 4, y: 5 },
    ];;
    let a = test[0];

    println!("{:?}", a);
    println!("{:?}", test);

}*/