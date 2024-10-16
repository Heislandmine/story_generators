// 定数
const BASE_FEMALE_MEAN_HEIGHT: i32 = 1569;
const BASE_FEMALE_HEIGHT_STD: i32 = 68;
const BASE_FEMALE_MEAN_WIGHT: i32 = 52;
const BASE_FEMALE_WIGHT_STD: i32 = 7;
const BASE_FEMALE_MEAN_BUST: i32 = 845;
const BASE_FEMALE_BUST_STD: i32 = 62;
const BASE_FEMALE_MEAN_UNDER_BUST: i32 = 718;
const BASE_FEMALE_UNDER_BUST_STD: i32 = 51;
const BASE_FEMALE_MEAN_WAIST: i32 = 664;
const BASE_FEMALE_WAIST_STD: i32 = 72;
const BASE_FEMALE_MEAN_HIP: i32 = 897;
const BASE_FEMALE_HIP_STD: i32 = 45;

pub struct BaseFemale {
    pub height: i32,
    pub wight: i32,
    pub bust: i32,
    pub under_bust: i32,
    pub waist: i32,
    pub hip: i32,
}

pub fn generate_base_female() -> BaseFemale {
    BaseFemale {
        height: BASE_FEMALE_MEAN_HEIGHT,
        wight: BASE_FEMALE_MEAN_WIGHT,
        bust: BASE_FEMALE_MEAN_BUST,
        under_bust: BASE_FEMALE_MEAN_UNDER_BUST,
        waist: BASE_FEMALE_MEAN_WAIST,
        hip: BASE_FEMALE_MEAN_HIP
    }
}

#[test]
fn test_generate_base_female() {
    let result = generate_base_female();

    assert!(result.waist < result.under_bust);
    assert!(result.waist < result.bust);
    assert!(result.waist < result.hip);
    assert!(result.under_bust < result.bust);
    assert!(result.under_bust < result.hip);
}