use rand_distr::{Distribution, Normal};
use rstest::rstest;

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

#[derive(Debug ,PartialEq)]
pub enum CupSize {
    AAA,
    AA,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K
}

impl BaseFemale {
    pub fn cup_size(&self) -> CupSize {
        let sub = self.bust - self.under_bust;

        if sub < 6.5 {
            return CupSize::AAA
        } else if sub >= 6.5 && sub < 9.0 {
            return CupSize::AA
        } else if sub >= 9.0 && sub < 11.5 {
            return CupSize::A
        } else if sub >= 11.5 && sub < 14.0 {
            return  CupSize::B;
        } else if sub >= 14.0 && sub < 16.5 {
            return CupSize::C;
        } else if sub >= 16.5 && sub < 19.0 {
            return  CupSize::D;
        } else if sub >= 19.0 && sub < 21.5 {
            return  CupSize::E;
        } else if sub >= 21.5 && sub < 24.0 {
            return  CupSize::F;
        } else if sub >= 24.0 && sub < 26.5 {
            return  CupSize::G;
        } else if sub >= 26.5 && sub < 29.0 {
            return  CupSize::H;
        } else if sub >= 29.0 && sub < 31.5 {
            return  CupSize::I;
        } else if sub >= 31.5 && sub < 34.0 {
            return  CupSize::J;
        } else {
            return CupSize::K
        }
    }
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

#[rstest]
#[case(72.0, 65.6, CupSize::AAA)]
#[case(72.0, 65.5, CupSize::AA)]
#[case(92.0, 83.1, CupSize::AA)]
#[case(92.0, 83.0, CupSize::A)]
#[case(92.0, 80.6, CupSize::A)]
#[case(92.0, 80.5, CupSize::B)]
#[case(92.0, 78.1, CupSize::B)]
#[case(92.0, 78.0, CupSize::C)]
#[case(92.0, 75.6, CupSize::C)]
#[case(92.0, 75.5, CupSize::D)]
#[case(92.0, 73.1, CupSize::D)]
#[case(92.0, 73.0, CupSize::E)]
#[case(92.0, 70.6, CupSize::E)]
#[case(92.0, 70.5, CupSize::F)]
#[case(92.0, 68.1, CupSize::F)]
#[case(92.0, 68.0, CupSize::G)]
#[case(92.0, 65.6, CupSize::G)]
#[case(92.0, 65.5, CupSize::H)]
#[case(92.0, 63.1, CupSize::H)]
#[case(92.0, 63.0, CupSize::I)]
#[case(92.0, 60.6, CupSize::I)]
#[case(92.0, 60.5, CupSize::J)]
#[case(92.0, 58.1, CupSize::J)]
#[case(92.0, 58.0, CupSize::K)]
fn test_cup_size(#[case] bust:  f32, #[case] under_bust: f32, #[case] expected_cup_size: CupSize) {
    let sut = BaseFemale {
        height: 1500.0,
        wight: 50.0,
        bust,
        under_bust,
        waist: 50.0,
        hip: 80.0
    };

    let result = sut.cup_size();

    assert_eq!(result, expected_cup_size);
}