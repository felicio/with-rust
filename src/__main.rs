mod protocol;

use crate::protocol::{Chat2Message, TOY_CHAT_CONTENT_TOPIC};
use chrono::Utc;
use prost::Message;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use waku_bindings::{
    waku_new, ContentFilter, Multiaddr, PagingOptions, ProtocolId, Running, StoreQuery,
    WakuNodeHandle,
};

#[derive(Debug)]
struct PrettyMessage {
    timestamp: String,
    nick: String,
    message: String,
}

const NODES: &[&str] = &[
    // "/dns4/node-01.ac-cn-hongkong-c.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAkvWiyFsgRhuJEb9JfjYxEkoHLgnUQmr1N5mKWnYjxYRVm",
    "/dns4/node-01.do-ams3.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAmPLe7Mzm8TsYUubgCAW1aJoeFScxrLj8ppHFivPo97bUZ",
    // "/dns4/node-01.gc-us-central1-a.wakuv2.test.statusim.net/tcp/30303/p2p/16Uiu2HAmJb2e28qLXxT5kZxVUUoJt72EMzNGXB47Rxx5hw3q4YjS"
];

fn retrieve_history(
    node_handle: &WakuNodeHandle<Running>,
) -> waku_bindings::Result<Vec<Chat2Message>> {
    let self_id = node_handle.peer_id().unwrap();
    let peer = node_handle
        .peers()?
        .iter()
        .cloned()
        .find(|peer| peer.peer_id() != &self_id)
        .unwrap();

    let result = node_handle.store_query(
        &StoreQuery {
            pubsub_topic: None,
            content_filters: vec![ContentFilter::new(TOY_CHAT_CONTENT_TOPIC.clone())],
            start_time: Some(
                (Duration::from_secs(Utc::now().timestamp() as u64)
                    - Duration::from_secs(60 * 60 * 24))
                .as_nanos() as usize,
            ),
            end_time: None,
            paging_options: Some(PagingOptions {
                page_size: 25,
                cursor: None,
                forward: true,
            }),
        },
        peer.peer_id(),
        Some(Duration::from_secs(10)),
    )?;

    Ok(result
        .messages()
        .iter()
        .map(|waku_message| {
            <Chat2Message as Message>::decode(waku_message.payload())
                .expect("Toy chat messages should be decodeable")
        })
        .collect())
}

fn main() {
    // Start
    let node_handle = waku_new(None).unwrap();
    let node_handle = node_handle.start().unwrap();

    for address in NODES.iter().map(|a| Multiaddr::from_str(a).unwrap()) {
        let peerid = node_handle.add_peer(&address, ProtocolId::Relay).unwrap();
        node_handle.connect_peer_with_id(peerid, None);
    }

    // Fetch
    let messages: Arc<RwLock<Vec<Chat2Message>>> = Arc::new(RwLock::new(Vec::new()));
    let history = retrieve_history(&node_handle).unwrap();
    if !history.is_empty() {
        *messages.write().unwrap() = history;
    }

    let result = messages
        .read()
        .unwrap()
        .iter()
        .map(|message| PrettyMessage {
            timestamp: message
                .timestamp()
                .unwrap()
                .format("%d-%m-%y %H:%M")
                .to_string(),
            nick: message.nick().to_string(),
            message: message.message(),
        })
        .collect::<Vec<PrettyMessage>>();

    println!("{:?}", result);

    // Stop
    node_handle.stop();
}
