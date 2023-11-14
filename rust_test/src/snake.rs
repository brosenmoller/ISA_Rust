use std::collections::{self, LinkedList};

#[derive(PartialEq, Eq)]
struct Vector2 {
    x: i32,
    y: i32,
}

enum Direction {
    Stop,
    Up,
    Down,
    Left,
    Right
}

pub(crate) fn snake() {
    const WIDTH: i32 = 200;
    const HEIGHT: i32 = 200;

    let mut terminated = false;
    let mut score = 0;

    let mut head = Vector2 {x: 0, y: 0};
    let mut fruit = Vector2 {x: 0, y: 0};
    let mut tail: LinkedList<Vector2> = LinkedList::new();

    let mut direction = Direction::Stop;

    while !terminated {
        draw(WIDTH, HEIGHT, &mut score, &head, &fruit, &mut tail);
        input(&mut direction);
        logic();
    }
}

fn draw(width: i32, height: i32, score: &mut i32, head: &Vector2, fruit: &Vector2, tail: &mut LinkedList<Vector2>) {
    print!("{esc}c", esc = 27 as char);
    println!("Score: {}", score);

    for y in (1..height).rev() 
    {
        for x in 1..width
        {
            let pos = Vector2 {x, y};

            // Border
            if pos.y == 1 || pos.y == height { print!("#"); }
            else if pos.x == 0 || pos.x == width - 1 { print!("#"); }

            // Snake head
            else if pos.eq(head) { print!("O"); }

            // Fruit
            else if pos.eq(fruit) { print!("x"); }

            // tail
            else if tail.contains(&pos) { print!("o"); }

            // Blank space
            else { print!(" "); }
        }

        println!();
    }
}

fn input(direction: &mut Direction) {
    
}

fn logic() {

}

