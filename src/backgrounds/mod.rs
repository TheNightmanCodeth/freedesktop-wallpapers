use std::path::PathBuf;

use xdg::BaseDirectories;

pub(super) fn wallpapers_base_paths() -> Vec<PathBuf> {
    let mut data_dirs: Vec<_> = BaseDirectories::new()
        .map(|bd| {
            let mut data_dirs: Vec<_> = bd
                .get_data_dirs()
                .into_iter()
                .map(|p| p.join("backgrounds"))
                .collect();
            data_dirs
        })
        .unwrap_or_default();
    data_dirs.into_iter().filter(|p| p.exists()).collect()
}

#[cfg(test)]
mod test {
    use crate::backgrounds::wallpapers_base_paths;
    use std::io::prelude::*;

    #[test]
    fn should_print_all_paths() {
        let paths = wallpapers_base_paths();
        println!("{paths:?}");
        // assert_that!(paths).is_some();
    }
}
