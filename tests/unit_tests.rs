#[cfg(test)]
mod tests {
    use pxp::purge;
    use std::time::UNIX_EPOCH;
    use tempfile::tempfile;

    #[test]
    fn test_time() {
        let mut temp_file = tempfile().expect("Failed to create temporary file");

        purge::time(&mut temp_file);

        // Fetch and verify the new times
        let file_metadata = temp_file.metadata().expect("Failed to get file metadata");
        let accessed = file_metadata
            .accessed()
            .expect("Failed to get accessed time");
        let modified = file_metadata
            .modified()
            .expect("Failed to get modified time");

        // Since the precision of SystemTime can be limited, especially on older systems,
        // we should focus on confirming it is at or very close to the UNIX epoch.
        assert_eq!(
            accessed, UNIX_EPOCH,
            "Accessed time is not equal to UNIX epoch"
        );
        assert_eq!(
            modified, UNIX_EPOCH,
            "Modified time is not equal to UNIX epoch"
        );
    }
}
