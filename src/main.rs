pub mod char_helper;
pub mod ownership_analyzer;
pub mod utils;

use char_helper as char_helper_1;
use utils::data_helper as char_helper_2;

fn main() {
    char_helper_1::a_to_z();
    char_helper_2::a_to_z();

    ownership_analyzer::do_analyze();
}
