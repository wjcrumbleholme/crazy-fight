

#[derive(Debug)]
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
}

#[cfg(not(target_arch = "wasm32"))]
pub mod ws {
    use super::WsMessage;
    use tokio::{runtime::Runtime, sync::mpsc as tokio_mpsc};
    use tokio_tungstenite::connect_async;
    use futures_util::{SinkExt, StreamExt};
    use std::sync::mpsc::{self, Receiver};

    pub struct WsClient {
        pub rx: Receiver<WsMessage>,
        pub tx: tokio_mpsc::UnboundedSender<WsMessage>,
    }

    pub fn connect(url: &str) -> WsClient {
        let (msg_tx, msg_rx) = mpsc::channel();
        let (send_tx, mut send_rx) = tokio_mpsc::unbounded_channel::<WsMessage>();

        let url = url.to_string();
        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let (ws_stream, _) = connect_async(url).await.unwrap();

                let (mut write, mut read) = ws_stream.split();

                // Receiving task
                tokio::spawn(async move {
                    while let Some(msg) = read.next().await {
                        if let Ok(tungstenite::Message::Text(txt)) = msg {
                            msg_tx.send(WsMessage::Text(txt.to_string())).ok();
                        }
                    }
                });

                // Sending task
                while let Some(msg) = send_rx.recv().await {
                    let _ = match msg {
                        WsMessage::Text(txt) => write.send(tungstenite::Message::Text(txt.into())).await,
                        WsMessage::Binary(data) => write.send(tungstenite::Message::Binary(data.into())).await,
                    };
                }
            });
        });

        WsClient { rx: msg_rx, tx: send_tx }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod ws {
    use super::WsMessage;
    use wasm_bindgen::{closure::Closure, JsCast};
    use web_sys::{WebSocket, MessageEvent};
    use std::rc::Rc;
    use std::cell::RefCell;

    pub struct WsClient {
        socket: WebSocket,
        message_queue: Rc<RefCell<Vec<WsMessage>>>,
        _onmessage: Closure<dyn FnMut(MessageEvent)>,
    }

    impl WsClient {
        pub fn send(&self, msg: &str) {
            let _ = self.socket.send_with_str(msg);
        }

        pub fn try_recv(&self) -> Option<WsMessage> {
            self.message_queue.borrow_mut().pop()
        }
    }

    pub fn connect(url: &str) -> WsClient {
        let socket = WebSocket::new(url).unwrap();
        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let queue = Rc::new(RefCell::new(Vec::<WsMessage>::new()));
        let queue_clone = queue.clone();

        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                queue_clone.borrow_mut().insert(0, WsMessage::Text(txt.into()));
            }
        }) as Box<dyn FnMut(_)>);
        socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        WsClient {
            socket,
            message_queue: queue,
            _onmessage: onmessage,
        }
    }
}