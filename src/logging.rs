use crate::prelude::*;
use std::{fs::File, sync::Arc};

use tracing_subscriber::{fmt::layer, layer::SubscriberExt, Registry};
pub(crate) fn init_tracing(file: File) -> Result<()> {
    let stdout_log = layer().compact();
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(file));
    let subscriber = Registry::default().with(stdout_log).with(json_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
