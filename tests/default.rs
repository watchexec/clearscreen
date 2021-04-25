#[test]
fn default_clear() {
	let cs = clearscreen::ClearScreen::default();
	dbg!(&cs);
	cs.clear().unwrap();
}
