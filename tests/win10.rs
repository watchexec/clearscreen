// obviously, only run this test on win10
#[cfg(windows)]
#[test]
fn is_win10() {
	assert!(clearscreen::is_windows_10());
}
