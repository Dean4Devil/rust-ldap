extern crate byteorder;
extern crate futures;

use futures::prelude::*;
use std::os::unix::io::{IntoRawFd, RawFd};

/// LDAP Socket
///
/// A LSocket considers itself the *sole* owner of a file descriptor. *DO NOT* give a LSocket a
/// cloned Fd in an attempt to multiplex connections.
#[derive(PartialEq, Eq, Debug)]
struct LSocket {
    inner: RawFd,
    tmp: Option<<Self as Sink>::SinkItem>,
}

impl LSocket {
    pub fn new<F: IntoRawFd>(fd: F) -> Self {
        LSocket {
            inner: fd.into_raw_fd(),
            tmp: None,
        }
    }
}

impl Sink for LSocket {
    type SinkItem = Frame;
    type SinkError = LSocketError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        match self.tmp {
            Some(_) => Ok(AsyncSink::NotReady(item)),
            None => {
                self.tmp = Some(item);
                Ok(AsyncSink::Ready)
            }
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::NotReady)
    }
}

impl Stream for LSocket {
    type Item = Frame;
    type Error = LSocketError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::NotReady)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Frame {}

enum LSocketError {}
