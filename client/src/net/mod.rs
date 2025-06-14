

#[cfg(not(target_arch = "wasm32"))]
pub type WebSocketClientBox = Box<dyn WebSocketClient + Send>;

#[cfg(target_arch = "wasm32")]
pub type WebSocketClientBox = Box<dyn WebSocketClient>;

pub enum ConnectionResult {
    Success(WebSocketClientBox),
    Failure(String),
}

pub trait WebSocketClient {
    fn send_text(&self, msg: &str);
    fn try_recv(&self) -> Option<WsMessage>;
    fn connection_failed(&self) -> bool;
}

#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use std::sync::mpsc::{self, Receiver};
    use tokio::{runtime::Runtime, sync::mpsc as tokio_mpsc};
    use tokio_tungstenite::connect_async;
    use tungstenite::Message;

    struct NativeClient {
        rx: Receiver<WsMessage>,
        tx: tokio_mpsc::UnboundedSender<WsMessage>,
    }

    impl WebSocketClient for NativeClient {
        fn send_text(&self, msg: &str) {
            let _ = self.tx.send(WsMessage::Text(msg.to_string()));
        }

        fn try_recv(&self) -> Option<WsMessage> {
            self.rx.try_recv().ok()
        }

        fn connection_failed(&self) -> bool {
            false // not used in native
        }
    }

    pub async fn connect(url: &str) -> ConnectionResult {
        let (msg_tx, msg_rx) = mpsc::channel();
        let (send_tx, mut send_rx) = tokio_mpsc::unbounded_channel();
        let url = url.to_string();
        let (result_tx, result_rx) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                match connect_async(&url).await {
                    Ok((ws_stream, _)) => {
                        let (mut write, mut read) = ws_stream.split();

                        result_tx.send(ConnectionResult::Success(Box::new(NativeClient {
                            rx: msg_rx,
                            tx: send_tx.clone(),
                        }))).unwrap();

                        tokio::spawn(async move {
                            while let Some(msg) = read.next().await {
                                if let Ok(Message::Text(txt)) = msg {
                                    let _ = msg_tx.send(WsMessage::Text(txt.to_string()));
                                }
                            }
                        });

                        while let Some(msg) = send_rx.recv().await {
                            let _ = match msg {
                                WsMessage::Text(txt) => write.send(Message::Text(txt.into())).await,
                                WsMessage::Binary(data) => write.send(Message::Binary(data.into())).await,
                            };
                        }
                    }
                    Err(e) => {
                        result_tx.send(ConnectionResult::Failure(format!("Failed to connect: {:?}", e))).unwrap();
                    }
                }
            });
        });

        result_rx.recv().unwrap_or(ConnectionResult::Failure("Thread join failed".into()))
    }
}

#[cfg(target_arch = "wasm32")]
pub mod platform {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, CloseEvent, MessageEvent, WebSocket};
    use futures::future::poll_fn;

    struct WasmClient {
        socket: WebSocket,
        queue: Rc<RefCell<Vec<WsMessage>>>,
        failed: Rc<RefCell<bool>>,
        _onmessage: Closure<dyn FnMut(MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::Event)>,
        _onclose: Closure<dyn FnMut(CloseEvent)>,
    }

    impl WebSocketClient for WasmClient {
        fn send_text(&self, msg: &str) {
            let _ = self.socket.send_with_str(msg);
        }

        fn try_recv(&self) -> Option<WsMessage> {
            self.queue.borrow_mut().pop()
        }

        fn connection_failed(&self) -> bool {
            *self.failed.borrow()
        }
    }

    async fn wait_for_open_or_fail(socket: &WebSocket, failed: Rc<RefCell<bool>>) -> Result<(), ()> {
        let opened = Rc::new(RefCell::new(false));
        let opened_clone = opened.clone();
        let failed_clone1 = failed.clone();
        let failed_clone2 = failed.clone();

        // onopen sets opened to true
        let onopen = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            *opened_clone.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        socket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        // onerror sets failed to true
        let onerror = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            *failed_clone1.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        socket.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        // onclose also sets failed to true
        let onclose = Closure::wrap(Box::new(move |_e: CloseEvent| {
            *failed_clone2.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();

        // Wait until opened or failed is true
        poll_fn(move |_cx| {
            if *opened.borrow() {
                std::task::Poll::Ready(Ok(()))
            } else if *failed.borrow() {
                std::task::Poll::Ready(Err(()))
            } else {
                std::task::Poll::Pending
            }
        })
        .await
    }

    pub async fn connect(url: &str) -> ConnectionResult {
        let socket = match WebSocket::new(url) {
            Ok(s) => s,
            Err(e) => return ConnectionResult::Failure(format!("WebSocket init error: {:?}", e)),
        };

        socket.set_binary_type(BinaryType::Arraybuffer);

        let failed = Rc::new(RefCell::new(false));

        // Await socket open or failure
        if wait_for_open_or_fail(&socket, failed.clone()).await.is_err() {
            return ConnectionResult::Failure("WebSocket failed to open or connect".to_string());
        }

        let queue = Rc::new(RefCell::new(Vec::new()));

        // Setup onmessage handler
        let queue_clone = queue.clone();
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                queue_clone.borrow_mut().insert(0, WsMessage::Text(txt.into()));
            }
        }) as Box<dyn FnMut(_)>);
        socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        // onerror and onclose handlers update failed flag after connection
        let failed_clone = failed.clone();
        let onerror = Closure::wrap(Box::new(move |_e| {
            *failed_clone.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        socket.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let failed_clone = failed.clone();
        let onclose = Closure::wrap(Box::new(move |_e| {
            *failed_clone.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));


        ConnectionResult::Success(Box::new(WasmClient {
            socket,
            queue,
            failed,
            _onmessage: onmessage,
            _onerror: onerror,
            _onclose: onclose,
        }))
    }
}


#[cfg(not(target_arch = "wasm32"))]
pub use platform::connect;

#[cfg(target_arch = "wasm32")]
pub use platform::connect;

#[derive(Debug)]
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
}
