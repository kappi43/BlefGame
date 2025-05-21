use clearscreen::ClearScreen;

pub fn clear_screen() {
    let res = ClearScreen::default().clear();
    if let Err(e) = res {
        println!("Could not clear screen: {e}");
    }
}
