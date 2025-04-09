use crossterm::event::KeyCode;

/// A single menu item entry
#[derive(Clone)]
pub struct TMenuItem {
    pub label: String,                   // Label text with ~hotkey~ markers, e.g. "~O~pen"
    pub command: u16,                    // Numeric command ID
    pub keycode: Option<KeyCode>,        // Optional keyboard shortcut
    pub disabled: bool,                  // Whether the item is disabled
    pub help_context: Option<u16>,       // Optional help context ID
    pub submenu: Option<TMenu>,          // Optional nested submenu
}

impl TMenuItem {
    pub fn new(label: &str, command: u16) -> Self {
        Self {
            label: label.to_string(),
            command,
            keycode: None,
            disabled: false,
            help_context: None,
            submenu: None,
        }
    }

    pub fn with_keycode(mut self, key: KeyCode) -> Self {
        self.keycode = Some(key);
        self
    }

    pub fn with_help_context(mut self, ctx: u16) -> Self {
        self.help_context = Some(ctx);
        self
    }

    pub fn with_submenu(mut self, submenu: TMenu) -> Self {
        self.submenu = Some(submenu);
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.disabled = val;
        self
    }

    /// Parses and returns the first hotkey from the label (after a `~`)
    pub fn hotkey(&self) -> Option<char> {
        self.label
            .find('~')
            .and_then(|i| self.label.chars().nth(i + 1))
            .map(|c| c.to_ascii_lowercase())
    }

    /// Returns the label string with hotkey markers (`~`) stripped
    pub fn clean_label(&self) -> String {
        self.label.replace("~", "")
    }
}

#[derive(Clone)]
pub struct TMenu {
    pub items: Vec<TMenuItem>,
}

impl TMenu {
    pub fn new(items: Vec<TMenuItem>) -> Self {
        Self { items }
    }

    pub fn add_item(&mut self, item: TMenuItem) {
        self.items.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
