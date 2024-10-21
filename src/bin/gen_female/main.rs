mod utils;

use story_generaters::generators::base_female_generator::generate_base_female;
use utils::utils::{display_base_female, display_header};

fn main() {
    display_header();

    for _ in 0..1000 {
        let female = generate_base_female();
        display_base_female(&female);
    }
}
