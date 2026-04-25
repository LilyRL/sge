use futures::channel::oneshot;
use ureq::AsSendBody;

#[derive(Debug)]
pub enum Error {
    Ureq(ureq::Error),
    SenderDropped,
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self::Ureq(value)
    }
}

pub async fn get_text(url: impl Into<String> + Send + 'static) -> Result<String, Error> {
    let url = url.into();
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result = ureq::get(&url)
            .call()
            .map_err(|e| e.into())
            .and_then(|mut r| r.body_mut().read_to_string().map_err(|e| e.into()));
        let _ = tx.send(result);
    });
    rx.await.map_err(|_| Error::SenderDropped)?
}

pub async fn get_bytes(url: impl Into<String> + Send + 'static) -> Result<Vec<u8>, Error> {
    let url = url.into();
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result = ureq::get(&url)
            .call()
            .map_err(|e| e.into())
            .and_then(|mut r| r.body_mut().read_to_vec().map_err(|e| e.into()));
        let _ = tx.send(result);
    });
    rx.await.map_err(|_| Error::SenderDropped)?
}

pub async fn post(
    url: impl Into<String> + Send + 'static,
    body: impl AsSendBody + Send + 'static,
) -> Result<String, Error> {
    let url = url.into();
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result = ureq::post(&url)
            .send(body)
            .map_err(|e| e.into())
            .and_then(|mut r| r.body_mut().read_to_string().map_err(|e| e.into()));
        let _ = tx.send(result);
    });
    rx.await.map_err(|_| Error::SenderDropped)?
}
