use std::borrow::Cow;
use std::fs::{self, File, Permissions};
use std::io::{self, Write as _};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail};
use tempfile::{Builder, NamedTempFile};

use super::sink::{DestinationAdapter, MediaKind, SinkTransaction};

pub(super) struct AtomicFileDestination {
    requested_path: PathBuf,
}

impl AtomicFileDestination {
    pub(super) const fn new(requested_path: PathBuf) -> Self {
        Self { requested_path }
    }

    fn prepare(&self) -> Result<PreparedTarget> {
        if self.requested_path.as_os_str().is_empty() {
            bail!("output path cannot be empty");
        }

        let target_path = resolve_symlink_target(&self.requested_path)?;
        let parent = usable_parent(&target_path).to_path_buf();
        let parent_metadata = fs::metadata(&parent)
            .with_context(|| format!("reading output parent directory {}", parent.display()))?;
        if !parent_metadata.is_dir() {
            bail!("output parent {} is not a directory", parent.display());
        }

        let permissions = match fs::metadata(&target_path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    bail!(
                        "output target {} is not a regular file",
                        target_path.display()
                    );
                }
                File::options()
                    .write(true)
                    .open(&target_path)
                    .with_context(|| {
                        format!("checking write access to output {}", target_path.display())
                    })?;
                Some(metadata.permissions())
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("reading output target {}", target_path.display()));
            }
        };

        Ok(PreparedTarget {
            target_path,
            parent,
            permissions,
        })
    }
}

impl DestinationAdapter for AtomicFileDestination {
    fn validate(&self, _media: MediaKind) -> Result<()> {
        self.prepare().map(|_| ())
    }

    fn label(&self) -> Cow<'_, str> {
        self.requested_path.to_string_lossy()
    }

    fn is_terminal(&self) -> bool {
        false
    }

    fn begin(&self) -> Result<Box<dyn SinkTransaction>> {
        let prepared = self.prepare()?;
        let mut builder = Builder::new();
        builder.prefix(".noaa-weather-");
        #[cfg(unix)]
        builder.permissions(Permissions::from_mode(0o666));
        let temporary = builder.tempfile_in(&prepared.parent).with_context(|| {
            format!(
                "creating temporary output beside {}",
                prepared.target_path.display()
            )
        })?;

        Ok(Box::new(AtomicFileTransaction {
            temporary,
            target_path: prepared.target_path,
            permissions: prepared.permissions,
        }))
    }
}

struct PreparedTarget {
    target_path: PathBuf,
    parent: PathBuf,
    permissions: Option<Permissions>,
}

struct AtomicFileTransaction {
    temporary: NamedTempFile,
    target_path: PathBuf,
    permissions: Option<Permissions>,
}

impl io::Write for AtomicFileTransaction {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.temporary.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.temporary.flush()
    }
}

impl SinkTransaction for AtomicFileTransaction {
    fn commit(mut self: Box<Self>) -> Result<()> {
        self.temporary.flush().with_context(|| {
            format!(
                "flushing temporary output for {}",
                self.target_path.display()
            )
        })?;
        if let Some(permissions) = self.permissions.take() {
            self.temporary
                .as_file()
                .set_permissions(permissions)
                .with_context(|| {
                    format!(
                        "preserving permissions for output {}",
                        self.target_path.display()
                    )
                })?;
        }

        let AtomicFileTransaction {
            temporary,
            target_path,
            ..
        } = *self;
        let (file, temporary_path) = temporary.into_parts();
        drop(file);
        temporary_path
            .persist(&target_path)
            .map_err(|error| error.error)
            .with_context(|| format!("atomically replacing output {}", target_path.display()))
    }
}

fn usable_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn resolve_symlink_target(requested_path: &Path) -> Result<PathBuf> {
    let mut target = requested_path.to_path_buf();
    for _ in 0..40 {
        match fs::symlink_metadata(&target) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let link = fs::read_link(&target)
                    .with_context(|| format!("reading output symlink {}", target.display()))?;
                target = if link.is_absolute() {
                    link
                } else {
                    usable_parent(&target).join(link)
                };
            }
            Ok(_) => return Ok(target),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(target),
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("inspecting output path {}", target.display()));
            }
        }
    }

    bail!(
        "output path {} contains too many symlink levels",
        requested_path.display()
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[cfg(unix)]
    #[test]
    fn resolves_relative_symlink_without_replacing_it() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("target.txt");
        let link = directory.path().join("link.txt");
        fs::write(&target, "before").unwrap();
        symlink("target.txt", &link).unwrap();

        let destination = AtomicFileDestination::new(link.clone());
        let mut transaction = destination.begin().unwrap();
        transaction.write_all(b"after").unwrap();
        transaction.commit().unwrap();

        assert!(
            fs::symlink_metadata(&link)
                .unwrap()
                .file_type()
                .is_symlink()
        );
        assert_eq!(fs::read_to_string(&target).unwrap(), "after");
    }

    #[cfg(unix)]
    #[test]
    fn preserves_existing_permissions() {
        let directory = tempfile::tempdir().unwrap();
        let target = directory.path().join("output.txt");
        fs::write(&target, "before").unwrap();
        fs::set_permissions(&target, Permissions::from_mode(0o640)).unwrap();

        let destination = AtomicFileDestination::new(target.clone());
        let mut transaction = destination.begin().unwrap();
        transaction.write_all(b"after").unwrap();
        transaction.commit().unwrap();

        let mode = fs::metadata(&target).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o640);
    }
}
