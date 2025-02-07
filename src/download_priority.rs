#![allow(non_camel_case_types)]

/// libtorrent/download_priority.hpp
#[repr(u8)]
pub enum DownloadPriority {
    /// Don't download the file or piece. Partial pieces may still be downloaded when
    /// setting file priorities.
    /// constexpr download_priority_t dont_download{0};
    dont_download = 0,

    /// The default priority for files and pieces.
    /// constexpr download_priority_t default_priority{4};
    default_priority = 4,

    /// The lowest priority for files and pieces.
    /// constexpr download_priority_t low_priority{1};
    low_priority = 1,

    /// The highest priority for files and pieces.
    /// constexpr download_priority_t top_priority{7};
    top_priority = 7,
}
