// Copyright (c) 2018-2020 Sean McArthur
// Licensed under the MIT license http://opensource.org/licenses/MIT
// port from https://github.com/seanmonstar/warp/blob/master/src/filters/sse.rs
//! Middleware for Server-Sent Events (SSE)
//!
//! # Example
//!
//! ```no_run
//! use std::time::Duration;
//! use std::convert::Infallible;
//! use futures_util::stream::iter;
//! use futures_util::Stream;
//!
//! use salvo_core::prelude::*;
//! use salvo_extra::sse::{self, SseEvent};
//!
//! fn sse_events() -> impl Stream<Item = Result<SseEvent, Infallible>> {
//!     iter(vec![
//!         Ok(SseEvent::default().text("unnamed event")),
//!         Ok(
//!             SseEvent::default().name("chat")
//!             .text("chat message")
//!         ),
//!         Ok(
//!             SseEvent::default().id(13.to_string())
//!             .name("chat")
//!             .text("other chat message\nwith next line")
//!             .retry(Duration::from_millis(5000))
//!         )
//!     ])
//! }
//! #[handler]
//! async fn handle(res: &mut Response) {
//!     sse::stream(res, sse_events());
//! }
//! #[tokio::main]
//! async fn main() {
//!     let router = Router::with_path("push-notifications").get(handle);
//!     let accepor = TcpListener::new("127.0.0.1:5800").bind().await;
//!     Server::new(accepor).serve(router).await;
//! }
//! ```
//!
//! Each field already is event which can be sent to client.
//! The events with multiple fields can be created by combining fields using tuples.
//!
//! See also the [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) API,
//! which specifies the expected behavior of Server Sent Events.

use futures_util::AsyncWriteExt;
use salvo_core::http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE};
use salvo_core::http::Response;
use serde::Serialize;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};
use std::future::Future;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc;
use tokio::time::{self};
use tracing::info;

/// Server-sent event data type
#[derive(Clone, Debug)]
enum DataType {
    Text(String),
    Json(String),
}
/// SseError
#[derive(Debug)]
pub struct SseError;

impl Display for SseError {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "sse error")
    }
}

impl StdError for SseError {}
/// Server-sent event
#[derive(Default, Clone, Debug)]
pub struct SseEvent {
    name: Option<String>,
    id: Option<String>,
    data: Option<DataType>,
    comment: Option<String>,
    retry: Option<Duration>,
}

impl SseEvent {
    /// Sets Server-sent event data.
    #[inline]
    pub fn text<T: Into<String>>(mut self, data: T) -> SseEvent {
        self.data = Some(DataType::Text(data.into()));
        self
    }

    /// Sets Server-sent event data.
    #[inline]
    pub fn json<T: Serialize>(mut self, data: T) -> Result<SseEvent, serde_json::Error> {
        self.data = Some(DataType::Json(serde_json::to_string(&data)?));
        Ok(self)
    }

    /// Sets Server-sent event comment.`
    #[inline]
    pub fn comment<T: Into<String>>(mut self, comment: T) -> SseEvent {
        self.comment = Some(comment.into());
        self
    }

    /// Sets Server-sent event event.
    #[inline]
    pub fn name<T: Into<String>>(mut self, event: T) -> SseEvent {
        self.name = Some(event.into());
        self
    }

    /// Sets Server-sent event retry.
    #[inline]
    pub fn retry(mut self, duration: Duration) -> SseEvent {
        self.retry = Some(duration);
        self
    }

    /// Sets Server-sent event id.
    #[inline]
    pub fn id<T: Into<String>>(mut self, id: T) -> SseEvent {
        self.id = Some(id.into());
        self
    }
}

impl Display for SseEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(comment) = &self.comment {
            ":".fmt(f)?;
            comment.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(name) = &self.name {
            "event:".fmt(f)?;
            name.fmt(f)?;
            f.write_char('\n')?;
        }

