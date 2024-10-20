use generators::base_female_generator::{generate_base_female, BaseFemale};

mod generators;

fn display_header() {
    let header = "身長,体重,バスト,アンダーバスト,カップサイズ,ウエスト,ヒップ";
    println!("{}", header);
}

fn display_base_female(base_female: &BaseFemale) {
    println!("{},{},{},{},{},{},{}", base_female.height, base_female.wight, base_female.bust, base_female.under_bust, base_female.cup_size(), base_female.waist, base_female.hip);
}
fn main() {

    display_header();

    for _ in 0..1000 {
        let female = generate_base_female();
        display_base_female(&female);
    }
}