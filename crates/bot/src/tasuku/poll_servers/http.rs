use {
  super::tokiort::TokioIo,
  asahi::{
    AsahiError,
    AsahiResult,
    error
  },
  bytes::{
    Buf,
    Bytes
  },
  farmsim::DssData,
  http_body_util::{
    BodyExt,
    Empty
  },
  hyper::{
    Request,
    Response,
    Uri,
    body::Incoming,
    client::conn::http1
  },
  std::borrow::Cow,
  tokio::{
    net::TcpStream,
    task,
    time::{
      Duration,
      timeout
    }
  }
};

enum DataKind {
  Dss(DssData)
}

const PIPELINE_NO_API_ACCESS: &str = "No access to {{ url }}'s API endpoint: {{ http.status }}";
const PIPELINE_UNEXPECTED_STATUS: &str = "Received an unexpected status code: {{ http.status }}";

async fn handshake(url: Uri) -> AsahiResult<http1::SendRequest<Empty<Bytes>>> {
  let host = match url.host() {
    Some(h) => h,
    None => return Err(AsahiError::Network(Cow::Borrowed("URL has no host!")))
  };
  let port = url.port_u16().unwrap_or(8080);

  let connect_timeout = Duration::from_secs(15);
  let stream = match timeout(connect_timeout, TcpStream::connect(format!("{host}:{port}"))).await {
    Ok(Ok(s)) => s,
    Ok(Err(e)) => {
      if e.kind() == std::io::ErrorKind::ConnectionReset {
        return Err(AsahiError::Network(Cow::Borrowed("Connection reset by peer")))
      }
      return Err(AsahiError::Network(format!("Connection error: {e}").into()))
    },
    Err(_) => return Err(AsahiError::Network(Cow::Borrowed("Connection timed out")))
  };
  let io = TokioIo::new(stream);

  // Do a HTTP/1.1 handshake to the given URL
  let (sender, conn) = http1::handshake(io).await?;
  task::spawn(async move {
    if let Err(e) = conn.await {
      error!("Handshake fail: {e}");
    }
  });

  Ok(sender)
}

fn request_builder(url: Uri) -> Request<Empty<Bytes>> {
  let path = url.path_and_query().map(|pq| pq.as_str()).unwrap();

  Request::builder()
    .method("GET")
    .uri(path)
    .version(hyper::Version::HTTP_11)
    .header("Connection", "keep-alive")
    .header("Accept", "application/json")
    .header("Host", url.host().unwrap())
    .body(Empty::<Bytes>::new())
    .unwrap()
}

async fn http_codes(
  res: Response<Incoming>,
  url: Uri
) -> AsahiResult<DataKind> {
  match res.status().as_u16() {
    200 => {
      let body = res.collect().await?.aggregate();
      Ok(DataKind::Dss(serde_json::from_reader(body.reader())?))
    },
    204 => Err(AsahiError::Network(Cow::Borrowed("Received data was empty (204 No Content)"))),
    401 => {
      let no_api_access = PIPELINE_NO_API_ACCESS
        .replace("{{ url }}", url.host().unwrap())
        .replace("{{ http.status }}", res.status().as_str());
      Err(AsahiError::Network(no_api_access.into()))
    },
    _ => {
      let unexpected_status = PIPELINE_UNEXPECTED_STATUS.replace("{{ http.status }}", res.status().as_str());
      Err(AsahiError::Network(unexpected_status.into()))
    }
  }
}

macro_rules! fetch_api {
  (
    $fn_name:ident,
    $expected_variant:path,
    $ok_ty:ty
  ) => {
    pub async fn $fn_name(url: String) -> AsahiResult<$ok_ty> {
      let url: Uri = url.parse().unwrap();

      let mut sender = handshake(url.clone()).await?;

      let req = request_builder(url.clone());

      let res = match sender.send_request(req).await {
        Ok(r) => r,
        Err(e) => {
          error!("({}) send request error: {e:?}", stringify!($fn_name));
          return Err(e.into());
        }
      };

      if !res.status().is_success() {
        error!("({}) request failed with status: {}", stringify!($fn_name), res.status().as_str());
      }

      match http_codes(res, url.clone()).await {
        Ok($expected_variant(d)) => Ok(d),
        Err(e) => Err(e)
      }
    }
  };
}

// Retrieves DSS data from the given URL and returns it
fetch_api!(fetch_dss, DataKind::Dss, DssData);
