use crate::{Requester, Parser};
use anyhow::Result;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;

pub struct Scheduler<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    request_tx: Sender<Arc<Requester>>,
    request_rx: Option<Receiver<Arc<Requester>>>,
    parser_tx: Sender<Arc<P::Output>>,
    parser_rx: Option<Receiver<Arc<P::Output>>>,
    capacity: usize,
}

impl<P> Scheduler<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    pub fn new(capacity: usize) -> Self {
        let (request_tx, request_rx) = channel(capacity);
        let (parser_tx, parser_rx) = channel(capacity);
        Self {
            request_tx,
            request_rx: Some(request_rx),
            parser_tx,
            parser_rx: Some(parser_rx),
            capacity,
        }
    }

    pub fn split(&mut self) -> (SchedulerSender<P>, SchedulerReceiver<P>) {
        let request_rx = self.request_rx.take().expect("Receiver already taken");
        let parser_rx = self.parser_rx.take().expect("Receiver already taken");
        
        (
            SchedulerSender {
                request_tx: self.request_tx.clone(),
                parser_tx: self.parser_tx.clone(),
            },
            SchedulerReceiver {
                request_rx,
                parser_rx,
            },
        )
    }
}

pub struct SchedulerSender<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    request_tx: Sender<Arc<Requester>>,
    parser_tx: Sender<Arc<P::Output>>,
}

impl<P> SchedulerSender<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    pub async fn add_requests(&self, urls: Vec<String>) -> Result<()> {
        for url in urls {
            let requester = Arc::new(Requester::new(&url));
            self.request_tx.send(requester).await?;
        }
        Ok(())
    }

    pub async fn add_parsed_data(&self, data: P::Output) -> Result<()> {
        self.parser_tx.send(Arc::new(data)).await?;
        Ok(())
    }
}

pub struct SchedulerReceiver<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    request_rx: Receiver<Arc<Requester>>,
    parser_rx: Receiver<Arc<P::Output>>,
}

impl<P> SchedulerReceiver<P>
where
    P: Parser,
    P::Output: Send + Sync + Clone + 'static,
{
    pub async fn get_request(&mut self) -> Option<Arc<Requester>> {
        self.request_rx.recv().await
    }

    pub async fn get_parsed_data(&mut self) -> Option<Arc<P::Output>> {
        self.parser_rx.recv().await
    }
}