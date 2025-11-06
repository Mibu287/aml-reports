use shadow_rs::{BuildPattern, ShadowBuilder};

fn main() {
    #[cfg(feature = "packaging")]
    {
        if !shadow_rs::git_clean() {
            panic!("Can not build with feature 'packaging' when git repo is not clean!!!")
        }
    }

    ShadowBuilder::builder()
        .build_pattern(BuildPattern::RealTime)
        .build()
        .unwrap();
}
