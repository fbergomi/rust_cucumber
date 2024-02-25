pub mod animals;

use animals::cat::Cat;

fn main() {
    let alive_cat = Cat {
        hungry: true,
        vomit: false,
        starved: false,
    };


    let dead_cat = Cat{
        hungry : true,
        vomit: false,
        starved: true,
    };

    println!("Hello, cat :\n{alive_cat}");
    println!("Hello, cat :\n{dead_cat}");
}

