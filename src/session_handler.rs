use bitflags::bitflags;

bitflags! {
    /// libtorrent/session_handler.hpp
    /// These flags are defined in struct session_handle
    pub struct SaveStateFlags: u32 {
        // saves settings (i.e. the settings_pack)
        // static constexpr save_state_flags_t save_settings = 0_bit;
        const SaveSettings = 1 << 0;
        // saves dht state such as nodes and node-id, possibly accelerating
        // joining the DHT if provided at next session startup.
        // static constexpr save_state_flags_t save_dht_state = 2_bit;
        const SaveDhtState = 1 << 2;
        // load or save state from plugins
        // static constexpr save_state_flags_t save_extension_state = 11_bit;
        const SaveExtensionState = 1 << 11;

        // load or save the IP filter set on the session
        // static constexpr save_state_flags_t save_ip_filter = 12_bit;
        const SaveIpFilter = 1 << 12;

        const All = !0;
    }
}
