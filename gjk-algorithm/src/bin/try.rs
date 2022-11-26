trait S {
    type Itr<'a>: Iterator<Item = i32>
    where
        Self: 'a;

    fn v<'a>(&'a self) -> Self::Itr<'a>;
}

struct IS {
    ver: Vec<i32>,
}

impl S for IS {
    type Itr<'a> = std::iter::Cloned<std::slice::Iter<'a, i32>>;

    fn v<'a>(&'a self) -> Self::Itr<'a> {
        self.ver.iter().cloned()
    }
}

enum RS {
    Square,
}

impl S for RS {
    type Itr<'a> = std::iter::Map<std::ops::Range<i32>, Box<dyn FnMut(i32) -> i32>>;

    fn v<'a>(&'a self) -> Self::Itr<'a> {
        match self {
            RS::Square => (0..4).map(Box::new(|x| x * 90)),
        }
    }
}

fn main() {
    println!("yes");
}
