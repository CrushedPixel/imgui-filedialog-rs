pub extern crate imgui_filedialog_sys as sys;
pub mod flags;
pub mod selection;
mod util;

use std::ffi::CString;

pub use crate::flags::{FileDialogFlags, FileStyleFlags};
pub use imgui::WindowFlags;

use crate::selection::Selection;
use crate::util::ptr_into_string;
use imgui::ImString;

// matches imgui-rs that also expects Into<MintVec2> as args
type MintVec2 = mint::Vector2<f32>;
type MintVec4 = mint::Vector4<f32>;

/// Main file dialog context
pub struct Context {
    ptr: *mut sys::ImGuiFileDialog,
}

#[must_use]
impl Context {
    /// Create a new file dialog context
    fn new() -> Self {
        let igfd_ctx = unsafe { sys::IGFD_Create() };
        Self { ptr: igfd_ctx }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::IGFD_Destroy(self.ptr) }
    }
}

/// File dialog configuration options
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct FileDialogConfig {
    /// Initial path to open
    pub path: String,
    /// Default filename
    pub file_name: String,
    /// Combined file path and name (takes precedence over separate path/filename)
    pub file_path_name: String,
    /// Maximum number of files that can be selected (0 = unlimited)
    pub count_selection_max: i32,
    /// Dialog behavior flags
    pub flags: FileDialogFlags,
    /// Width of the side pane (if enabled)
    pub side_pane_width: f32,
}

impl Default for FileDialogConfig {
    fn default() -> Self {
        Self {
            path: Default::default(),
            file_name: Default::default(),
            file_path_name: Default::default(),
            count_selection_max: 1,
            flags: FileDialogFlags::DEFAULT,
            side_pane_width: 250.0,
        }
    }
}

/// How to handle file extensions in results
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub enum ResultMode {
    /// Add file extension if none exists
    #[default]
    AddIfNoFileExt = sys::IGFD_ResultMode_AddIfNoFileExt as isize,
    /// Overwrite existing file extension with current filter
    OverwriteFileExt = sys::IGFD_ResultMode_OverwriteFileExt as isize,
    /// Keep the input filename unchanged
    KeepInputFile = sys::IGFD_ResultMode_KeepInputFile as isize,
}

/// Main file dialog interface
pub struct FileDialog {
    id: ImString,
    context: Context,
}

impl FileDialog {
    /// Create a new file dialog with the given identifier
    pub fn create(_ui: &mut imgui::Ui, id: &str) -> Self {
        // SAFETY: mutable access to Ui means it's safe to make ImGui sys calls.
        // FileDialog is not Send + Sync because it holds a raw pointer (in Context),
        // therefore all other functions will be safe to call as we stay on this thread.
        Self {
            context: Context::new(),
            id: ImString::new(id),
        }
    }

    /// Open the file dialog with specified parameters
    ///
    /// Arguments:
    /// - `title` - Dialog window title
    /// - `filters` - File filters (e.g., "Image files{.png,.jpg,.jpeg},Text files{.txt}" or ".json,.yaml")
    /// - `config` - Dialog configuration options
    pub fn open(
        &self,
        title: impl Into<String>,
        filters: Option<impl Into<String>>,
        config: FileDialogConfig,
    ) {
        let title_cstr = CString::new(title.into()).unwrap();
        let filters_cstr = filters.map(|f| CString::new(f.into()).unwrap());

        let path_cstr = CString::new(config.path.as_str()).unwrap();
        let filename_cstr = CString::new(config.file_name.as_str()).unwrap();
        let filepath_cstr = CString::new(config.file_path_name.as_str()).unwrap();

        let c_config = sys::IGFD_FileDialog_Config {
            path: path_cstr.as_ptr(),
            fileName: filename_cstr.as_ptr(),
            filePathName: filepath_cstr.as_ptr(),
            countSelectionMax: config.count_selection_max,
            userDatas: std::ptr::null_mut(),
            sidePane: None,
            sidePaneWidth: config.side_pane_width,
            flags: config.flags.bits() as sys::ImGuiFileDialogFlags,
        };

        unsafe {
            sys::IGFD_OpenDialog(
                self.context.ptr,
                self.id.as_ptr(),
                title_cstr.as_ptr(),
                filters_cstr
                    .as_ref() // important - otherwise value gets moved out and dropped
                    .map_or(std::ptr::null(), |c| c.as_ptr()),
                c_config,
            );
        }
    }

    /// Displays the dialog and returns true if a result was obtained (ok or not).
    /// If max size is not larger than min size, the window is made no-resize.
    ///
    /// Arguments:
    /// - `flags` - ImGui window flags
    /// - `min_size` - Minimum window size
    /// - `max_size` - Maximum window size
    pub fn display(
        &self,
        mut flags: WindowFlags,
        min_size: impl Into<MintVec2>,
        max_size: impl Into<MintVec2>,
    ) -> bool {
        let min_size = min_size.into();
        let mut max_size = max_size.into();

        if max_size.x <= min_size.x && max_size.y <= min_size.y {
            flags |= WindowFlags::NO_RESIZE;
        }

        // ensure max size isn't smaller than min size
        max_size.x = max_size.x.max(min_size.x);
        max_size.y = max_size.y.max(min_size.y);

        unsafe {
            sys::IGFD_DisplayDialog(
                self.context.ptr,
                self.id.as_ptr(),
                flags.bits() as sys::ImGuiWindowFlags,
                sys::ImVec2 {
                    x: min_size.x,
                    y: min_size.y,
                },
                sys::ImVec2 {
                    x: max_size.x,
                    y: max_size.y,
                },
            )
        }
    }

