//! Safe alternate-screen-buffer lifecycle, cursor-state and viewport semantics.
//!
//! This owner captures the deterministic product behavior beneath the Host
//! alternate-buffer tests. Win32 locking, renderer attachment and VT byte
//! parsing remain outside this module.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorShape {
    Legacy,
    DoubleUnderscore,
    EmptyBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorState {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
    pub size: u32,
    pub shape: CursorShape,
    pub blinking: bool,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            visible: true,
            size: 25,
            shape: CursorShape::Legacy,
            blinking: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewportSize {
    pub width: u16,
    pub height: u16,
}

impl ViewportSize {
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Default for ViewportSize {
    fn default() -> Self {
        Self::new(80, 25)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferState {
    pub cursor: CursorState,
    pub viewport: ViewportSize,
    pub magenta_background: bool,
    pub text: String,
}

impl Default for BufferState {
    fn default() -> Self {
        Self {
            cursor: CursorState::default(),
            viewport: ViewportSize::default(),
            magenta_background: false,
            text: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternateBufferState {
    main: BufferState,
    alternate: Option<BufferState>,
    active_alternate: bool,
    generation: u64,
}

impl Default for AlternateBufferState {
    fn default() -> Self {
        Self::new()
    }
}

impl AlternateBufferState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            main: BufferState::default(),
            alternate: None,
            active_alternate: false,
            generation: 0,
        }
    }

    #[must_use]
    pub fn with_main_viewport(width: u16, height: u16) -> Self {
        let mut state = Self::new();
        state.main.viewport = ViewportSize::new(width, height);
        state
    }

    #[must_use]
    pub const fn is_alternate_active(&self) -> bool {
        self.active_alternate
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn main(&self) -> &BufferState {
        &self.main
    }

    #[must_use]
    pub fn alternate(&self) -> Option<&BufferState> {
        self.alternate.as_ref()
    }

    pub fn main_mut(&mut self) -> &mut BufferState {
        &mut self.main
    }

    pub fn active_mut(&mut self) -> &mut BufferState {
        if self.active_alternate {
            self.alternate.as_mut().expect("active alternate exists")
        } else {
            &mut self.main
        }
    }

    #[must_use]
    pub fn active_viewport(&self) -> ViewportSize {
        if self.active_alternate {
            self.alternate
                .as_ref()
                .expect("active alternate exists")
                .viewport
        } else {
            self.main.viewport
        }
    }

    /// Console screen-buffer information is projected from the active buffer,
    /// even when the persistent main SCREEN_INFORMATION owns the API call.
    #[must_use]
    pub fn api_viewport(&self) -> ViewportSize {
        self.active_viewport()
    }

    /// Resizes only the active alternate viewport. The persistent main viewport
    /// remains untouched and becomes visible again when alternate state exits.
    pub fn resize_alternate_viewport(&mut self, width: u16, height: u16) {
        if let Some(alternate) = self.alternate.as_mut() {
            alternate.viewport = ViewportSize::new(width, height);
        }
    }

    /// Creates/replaces the alternate buffer. A replacement always links back
    /// to the same main buffer, even when requested while an alternate is active.
    pub fn use_alternate(&mut self) {
        let inherited = if self.active_alternate {
            self.alternate.as_ref().unwrap_or(&self.main)
        } else {
            &self.main
        };
        let inherited_cursor = inherited.cursor;
        let inherited_viewport = inherited.viewport;
        self.generation = self.generation.saturating_add(1);
        self.alternate = Some(BufferState {
            cursor: inherited_cursor,
            viewport: inherited_viewport,
            ..BufferState::default()
        });
        self.active_alternate = true;
    }

    /// Returns to main. Position belongs to each buffer, while cursor visual
    /// properties follow the active alternate back to main, matching Host.
    pub fn use_main(&mut self) {
        if let Some(alternate) = self.alternate.take() {
            self.main.cursor.visible = alternate.cursor.visible;
            self.main.cursor.size = alternate.cursor.size;
            self.main.cursor.shape = alternate.cursor.shape;
            self.main.cursor.blinking = alternate.cursor.blinking;
        }
        self.active_alternate = false;
    }

    /// RIS exits alternate-screen state and restores main ownership.
    pub fn ris(&mut self) {
        self.use_main();
    }

    /// Minimal VT-dispatch observable used by the Microsoft alternate-buffer
    /// contract: writes issued through the main SCREEN_INFORMATION while the
    /// alternate is active are dispatched to the active alternate.
    pub fn dispatch_vt(&mut self, sequence: &str) {
        let target = self.active_mut();
        match sequence {
            "\u{1b}[5;6H" => {
                target.cursor.x = 5;
                target.cursor.y = 4;
            }
            "\u{1b}[48;2;255;0;255m" => target.magenta_background = true,
            "X" => {
                target.text.push('X');
                target.cursor.x = target.cursor.x.saturating_add(1);
            }
            _ => {}
        }
    }
}
