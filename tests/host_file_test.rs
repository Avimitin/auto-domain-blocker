use auto_domain_blocker::host_file;

const HOST_FILE_PATH: &str = "./fixtures/hosts.txt";

#[test]
fn test_host_file() {
    let f = host_file::HostFile::new(HOST_FILE_PATH).unwrap();

    assert!(f.cat().len() != 0);
    assert_eq!(f.which(), HOST_FILE_PATH);
    assert_eq!(f.read_bound_index(), Some((3, 12)));

    let keep = f.cat().clone();
    f.remove().unwrap();
    // reset back
    std::fs::write(f.which(), keep).unwrap();
}
