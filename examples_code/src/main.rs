fn structures() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    // println!("{:?}", hokey); // will error (moved)
    assert_eq!(hokey1.name, "Hokey I");
    
}
#[derive(Debug)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone, Debug)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

// receive the input Broom by value, taking ownership
fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>

}

impl Queue {
    pub fn push(&mut self, c:char) {
        self.younger.push(c);
    }
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    // let mut q = Queue::new();
    pub fn new() -> Queue {
        Queue {older: Vec::new(), younger: Vec::new()}
    }


}

fn simple_implementation() {
    let mut q: Queue  = Queue {older: Vec::new(), younger: Vec::new()};
    q.push('0');
    q.push('1');
    println!("{:?}", q);
    assert_eq!(q.pop(), Some('0'));
    println!("{:?}", q);
    q.push('∞');
    println!("{:?}", q);
    assert_eq!(q.pop(), Some('1'));
    println!("{:?}", q);
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
}

pub struct Generic_Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Generic_Queue<T> {
    pub fn new() -> Generic_Queue<T> {
        Generic_Queue  { older: Vec::new(), younger: Vec::new()}
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

fn struct_with_generic() {
    let mut q: Generic_Queue<char> = Generic_Queue::<char>::new();
    // turbo fish - ::<> notation
    let mut r=Generic_Queue::new();
    r.push(123.2); // rust can figure it out!

}

// struct with lifetime parameter!
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least { least = &slice[i];}
        if slice[i] > *greatest { greatest = &slice[i];}
    }

    Extrema { greatest, least }
}

// deriving common traits for struct types

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}


// interior mutability
use std::cell::{Cell, RefCell};
pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    leg_devices: [fd::FileDesc; 8],
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>
}

impl SpiderRobot {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n+1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

