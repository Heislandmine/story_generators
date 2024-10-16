use rand_distr::{Distribution, Normal};



// 定数
const BASE_FEMALE_MEAN_HEIGHT: f32 = 1569.0;
const BASE_FEMALE_HEIGHT_STD: f32 = 68.0;
const BASE_FEMALE_MEAN_WIGHT: f32 = 52.0;
const BASE_FEMALE_WIGHT_STD: f32 = 7.0;
const BASE_FEMALE_MEAN_BUST: f32 = 845.0;
const BASE_FEMALE_BUST_STD: f32 = 62.0;
const BASE_FEMALE_MEAN_UNDER_BUST: f32 = 718.0;
const BASE_FEMALE_UNDER_BUST_STD: f32 = 51.0;
const BASE_FEMALE_MEAN_WAIST: f32 = 664.0;
const BASE_FEMALE_WAIST_STD: f32 = 72.0;
const BASE_FEMALE_MEAN_HIP: f32 = 897.0;
const BASE_FEMALE_HIP_STD: f32 = 45.0;

pub struct BaseFemale {
    pub height: f32,
    pub wight: f32,
    pub bust: f32,
    pub under_bust: f32,
    pub waist: f32,
    pub hip: f32,
}

pub fn generate_base_female() -> BaseFemale {
    let waist = sample_from_normal(BASE_FEMALE_MEAN_WAIST, BASE_FEMALE_WAIST_STD);

    let mut under_bust = sample_from_normal(BASE_FEMALE_MEAN_UNDER_BUST, BASE_FEMALE_UNDER_BUST_STD);
    while waist > under_bust {
        under_bust = sample_from_normal(BASE_FEMALE_MEAN_UNDER_BUST, BASE_FEMALE_UNDER_BUST_STD);
    }

    let mut bust = sample_from_normal(BASE_FEMALE_MEAN_BUST, BASE_FEMALE_BUST_STD);
    while waist > bust || under_bust > bust {
        bust = sample_from_normal(BASE_FEMALE_MEAN_BUST, BASE_FEMALE_BUST_STD);
    }

    let mut hip = sample_from_normal(BASE_FEMALE_MEAN_HIP, BASE_FEMALE_HIP_STD);
    while waist > hip || under_bust > hip {
        hip = sample_from_normal(BASE_FEMALE_MEAN_HIP, BASE_FEMALE_HIP_STD);
    }

    BaseFemale {
        height: sample_from_normal(BASE_FEMALE_MEAN_HEIGHT, BASE_FEMALE_HEIGHT_STD),
        wight: sample_from_normal(BASE_FEMALE_MEAN_WIGHT, BASE_FEMALE_WIGHT_STD),
        bust,
        under_bust,
        waist,
        hip: hip
    }
}

pub fn sample_from_normal(mean: f32, std_dev: f32) -> f32 {
    let normal = Normal::new(mean, std_dev).unwrap();

    normal.sample(&mut rand::thread_rng())
}

#[test]
fn test_generate_base_female() {
    for _ in 0..100 {
        let result = generate_base_female();

        assert!(result.waist < result.under_bust);
        assert!(result.waist < result.bust);
        assert!(result.waist < result.hip);
        assert!(result.under_bust < result.bust);
        assert!(result.under_bust < result.hip);
    }
}