#[cfg(test)]
mod tests {
    use crate::utils::file_utils::is_empty_dir;
    use std::{fs, path::PathBuf};

    #[test]
    fn test_is_not_empty_dir() {
        let temp_dir: PathBuf = std::env::temp_dir();
        let sample_dir = format!("{}/sample", temp_dir.to_str().unwrap());
        let sample_file = format!("{}/sample/sample.txt", temp_dir.to_str().unwrap());
        let _ = fs::remove_dir(&sample_dir);
        let _ = fs::remove_file(&sample_file);
        let _ = fs::create_dir(&sample_dir);
        let _ = fs::File::create(&sample_file);
        let res = is_empty_dir(PathBuf::from(&sample_dir).as_path()).unwrap();
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_empty_dir() {
        let temp_dir = std::env::temp_dir();
        let sample_dir = format!("{}/sample", temp_dir.to_str().unwrap());
        let sample_file = format!("{}/sample/sample.txt", temp_dir.to_str().unwrap());
        let _ = fs::remove_dir(&sample_dir);
        let _ = fs::remove_file(&sample_file);
        let _ = fs::create_dir(&sample_dir);
        let res = is_empty_dir(PathBuf::from(&sample_dir).as_path()).unwrap();
        assert_eq!(res, true);
    }
}
