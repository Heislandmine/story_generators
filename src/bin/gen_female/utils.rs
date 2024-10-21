pub mod utils {
    use story_generaters::generators::base_female_generator::BaseFemale;

    pub fn display_header() {
        let header = "身長,体重,バスト,アンダーバスト,カップサイズ,ウエスト,ヒップ";
        println!("{}", header);
    }
    
    pub fn display_base_female(base_female: &BaseFemale) {
        println!(
            "{},{},{},{},{},{},{}",
            base_female.height,
            base_female.wight,
            base_female.bust,
            base_female.under_bust,
            base_female.cup_size(),
            base_female.waist,
            base_female.hip
        );
    }
}