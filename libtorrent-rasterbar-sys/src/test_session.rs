#[cfg(test)]
mod tests {
    use cxx::UniquePtr;

    use crate::{
        ffi::{create_session, ParamPair, Session},
        flags::SaveStateFlags,
    };

    fn create_default_session() -> anyhow::Result<UniquePtr<Session>> {
        let ses = create_session(
            &[
                ParamPair {
                    key: "user_agent",
                    value: "libtorrent-rs/2.0.11",
                },
                ParamPair {
                    key: "alert_mask",
                    value: "error,peer,port_mapping,storage,tracker,connect,status,ip_block,performance_warning,dht,incoming_request,dht_operation,port_mapping_log,file_progress",
                },
            ],
            SaveStateFlags::SaveDhtState.bits(),
            "/tmp/t/libtest/ses.state",
            "/tmp/t/libtest/resume",
            "/tmp/t/libtest/torrents",
            20,
        )?;

        Ok(ses)
    }

    #[test]
    fn test_create_session() {
        let ses = create_default_session().unwrap();
        assert!(!ses.is_null());

        std::thread::sleep(std::time::Duration::from_secs(5000));
    }

    #[test]
    fn test_add_magnet() {
        let ses = create_default_session().unwrap();
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5000));
    }

    #[test]
    fn test_get_torrents() {
        let ses = create_default_session().unwrap();
        assert!(!ses.is_null());

        ses.add_torrent(
            "/tmp/t/START-243.torrent",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(20));

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let t = h.get_torrent_info();

        assert!(!t.files.is_empty());

        println!("{:#?}", t);
    }

    #[test]
    fn test_get_peers() {
        let mut ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let peers = h.get_peers();
        println!("----- 5");
        println!("peers: {:#?}", peers);
        std::thread::sleep(std::time::Duration::from_secs(1));

        let peers = h.get_peers();
        println!("----- 5");
        println!("peers: {:#?}", peers);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_get_file_progress() {
        let mut ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let progress = h.get_file_progress(true);
        println!("----- 5");
        println!("progress: {:#?}", progress);
        std::thread::sleep(std::time::Duration::from_secs(1));

        let progress = h.get_file_progress(true);
        println!("----- 5");
        println!("progress: {:#?}", progress);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_get_piece_availability() {
        let mut ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let availability = h.get_piece_availability();
        println!("----- 5");
        println!("availability: {:#?}", availability);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let availability = h.get_piece_availability();
        println!("----- 5");
        println!("availability: {:#?}", availability);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_get_trackers() {
        let mut ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let trackers = h.get_trackers();
        println!("----- 5");
        println!("trackers: {:#?}", trackers);
        std::thread::sleep(std::time::Duration::from_secs(10));
        let trackers = h.get_trackers();
        println!("----- 5");
        println!("trackers: {:#?}", trackers);
    }

    #[test]
    fn test_get_piece_info() {
        let mut ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/FRD-015",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        loop {
            let piece_info = h.get_piece_info();
            println!("----- 5");
            println!("piece_info: {:#?}", piece_info);
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }

    #[test]
    fn test_get_torrent_status() {
        let ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tts = ses.get_torrents();
        assert!(!tts.is_empty());
        println!("{:#?}", tts);

        let h = ses.get_torrent_handle(&tts[0].info_hash);
        assert!(h.is_valid());

        let ts = h.get_torrent_status();
        println!("----- 5");
        println!("trackers: {:#?}", ts);
        std::thread::sleep(std::time::Duration::from_secs(10));
        let ts = h.get_torrent_status();
        println!("----- 5");
        println!("trackers: {:#?}", ts);
    }

    #[test]
    fn test_get_session_stats() {
        let ses = create_default_session().unwrap();

        println!("----- 1");
        assert!(!ses.is_null());

        ses.add_magnet(
            "magnet:?xt=urn:btih:771cc76f2c725648b20a74d27ffeba2458a88343",
            &[
                ParamPair {
                    key: "max_connections",
                    value: "100",
                },
                ParamPair {
                    key: "max_uploads",
                    value: "-1",
                },
                ParamPair {
                    key: "save_path",
                    value: "/tmp/t/libtest/items/3574438",
                },
            ],
        )
        .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("----- 2");

        let tss = ses.get_stats();
        println!("{:#?}", tss);
    }
}
