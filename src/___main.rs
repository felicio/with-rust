use waku_bindings::{
    waku_store_query, ContentFilter, StoreQuery, WakuContentTopic, WakuPubSubTopic,
};

fn main() {
    let response = waku_store_query(
        {
            &StoreQuery {
                pubsub_topic: Option<&WakuPubSubTopic>,
                content_filters: vec![ContentFilter {
                    content_topic: WakuContentTopic {
                        application_name: String::from(""),
                        version: None,
                        content_topic_name: String::from(""),
                        encoding: None,
                    },
                }],
            }
        },
        &String::from(""),
        5,
    );
}
