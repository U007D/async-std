use std::fs;
use std::path::Path;

use crate::io;
use crate::task::blocking;

/// Removes an existing, empty directory.
///
/// This function is an async version of [`std::fs::remove_dir`].
///
/// [`std::fs::remove_dir`]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
///
/// # Errors
///
/// An error will be returned in the following situations (not an exhaustive list):
///
/// * `path` is not an empty directory.
/// * The current process lacks permissions to remove directory at `path`.
///
/// # Examples
///
/// ```no_run
/// # #![feature(async_await)]
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::fs;
///
/// fs::remove_dir("./some/dir").await?;
/// #
/// # Ok(()) }) }
/// ```
pub async fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    blocking::spawn(async move { fs::remove_dir(path) }).await
}
