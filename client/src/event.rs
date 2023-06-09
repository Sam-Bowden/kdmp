use crate::error_menu::ErrorMenu;

pub enum Event {
    OpenErrorMenu(ErrorMenu),
    OpenCommandMenu,
    Exit,
    None,
}
