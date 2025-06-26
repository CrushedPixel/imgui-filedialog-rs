use bitflags::bitflags;

bitflags! {
    /// File dialog behavior flags
    #[repr(transparent)]
    #[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
    pub struct FileDialogFlags: sys::ImGuiFileDialogFlags_ {
        /// No special flags
        const NONE = sys::ImGuiFileDialogFlags_None;
        /// Show confirmation dialog when overwriting files
        const CONFIRM_OVERWRITE = sys::ImGuiFileDialogFlags_ConfirmOverwrite;
        /// Don't show hidden files (files starting with .)
        const DONT_SHOW_HIDDEN_FILES = sys::ImGuiFileDialogFlags_DontShowHiddenFiles;
        /// Disable the create directory button
        const DISABLE_CREATE_DIRECTORY_BUTTON = sys::ImGuiFileDialogFlags_DisableCreateDirectoryButton;
        /// Hide the file type column
        const HIDE_COLUMN_TYPE = sys::ImGuiFileDialogFlags_HideColumnType;
        /// Hide the file size column
        const HIDE_COLUMN_SIZE = sys::ImGuiFileDialogFlags_HideColumnSize;
        /// Hide the file date column
        const HIDE_COLUMN_DATE = sys::ImGuiFileDialogFlags_HideColumnDate;
        /// Embed dialog in your own ImGui scope (no separate window)
        const NO_DIALOG = sys::ImGuiFileDialogFlags_NoDialog;
        /// Make filename field read-only for file open dialogs
        const READONLY_FILENAME_FIELD = sys::ImGuiFileDialogFlags_ReadOnlyFileNameField;
        /// Case insensitive file extension filtering
        // I took the liberty to fix the spelling of "extention"
        const CASE_INSENSITIVE_EXTENSION_FILTERING = sys::ImGuiFileDialogFlags_CaseInsensitiveExtentionFiltering;
        /// Make dialog modal
        const MODAL = sys::ImGuiFileDialogFlags_Modal;
        /// Disable thumbnail mode
        const DISABLE_THUMBNAIL_MODE = sys::ImGuiFileDialogFlags_DisableThumbnailMode;
        /// Disable place mode
        const DISABLE_PLACE_MODE = sys::ImGuiFileDialogFlags_DisablePlaceMode;
        /// Disable quick path selection
        const DISABLE_QUICK_PATH_SELECTION = sys::ImGuiFileDialogFlags_DisableQuickPathSelection;
        /// Show devices selection button
        const SHOW_DEVICES_BUTTON = sys::ImGuiFileDialogFlags_ShowDevicesButton;
        /// Enable natural sorting for filenames (slower than standard sorting)
        const NATURAL_SORTING = sys::ImGuiFileDialogFlags_NaturalSorting;

        /// Default behavior flags (most common use case).
        /// Equivalent to `CONFIRM_OVERWRITE | MODAL | HIDE_COLUMN_TYPE`
        const DEFAULT = sys::ImGuiFileDialogFlags_Default;
    }
}

bitflags! {
    /// File style flags for customizing file appearance
    #[repr(transparent)]
    #[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
    pub struct FileStyleFlags: sys::IGFD_FileStyleFlags_ {
        /// No style
        const NONE = sys::IGFD_FileStyle_None;
        /// Style for all files
        const BY_TYPE_FILE = sys::IGFD_FileStyleByTypeFile;
        /// Style for all directories
        const BY_TYPE_DIR = sys::IGFD_FileStyleByTypeDir;
        /// Style for all links
        const BY_TYPE_LINK = sys::IGFD_FileStyleByTypeLink;
        /// Style by file extension
        // I took the liberty to fix the spelling of "extention"
        const BY_EXTENSION = sys::IGFD_FileStyleByExtention;
        /// Style by full filename
        const BY_FULL_NAME = sys::IGFD_FileStyleByFullName;
        /// Style when criteria is contained in full name
        const BY_CONTAINED_IN_FULL_NAME = sys::IGFD_FileStyleByContainedInFullName;
    }
}
