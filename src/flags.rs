use std::ops::{BitAnd, BitOr, BitXor, Not};

/// File dialog behavior flags
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct FileDialogFlags(pub sys::ImGuiFileDialogFlags_);

impl FileDialogFlags {
    /// No special flags
    pub const NONE: Self = Self(sys::ImGuiFileDialogFlags_None);
    /// Show confirmation dialog when overwriting files
    pub const CONFIRM_OVERWRITE: Self = Self(sys::ImGuiFileDialogFlags_ConfirmOverwrite);
    /// Don't show hidden files (files starting with .)
    pub const DONT_SHOW_HIDDEN_FILES: Self = Self(sys::ImGuiFileDialogFlags_DontShowHiddenFiles);
    /// Disable the create directory button
    pub const DISABLE_CREATE_DIRECTORY_BUTTON: Self =
        Self(sys::ImGuiFileDialogFlags_DisableCreateDirectoryButton);
    /// Hide the file type column
    pub const HIDE_COLUMN_TYPE: Self = Self(sys::ImGuiFileDialogFlags_HideColumnType);
    /// Hide the file size column
    pub const HIDE_COLUMN_SIZE: Self = Self(sys::ImGuiFileDialogFlags_HideColumnSize);
    /// Hide the file date column
    pub const HIDE_COLUMN_DATE: Self = Self(sys::ImGuiFileDialogFlags_HideColumnDate);
    /// Embed dialog in your own ImGui scope (no separate window)
    pub const NO_DIALOG: Self = Self(sys::ImGuiFileDialogFlags_NoDialog);
    /// Make filename field read-only for file open dialogs
    pub const READONLY_FILENAME_FIELD: Self = Self(sys::ImGuiFileDialogFlags_ReadOnlyFileNameField);
    /// Case insensitive file extension filtering
    // I took the liberty to fix the spelling of "extention"
    pub const CASE_INSENSITIVE_EXTENSION_FILTERING: Self =
        Self(sys::ImGuiFileDialogFlags_CaseInsensitiveExtentionFiltering);
    /// Make dialog modal
    pub const MODAL: Self = Self(sys::ImGuiFileDialogFlags_Modal);
    /// Disable thumbnail mode
    pub const DISABLE_THUMBNAIL_MODE: Self = Self(sys::ImGuiFileDialogFlags_DisableThumbnailMode);
    /// Disable place mode
    pub const DISABLE_PLACE_MODE: Self = Self(sys::ImGuiFileDialogFlags_DisablePlaceMode);
    /// Disable quick path selection
    pub const DISABLE_QUICK_PATH_SELECTION: Self =
        Self(sys::ImGuiFileDialogFlags_DisableQuickPathSelection);
    /// Show devices selection button
    pub const SHOW_DEVICES_BUTTON: Self = Self(sys::ImGuiFileDialogFlags_ShowDevicesButton);
    /// Enable natural sorting for filenames (slower than standard sorting)
    pub const NATURAL_SORTING: Self = Self(sys::ImGuiFileDialogFlags_NaturalSorting);

    /// Default behavior flags (most common use case).
    /// Equivalent to `CONFIRM_OVERWRITE | MODAL | HIDE_COLUMN_TYPE`
    pub const DEFAULT: Self = Self(sys::ImGuiFileDialogFlags_Default);

    /// Create an empty set of flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create a set with all flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self(!0)
    }

    /// Get the underlying bits value
    #[inline]
    pub const fn bits(&self) -> sys::ImGuiFileDialogFlags_ {
        self.0
    }

    /// Create from raw bits value
    #[inline]
    pub const fn from_bits(bits: sys::ImGuiFileDialogFlags_) -> Option<Self> {
        Some(Self(bits))
    }

    /// Create from raw bits value, truncating invalid bits
    #[inline]
    pub const fn from_bits_truncate(bits: sys::ImGuiFileDialogFlags_) -> Self {
        Self(bits)
    }

    /// Check if no flags are set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if all flags are set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0 == !0
    }

    /// Check if any flags in `other` are set
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// Check if all flags in `other` are set
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Insert flags
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Remove flags
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggle flags
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Set flags to the specified value
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    /// Get the intersection of two sets of flags
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Get the union of two sets of flags
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Get the difference between two sets of flags
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Get the symmetric difference between two sets of flags
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Get the complement of the flags
    #[inline]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }
}

