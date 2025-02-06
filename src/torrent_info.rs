use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct FileEntry {
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
}

impl From<ffi::FileEntry> for FileEntry {
    fn from(f: ffi::FileEntry) -> Self {
        Self {
            file_path: f.file_path,
            file_name: f.file_name,
            file_size: f.file_size,
        }
    }
}

/// libtorrent/torrent_info.hpp
///
/// the torrent_info class holds the information found in a .torrent file.
#[derive(Serialize, Debug)]
pub struct TorrentInfo {
    /// The information about files in the torrent, including paths, sizes, and
    /// piece mapping
    pub files: Vec<FileEntry>,

    /// List of tracker URLs and their tier priority
    pub trackers: Vec<String>,

    /// List of similar torrents by their info-hash (BEP 38)
    pub similar_torrents: Vec<String>,

    /// List of collection names this torrent belongs to (BEP 38)
    pub collections: Vec<String>,

    /// List of web seed entries (HTTP/URL seeds)
    pub web_seeds: Vec<String>,

    /// If this torrent contains any DHT nodes, they are put in this vector in
    /// their original form (host name and port number).
    pub nodes: Vec<String>,

    /// the total number of bytes the torrent-file
    /// represents. Note that this is the number of pieces times the piece
    /// size (modulo the last piece possibly being smaller). With pad files,
    /// the total size will be larger than the sum of all (regular) file
    /// sizes.
    pub total_size: u64,

    /// ``piece_length`` and ``num_pieces`` are the number of byte
    /// for each piece and the total number of pieces, respectively. The
    /// difference between ``piece_size`` and ``piece_length`` is that
    /// ``piece_size`` takes the piece index as argument and gives you the
    /// exact size of that piece. It will always be the same as
    /// ``piece_length`` except in the case of the last piece, which may be
    /// smaller.
    pub piece_length: u32,
    pub num_pieces: u32,

    /// the number of blocks there are in the typical piece. There
    /// may be fewer in the last piece)
    pub blocks_per_piece: u32,

    /// the info-hash of the torrent. For BitTorrent v2 support, use
    /// ``info_hashes()`` to get an object that may hold both a v1 and v2
    /// info-hash
    pub info_hash: String,

    /// the number of files in the torrent
    pub num_files: u32,

    /// the name of the torrent.
    /// name contains UTF-8 encoded string.
    pub name: String,

    /// ``creation_date`` returns the creation date of the torrent as time_t
    /// (`posix time`_). If there's no time stamp in the torrent file, 0 is
    /// returned.
    /// .. _`posix time`: http://www.opengroup.org/onlinepubs/009695399/functions/time.html
    pub creation_date: i64,

    /// the creator string in the torrent. If there is
    /// no creator string it will return an empty string.
    pub creator: String,

    /// the comment associated with the torrent. If
    /// there's no comment, it will return an empty string.
    /// comment contains UTF-8 encoded string.
    pub comment: String,

    /// SSL certificate in x509 format (empty if not SSL torrent)
    pub ssl_cert: String,

    /// Flags indicating torrent properties
    pub is_private: bool, // True if this is a private torrent
    pub is_i2p: bool, // True if this is an i2p torrent
}

impl From<ffi::TorrentInfo> for TorrentInfo {
    fn from(t: ffi::TorrentInfo) -> Self {
        Self {
            files: t.files.into_iter().map(|f| f.into()).collect(),
            trackers: t.trackers,
            similar_torrents: t.similar_torrents,
            collections: t.collections,
            web_seeds: t.web_seeds,
            nodes: t.nodes,
            total_size: t.total_size,
            piece_length: t.piece_length,
            num_pieces: t.num_pieces,
            blocks_per_piece: t.blocks_per_piece,
            info_hash: t.info_hash,
            num_files: t.num_files,
            name: t.name,
            creation_date: t.creation_date,
            creator: t.creator,
            comment: t.comment,
            ssl_cert: t.ssl_cert,
            is_private: t.is_private,
            is_i2p: t.is_i2p,
        }
    }
}