        match self.data {
            Some(DataType::Text(ref data)) => {
                for line in data.split('\n') {
                    "data:".fmt(f)?;
                    line.fmt(f)?;
                    f.write_char('\n')?;
                }
            }
            Some(DataType::Json(ref data)) => {
                "data:".fmt(f)?;
                data.fmt(f)?;
                f.write_char('\n')?;
            }
            None => {}
        }

        if let Some(ref id) = &self.id {
            "id:".fmt(f)?;
            id.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(ref duration) = &self.retry {
            "retry:".fmt(f)?;

            let secs = duration.as_secs();
            let millis = duration.subsec_millis();

            if secs > 0 {
                // format seconds
                secs.fmt(f)?;

                // pad milliseconds
                if millis < 10 {
                    f.write_str("00")?;
                } else if millis < 100 {
                    f.write_char('0')?;
                }
            }

            // format milliseconds
            millis.fmt(f)?;

            f.write_char('\n')?;
        }

        f.write_char('\n')?;
        Ok(())
    }
}

/// SseKeepAlive
#[allow(missing_debug_implementations)]
pub struct SseKeepAlive {
    /// Comment field.
    pub comment: Cow<'static, str>,
    /// Max interval between keep-alive messages.
    pub max_interval: Duration,
}


#[derive(Debug)]
pub enum SSEMessage {
    Reply(SseEvent),
    Exit()
}


impl SseKeepAlive {
    /// Create new `SseKeepAlive`.
    #[inline]
    pub fn new() -> SseKeepAlive {
        SseKeepAlive {
            comment: Cow::Borrowed(""),
            max_interval: Duration::from_secs(15),
        }
    }

    /// Customize the interval between keep-alive messages.
    ///
    /// Default is 15 seconds.
    #[inline]
    pub fn max_interval(mut self, time: Duration) -> Self {
        self.max_interval = time;
        self
    }

    /// Customize the text of the keep-alive message.
    ///
    /// Default is an empty comment.
    #[inline]
    pub fn comment(mut self, comment: impl Into<Cow<'static, str>>) -> Self {
        self.comment = comment.into();
        self
    }

    /// Send events.
    #[inline]
    pub async fn stream<F, Fut>(self, res: &mut Response, mut rx: UnboundedReceiver<SSEMessage>, on_close: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        write_response_headers(res);
        let mut tx = res.channel();
        let comment = self.comment;
        let max_interval = self.max_interval;

        tokio::spawn(async move {
            let alive_timer = tokio::time::sleep(max_interval);
            tokio::pin!(alive_timer);

            loop {
                tokio::select! {
                    event = rx.recv() => {
                        match event {
                            Some(event) => {
                                match event {
                                    SSEMessage::Reply(event) => {
                                        let msg = event.to_string();
                                        if let Err(e) = tx.write_all(msg.as_bytes()).await {
                                            info!("SSE connection closed: {}", e);
                                            break;
                                        }
                                    }
                                    SSEMessage::Exit() => {
                                        break;
                                    }
                                }
                                alive_timer.as_mut().reset(time::Instant::now() + max_interval);
                            }
                            None => break,
                        }
                    }
                    _ = &mut alive_timer => {
                        let event = SseEvent::default().comment(comment.clone());
                        let msg = event.to_string();
                        if let Err(e) = tx.write_all(msg.as_bytes()).await {
                            info!("SSE connection closed: {}", e);
                            break;
                        }
                        alive_timer.as_mut().reset(time::Instant::now() + max_interval);
                    }
                }
            }
            info!("SSE connection closed normally");
            on_close().await;
        });
    }
}

#[inline]
fn write_response_headers(res: &mut Response) {
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
    res.headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
}

/// Send events.
#[inline]
pub fn stream(res: &mut Response, mut rx: mpsc::Receiver<SseEvent>) {
    write_response_headers(res);
    let mut tx = res.channel();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            let msg = event.to_string();
            if let Err(e) = tx.write_all(msg.as_bytes()).await {
                info!("SSE connection closed: {}", e);
                break;
            }
        }
        info!("SSE connection closed normally");
    });
}

