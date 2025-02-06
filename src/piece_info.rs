use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

/// libtorrent/torrent_handle.hpp
///
// holds the state of a block in a piece. Who we requested
// it from and how far along we are at downloading it.
#[derive(Serialize, Debug)]
pub struct BlockInfo {
    // the number of bytes that have been received for this block
    bytes_progress: u32, // default 15

    // the total number of bytes in this block.
    block_size: u32, // default 15

    // the state this block is in (see block_state_t)
    // this is the enum used for the block_info::state field.
    //
    // enum block_state_t
    // {
    // 	// This block has not been downloaded or requested form any peer.
    // 	none,
    // 	// The block has been requested, but not completely downloaded yet.
    // 	requested,
    // 	// The block has been downloaded and is currently queued for being
    // 	// written to disk.
    // 	writing,
    // 	// The block has been written to disk.
    // 	finished
    // };
    state: u8, // default 2

    // the number of peers that is currently requesting this block. Typically
    // this is 0 or 1, but at the end of the torrent blocks may be requested
    // by more peers in parallel to speed things up.
    num_peers: u32, // default 14
}

impl From<ffi::BlockInfo> for BlockInfo {
    fn from(bi: ffi::BlockInfo) -> Self {
        Self {
            bytes_progress: bi.bytes_progress,
            block_size: bi.block_size,
            state: bi.state,
            num_peers: bi.num_peers,
        }
    }
}

/// libtorrent/torrent_handle.hpp
///
/// This class holds information about pieces that have outstanding requests
/// or outstanding writes
#[derive(Serialize, Debug)]
pub struct PartialPieceInfo {
    // the index of the piece in question. ``blocks_in_piece`` is the number
    // of blocks in this particular piece. This number will be the same for
    // most pieces, but
    // the last piece may have fewer blocks than the standard pieces.
    pub piece_index: i32,

    // the number of blocks in this piece
    pub blocks_in_piece: i32,

    // the number of blocks that are in the finished state
    pub finished: i32,

    // the number of blocks that are in the writing state
    pub writing: i32,

    // the number of blocks that are in the requested state
    pub requested: i32,

    // this is an array of ``blocks_in_piece`` number of
    // items. One for each block in the piece.
    //
    // .. warning:: This is a pointer that points to an array
    //	that's owned by the session object. The next time
    //	get_download_queue() is called, it will be invalidated.
    //	In the case of piece_info_alert, these pointers point into the alert
    //	object itself, and will be invalidated when the alert destruct.
    pub blocks: Vec<BlockInfo>,
}

impl From<ffi::PartialPieceInfo> for PartialPieceInfo {
    fn from(ppi: ffi::PartialPieceInfo) -> Self {
        Self {
            piece_index: ppi.piece_index,
            blocks_in_piece: ppi.blocks_in_piece,
            finished: ppi.finished,
            writing: ppi.writing,
            requested: ppi.requested,
            blocks: ppi.blocks.into_iter().map(BlockInfo::from).collect(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PieceInfo {
    partial_pieces: Vec<PartialPieceInfo>,
    blocks: Vec<BlockInfo>,
}

impl From<ffi::PieceInfo> for PieceInfo {
    fn from(pi: ffi::PieceInfo) -> Self {
        Self {
            partial_pieces: pi.partial_pieces.into_iter().map(PartialPieceInfo::from).collect(),
            blocks: pi.blocks.into_iter().map(BlockInfo::from).collect(),
        }
    }
}
