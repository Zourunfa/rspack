use std::{collections::HashMap, path::Path};

use rspack::{dev_server, rspack};
use rspack_core::{log, CompilerOptions};
use rspack_node::{normalize_bundle_options, RawOptions};

#[tokio::main]
async fn main() {
  // let guard = log::enable_tracing_by_env();
  log::enable_tracing_by_env();
  let mut dev_server = dev_server(
    normalize_bundle_options(RawOptions {
      entries: HashMap::from([("main".to_string(), "./src/index.js".to_string())]),
      root: Some(
        Path::new("./examples/react")
          // .resolve()
          .to_string_lossy()
          .to_string(),
      ),
      ..Default::default()
    })
    .unwrap(),
    vec![],
  );

  dev_server.serve().await;

  // if let Some(g) = guard {
  //   g.flush()
  // }
}