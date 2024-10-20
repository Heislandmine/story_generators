use generators::base_female_generator::generate_base_female;

mod generators;
fn main() {
    let female = generate_base_female();
    println!("身長:{}", female.height);
    println!("体重:{}", female.wight);
    println!("バスト:{}", female.bust);
    println!("アンダーバスト:{}", female.under_bust);
    println!("カップ数:{}", female.cup_size());
    println!("ウエスト:{}", female.waist);
    println!("ヒップ:{}", female.hip);
}
