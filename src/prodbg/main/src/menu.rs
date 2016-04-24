extern crate minifb;

use minifb::{Key, MENU_KEY_CTRL, MENU_KEY_COMMAND};
use minifb::Menu as MinifbMenu;

const MENU_TEST_ID: usize = 1;
const OTHER_MENU_ID: usize = 2;
const CLOSE_MENU_ID: usize = 6;

pub struct Menu<'a> {
    pub file_menu: Vec<MinifbMenu<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu {
            file_menu: Self::create_file_menu(),
        }
    }

    pub fn create_file_menu() -> Vec<MinifbMenu<'a>> {
        vec![
            MinifbMenu {
                name: "Menu Test",
                key: Key::W,
                id: MENU_TEST_ID,
                modifier: MENU_KEY_CTRL,
                mac_mod: MENU_KEY_COMMAND,
                ..MinifbMenu::default()
            },
            MinifbMenu::separotor(),
            MinifbMenu  {
                name: "Other menu!",
                key: Key::S,
                modifier: MENU_KEY_CTRL,
                mac_mod: MENU_KEY_CTRL,
                id: OTHER_MENU_ID,
                ..MinifbMenu::default()
            },
            MinifbMenu {
                name: "Remove Menu",
                key: Key::R,
                id: CLOSE_MENU_ID,
                ..MinifbMenu::default()
            }
        ]
    }
}

