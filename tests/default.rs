#[test]
fn default_clear() {
	let cs = clearscreen::clear();
	dbg!(&cs);
	cs.unwrap();
}
