#[cfg(test)]
mod tests {
    use crate::LTSession;
    use libtorrent_rasterbar_sys::flags::SaveStateFlags;

    fn create_default_session() -> anyhow::Result<LTSession> {
        let ses = LTSession::new(&[
            ("user_agent", "libtorrent-rs/2.0.11"),
            ("alert_mask", "error,peer,port_mapping,storage,tracker,connect,status,ip_block,performance_warning,dht,incoming_request,dht_operation,port_mapping_log,file_progress",
            )
        ],
            SaveStateFlags::save_dht_state.bits(),
            "/tmp/t/libtest/ses.state",
            "/tmp/t/libtest/resume",
            "/tmp/t/libtest/torrents",
            20,
        )?;

        Ok(ses)
    }

    #[test]
    fn test_add_magnet() {
        let ses = create_default_session().unwrap();

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ("max_connections", "100"),
                ("max_uploads", "-1"),
                ("save_path", "/tmp/t/libtest/items/3574438"),
            ],
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(10));

        std::thread::sleep(std::time::Duration::from_secs(100));
    }
}
