use futures;
use futures::prelude::*;

use failure;

use hyper;
use hyper_tls;

mod cache;
use self::cache::Cache;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Fetcher {
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>,
    cache: Cache,
}

impl Fetcher {
    pub fn new() -> Result<Fetcher, failure::Error> {
        let https = hyper_tls::HttpsConnector::new(4)?;
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let fetcher = Fetcher {
            client,
            cache: Cache::new(),
        };
        Ok(fetcher)
    }

    pub fn get(
        &mut self,
        uri: String,
    ) -> Box<dyn Future<Item = hyper::Chunk, Error = failure::Error> + Send> {
        let scheme_is_file =
            uri.starts_with("file://") || uri.starts_with("/") || uri.starts_with("./");
        let scheme_is_http = uri.starts_with("http://") || uri.starts_with("https://");
        if scheme_is_file {
            let path;
            if uri.starts_with("file://") {
                // TODO: Handle file URIs that specify the host
                path = PathBuf::from(&uri["file://".len()..]);
            } else {
                path = PathBuf::from(uri);
            }
            Box::new(self.get_file(path))
        } else if scheme_is_http {
            Box::new(self.get_http(uri.parse().unwrap()))
        } else {
            Box::new(futures::future::err(format_err!("Invalid URI: {}", uri)))
        }
    }

    fn get_file(&self, path: PathBuf) -> impl Future<Item = hyper::Chunk, Error = failure::Error> {
        futures::lazy(move || {
            let mut file = File::open(path)?;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents)?;
            let chunk = hyper::Chunk::from(file_contents);
            Ok(chunk)
        })
    }

    fn get_http(
        &mut self,
        uri: hyper::Uri,
    ) -> impl Future<Item = hyper::Chunk, Error = failure::Error> {
        self.client
            .get(uri)
            .from_err()
            .and_then(|response| {
                if response.status() != hyper::StatusCode::OK {
                    bail!("HTTP status code: {}", response.status())
                } else {
                    Ok(response.into_body().concat2().from_err())
                }
            }).and_then(|chunk| chunk)
    }
}