    /// Display the dialog with default parameters
    pub fn display_simple(&self) -> bool {
        self.display(
            WindowFlags::NO_COLLAPSE,
            MintVec2 { x: 200.0, y: 300.0 },
            MintVec2 { x: 700.0, y: 500.0 },
        )
    }

    /// Closes the dialog.
    pub fn close(&self) {
        unsafe {
            sys::IGFD_CloseDialog(self.context.ptr);
        }
    }

    /// Returns whether the dialog was closed with OK button.
    pub fn is_ok(&self) -> bool {
        unsafe { sys::IGFD_IsOk(self.context.ptr) }
    }

    /// Returns whether the dialog was opened this frame.
    pub fn was_opened_this_frame(&self) -> bool {
        unsafe { sys::IGFD_WasOpenedThisFrame(self.context.ptr) }
    }

    /// Returns whether the dialog with this key was opened this frame.
    pub fn was_key_opened_this_frame(&self) -> bool {
        unsafe { sys::IGFD_WasKeyOpenedThisFrame(self.context.ptr, self.id.as_ptr()) }
    }

    /// Returns whether the dialog is currently open
    pub fn is_opened(&self) -> bool {
        unsafe { sys::IGFD_IsOpened(self.context.ptr) }
    }

    /// Returns whether the dialog with this key is currently open
    pub fn is_key_opened(&self) -> bool {
        unsafe { sys::IGFD_IsKeyOpened(self.context.ptr, self.id.as_ptr()) }
    }

    /// Get the current path being browsed
    pub fn current_path(&self) -> String {
        unsafe {
            let ptr = sys::IGFD_GetCurrentPath(self.context.ptr);
            ptr_into_string(ptr)
        }
    }

    /// Get the current filename
    pub fn current_filename(&self, mode: ResultMode) -> String {
        unsafe {
            let ptr = sys::IGFD_GetCurrentFileName(self.context.ptr, mode as sys::IGFD_ResultMode);
            ptr_into_string(ptr)
        }
    }

    /// Get the current file path and name combined
    pub fn current_file_path_name(&self, mode: ResultMode) -> String {
        unsafe {
            let ptr = sys::IGFD_GetFilePathName(self.context.ptr, mode as sys::IGFD_ResultMode);
            ptr_into_string(ptr)
        }
    }

    /// Get the current filter
    pub fn current_filter(&self) -> String {
        unsafe {
            let ptr = sys::IGFD_GetCurrentFilter(self.context.ptr);
            ptr_into_string(ptr)
        }
    }

    /// Get selected files (for multi-selection dialogs)
    pub fn selection(&self, mode: ResultMode) -> Selection {
        unsafe {
            Selection::new(
                sys::IGFD_GetSelection(self.context.ptr, mode as sys::IGFD_ResultMode),
                &self.context,
            )
        }
    }

    /// Set custom file style by extension or criteria
    ///
    /// Arguments:
    /// - `flags` - What type of files to style
    /// - `criteria` - File extension or pattern to match
    /// - `color` - Color to use for matching files
    /// - `icon` - Optional icon text to display
    pub fn set_file_style(
        &self,
        flags: FileStyleFlags,
        criteria: impl Into<String>,
        color: impl Into<MintVec4>,
        icon: Option<impl Into<String>>,
    ) {
        let criteria_cstr = CString::new(criteria.into()).unwrap();
        let icon_cstr = icon.map(|i| CString::new(i.into()).unwrap());
        let icon_ptr = icon_cstr.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());

        let color = color.into();

        unsafe {
            sys::IGFD_SetFileStyle2(
                self.context.ptr,
                flags.bits() as sys::IGFD_FileStyleFlags,
                criteria_cstr.as_ptr(),
                color.x,
                color.y,
                color.z,
                color.w,
                icon_ptr,
                std::ptr::null_mut(), // font
            );
        }
    }

    /// Clear all custom file styles
    pub fn clear_file_styles(&self) {
        unsafe {
            sys::IGFD_ClearFilesStyle(self.context.ptr);
        }
    }

    /// Set locale for the dialog
    ///
    /// Arguments:
    /// - `category` - Locale category (e.g., LC_ALL)
    /// - `begin_locale` - Locale to use at beginning of dialog display
    /// - `end_locale` - Locale to use at end of dialog display
    pub fn set_locales(&self, category: i32, begin_locale: &str, end_locale: &str) {
        let begin_cstr = CString::new(begin_locale).unwrap();
        let end_cstr = CString::new(end_locale).unwrap();

        unsafe {
            sys::SetLocales(
                self.context.ptr,
                category,
                begin_cstr.as_ptr(),
                end_cstr.as_ptr(),
            );
        }
    }
}
