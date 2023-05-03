use assert_fs::TempDir;
use oranda::site::layout::Layout;

mod fixtures;
use super::utils::tokio_utils::TEST_RUNTIME;
use fixtures::{oranda_config, page};

fn temp_build_dir() -> (TempDir, String) {
    let dir = assert_fs::TempDir::new().unwrap();
    let dir_str = dir.to_str().unwrap().to_string();
    (dir, dir_str)
}

#[test]
fn it_adds_additional_css() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains("<link href=\"custom.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_renders_changelog_with_no_cargo_dist() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::changelog(&config, &layout);
    assert!(page.contents.contains("<h1>Releases</h1>"));
}

#[test]
fn it_renders_changelog_with_release_content() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::changelog(&config, &layout);
    assert!(page.contents.contains("Initial release."));
}

#[test]
fn it_adds_oranda_css() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains("<link href=\"oranda.css\" rel=\"stylesheet\"/>"));
}

#[test]
fn it_builds_the_site() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("<h1>axo</h1>"));
    assert!(page.contents.contains("custom.css"));
}

#[test]
fn reads_description() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("you axolotl questions"));
    assert!(page.contents.contains("My Axo project"))
}

#[test]
fn reads_theme() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("html class=\"dark\""));
}

#[test]
fn creates_nav() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("<nav class=\"nav\"><ul><li><a href=\"/\">Home</a></li><li><a href=\"/README.html\">Another Page</a></li></ul></nav>"));
}

#[test]
fn creates_nav_no_additional_pages() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("<nav class=\"nav\">"));
}

#[test]
fn creates_footer() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page
        .contents
        .contains("<footer class=\"footer\"><span>My Axo project</span></footer>"));
}

#[test]
fn creates_nav_item() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page
        .contents
        .contains("<li><a href=\"/artifacts.html\">Install</a></li>"));
}

#[test]
fn loads_js() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("<script src=\"/artifacts.js\">"));
}

#[test]
fn creates_download_for_mac() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("<span class=\"detect\">We have detected you are on <span class=\"detected-os\">mac</span>, are we wrong?</span>"));
}

#[test]
fn creates_downloads_page() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let artifacts_page = page::artifacts(&config, &layout);
    assert!(artifacts_page.contents.contains("<h3>Downloads</h3>"));
    assert!(artifacts_page
        .contents
        .contains("<span>Executable Zip</span><span>x86_64-pc-windows-msvc</span>"));
    assert!(artifacts_page
        .contents
        .contains("<h3>Install via script</h3>"))
}

#[test]
fn creates_nav_item_install() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::package_managers(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("View all downloads"));
}

#[test]
fn creates_copy_to_clipboard_home() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::cargo_dist(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("copy-clipboard-button"));
    assert!(page.contents.contains("installer.sh.txt\">Source</a>"));
}

#[test]
fn creates_copy_to_clipboard_artifacts() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::package_managers(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::artifacts(&config, &layout);
    assert!(page
        .contents
        .contains("<button class=\"button primary\" data-copy=\"npm install oranda\">"));
}

#[test]
fn adds_prefix() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::path_prefix(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_artifacts(&config, &layout);
    assert!(page.contents.contains("<script src=\"/axo/artifacts.js\">"));
    assert!(page
        .contents
        .contains("<a href=\"/axo/artifacts.html\">View all installation options</a>"))
}

#[test]
fn adds_changelog_nav() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::changelog(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index(&config, &layout);
    assert!(page.contents.contains("changelog.html"));
}

#[test]
fn it_renders_code_blocks_with_invalid_annotations() {
    let _guard = TEST_RUNTIME.enter();
    let (_t, temp_dir) = temp_build_dir();
    let config = oranda_config::no_artifacts(temp_dir);
    let layout = Layout::new(&config).unwrap();
    let page = page::index_with_warning(&config, &layout);
    assert!(page
        .contents
        .contains("this block will render but not be highlighted!"));
}
