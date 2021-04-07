use super::channel::AsyncRequester;
use super::request::Request;
use crate::error::*;
use core::pin::Pin;
use futures::task::Context;
use futures::task::Poll;
use futures::{future::LocalBoxFuture, Stream};

pub struct Response<'a, R: AsyncRequester> {
    reader: R,
    requests: Vec<Request>,
    serial: &'a str,
    inflight: Option<LocalBoxFuture<'a, Result<String>>>,
}

impl<'a, R: AsyncRequester> Stream for Response<'a, R> {
    type Item = Result<String>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        // ``\_("/)_/``
        panic!()
    }
}
