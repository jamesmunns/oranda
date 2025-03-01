use crate::config::Config;
use crate::errors::*;
use crate::site::link;

use axoasset::Asset;

/// Fetches the logo and adds it to the dist_dir, then returns the path to link it with
pub fn get_logo(logo: &str, config: &Config) -> Result<String> {
    let fetched_logo = fetch_logo(&config.build.path_prefix, &config.build.dist_dir, logo);

    tokio::runtime::Handle::current().block_on(fetched_logo)
}

/// Inner impl of [`get_logo`][]
async fn fetch_logo(
    path_prefix: &Option<String>,
    dist_dir: &str,
    origin_path: &str,
) -> Result<String> {
    let copy_result = Asset::copy(origin_path, dist_dir).await?;

    let path_as_string = copy_result.strip_prefix(dist_dir)?.to_string_lossy();
    let src = link::generate(path_prefix, &path_as_string);

    Ok(src)
}