impl Default for FileDialogFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl BitOr for FileDialogFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for FileDialogFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for FileDialogFlags {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for FileDialogFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for FileDialogFlags {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for FileDialogFlags {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for FileDialogFlags {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl From<sys::ImGuiFileDialogFlags_> for FileDialogFlags {
    #[inline]
    fn from(value: sys::ImGuiFileDialogFlags_) -> Self {
        Self(value)
    }
}

impl From<FileDialogFlags> for sys::ImGuiFileDialogFlags_ {
    #[inline]
    fn from(value: FileDialogFlags) -> Self {
        value.0
    }
}

/// File style flags for customizing file appearance
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct FileStyleFlags(pub sys::IGFD_FileStyleFlags_);

impl FileStyleFlags {
    /// No style
    pub const NONE: Self = Self(sys::IGFD_FileStyle_None);
    /// Style for all files
    pub const BY_TYPE_FILE: Self = Self(sys::IGFD_FileStyleByTypeFile);
    /// Style for all directories
    pub const BY_TYPE_DIR: Self = Self(sys::IGFD_FileStyleByTypeDir);
    /// Style for all links
    pub const BY_TYPE_LINK: Self = Self(sys::IGFD_FileStyleByTypeLink);
    /// Style by file extension
    // I took the liberty to fix the spelling of "extention"
    pub const BY_EXTENSION: Self = Self(sys::IGFD_FileStyleByExtention);
    /// Style by full filename
    pub const BY_FULL_NAME: Self = Self(sys::IGFD_FileStyleByFullName);
    /// Style when criteria is contained in full name
    pub const BY_CONTAINED_IN_FULL_NAME: Self = Self(sys::IGFD_FileStyleByContainedInFullName);

    /// Create an empty set of flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create a set with all flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self(!0)
    }

    /// Get the underlying bits value
    #[inline]
    pub const fn bits(&self) -> sys::IGFD_FileStyleFlags_ {
        self.0
    }

    /// Create from raw bits value
    #[inline]
    pub const fn from_bits(bits: sys::IGFD_FileStyleFlags_) -> Option<Self> {
        Some(Self(bits))
    }

    /// Create from raw bits value, truncating invalid bits
    #[inline]
    pub const fn from_bits_truncate(bits: sys::IGFD_FileStyleFlags_) -> Self {
        Self(bits)
    }

    /// Check if no flags are set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if all flags are set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.0 == !0
    }

    /// Check if any flags in `other` are set
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    /// Check if all flags in `other` are set
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Insert flags
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Remove flags
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    /// Toggle flags
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    /// Set flags to the specified value
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    /// Get the intersection of two sets of flags
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Get the union of two sets of flags
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Get the difference between two sets of flags
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Get the symmetric difference between two sets of flags
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Get the complement of the flags
    #[inline]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }
}

impl Default for FileStyleFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl BitOr for FileStyleFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for FileStyleFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for FileStyleFlags {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for FileStyleFlags {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for FileStyleFlags {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for FileStyleFlags {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for FileStyleFlags {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl From<sys::IGFD_FileStyleFlags_> for FileStyleFlags {
    #[inline]
    fn from(value: sys::IGFD_FileStyleFlags_) -> Self {
        Self(value)
    }
}

impl From<FileStyleFlags> for sys::IGFD_FileStyleFlags_ {
    #[inline]
    fn from(value: FileStyleFlags) -> Self {
        value.0
    }
}

// Import the BitOrAssign and BitAndAssign traits
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};
