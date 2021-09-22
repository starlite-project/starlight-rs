#[cfg(backtrace)]
pub(crate) use std::backtrace::{Backtrace, BacktraceStatus};

#[cfg(all(not(backtrace), feature = "backtrace"))]
pub(crate) use self::capture::{Backtrace, BacktraceStatus};

#[cfg(not(any(backtrace, feature = "backtrace")))]
pub(crate) enum Backtrace {}

#[cfg(backtrace)]
macro_rules! impl_backtrace {
    () => {
        std::backtrace::Backtrace
    };
}

#[cfg(all(not(backtrace), feature = "backtrace"))]
macro_rules! impl_backtrace {
    () => {
        impl core::fmt::Debug + core::fmt::Display
    };
}

#[cfg(any(backtrace, feature = "backtrace"))]
macro_rules! backtrace {
    () => {
        Some(crate::backtrace::Backtrace::capture())
    };
}

#[cfg(not(any(backtrace, feature = "backtrace")))]
macro_rules! backtrace {
    () => {
        None
    };
}

#[cfg(backtrace)]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        match $err.backtrace() {
            Some(_) => None,
            None => backtrace!(),
        }
    };
}

#[cfg(all(feature = "std", not(backtrace), feature = "backtrace"))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        backtrace!()
    };
}

#[cfg(all(feature = "std", not(backtrace), not(feature = "backtrace")))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        None
    };
}

#[cfg(all(not(backtrace), feature = "backtrace"))]
mod capture {
	use backtrace::{BacktraceFmt, BytesOrWideString, Frame, PrintFmt, SymbolName};
	use std::{
		borrow::Cow,
		cell::UnsafeCell,
		env,
		fmt::{Debug, Display, Formatter, Result as FmtResult},
		path::{Path, PathBuf},
		sync::atomic::{AtomicUsize, Ordering},
		sync::Once,
	};

	pub(crate) struct Backtrace {
		inner: Inner,
	}

	impl Backtrace {
		fn enabled() -> bool {
			static ENABLED: AtomicUsize = AtomicUsize::new(0);
			match ENABLED.load(Ordering::SeqCst) {
				0 => {}
				1 => return false,
				_ => return true,
			};

			let enabled = match env::var_os("RUST_LIB_BACKTRACE") {
				Some(s) => s != "0",
				None => match env::var_os("RUST_BACKTRACE") {
					Some(s) => s != "0",
					None => false,
				},
			};
			ENABLED.store(enabled as usize + 1, Ordering::SeqCst);
			enabled
		}

		pub(crate) fn capture() -> Self {
			if Self::enabled() {
				Self::create(Self::capture as usize)
			} else {
				let inner = Inner::Disabled;
				Self { inner }
			}
		}

		pub(crate) fn status(&self) -> BacktraceStatus {
			match self.inner {
				Inner::Unsupported => BacktraceStatus::Unsupported,
				Inner::Disabled => BacktraceStatus::Disabled,
				Inner::Captured(_) => BacktraceStatus::Captured,
			}
		}

		fn create(ip: usize) -> Self {
			let mut frames = Vec::new();
			let mut actual_start = None;

			backtrace::trace(|frame| {
				frames.push(BacktraceFrame {
					frame: frame.clone(),
					symbols: Vec::new(),
				});
				if frame.symbol_address() as usize == ip && actual_start.is_none() {
					actual_start = Some(frames.len() + 1);
				}
				true
			});

			let inner = if frames.is_empty() {
				Inner::Unsupported
			} else {
				Inner::Captured(LazilyResolvedCapture::new(Capture {
					actual_start: actual_start.unwrap_or_default(),
					resolved: false,
					frames,
				}))
			};

			Self { inner }
		}
	}

    impl Display for Backtrace {
        fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
            let capture = match &self.inner {
                Inner::Unsupported => return fmt.write_str("unsupported backtrace"),
                Inner::Disabled => return fmt.write_str("disabled backtrace"),
                Inner::Captured(c) => c.force(),
            };

            let full = fmt.alternate();
            let (frames, style) = if full {
                (&capture.frames[..], PrintFmt::Full)
            } else {
                (&capture.frames[capture.actual_start..], PrintFmt::Short)
            };

            // When printing paths we try to strip the cwd if it exists,
            // otherwise we just print the path as-is. Note that we also only do
            // this for the short format, because if it's full we presumably
            // want to print everything.
            let cwd = env::current_dir();
            let mut print_path = move |fmt: &mut Formatter, path: BytesOrWideString| {
                output_filename(fmt, path, style, cwd.as_ref().ok())
            };

            let mut f = BacktraceFmt::new(fmt, style, &mut print_path);
            f.add_context()?;
            for frame in frames {
                let mut f = f.frame();
                if frame.symbols.is_empty() {
                    f.print_raw(frame.frame.ip(), None, None, None)?;
                } else {
                    for symbol in frame.symbols.iter() {
                        f.print_raw_with_column(
                            frame.frame.ip(),
                            symbol.name.as_ref().map(|b| SymbolName::new(b)),
                            symbol.filename.as_ref().map(|b| match b {
                                BytesOrWide::Bytes(w) => BytesOrWideString::Bytes(w),
                                BytesOrWide::Wide(w) => BytesOrWideString::Wide(w),
                            }),
                            symbol.lineno,
                            symbol.colno,
                        )?;
                    }
                }
            }
            f.finish()?;
            Ok(())
        }
    }

	impl Debug for Backtrace {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			let capture = match &self.inner {
				Inner::Unsupported => return f.write_str("<unsupported>"),
				Inner::Disabled => return f.write_str("<disabled>"),
				Inner::Captured(c) => c.force(),
			};

			let frames = &capture.frames[capture.actual_start..];

			f.write_str("Backtrace ")?;

			let mut dbg = f.debug_list();

			for frame in frames {
				if frame.frame.ip().is_null() {
					continue;
				}

				dbg.entries(&frame.symbols);
			}

			dbg.finish()
		}
	}

	pub(crate) enum BacktraceStatus {
		Unsupported,
		Disabled,
		Captured,
	}

	enum Inner {
		Unsupported,
		Disabled,
		Captured(LazilyResolvedCapture),
	}

	struct Capture {
		actual_start: usize,
		resolved: bool,
		frames: Vec<BacktraceFrame>,
	}

	impl Capture {
		fn resolve(&mut self) {
			if self.resolved {
				return;
			}

			self.resolved = true;

			for frame in self.frames.iter_mut() {
				let symbols = &mut frame.symbols;
				let frame = &frame.frame;

				backtrace::resolve_frame(frame, |symbol| {
					symbols.push(BacktraceSymbol {
						name: symbol.name().map(|m| m.as_bytes().to_vec()),
						filename: symbol.filename_raw().map(|b| match b {
							BytesOrWideString::Bytes(b) => BytesOrWide::Bytes(b.to_owned()),
							BytesOrWideString::Wide(b) => BytesOrWide::Wide(b.to_owned()),
						}),
						lineno: symbol.lineno(),
						colno: symbol.colno(),
					});
				});
			}
		}
	}

	struct BacktraceFrame {
		frame: Frame,
		symbols: Vec<BacktraceSymbol>,
	}

	impl Debug for BacktraceFrame {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			let mut dbg = f.debug_list();
			dbg.entries(&self.symbols);
			dbg.finish()
		}
	}

	struct BacktraceSymbol {
		name: Option<Vec<u8>>,
		filename: Option<BytesOrWide>,
		lineno: Option<u32>,
		colno: Option<u32>,
	}

	impl Debug for BacktraceSymbol {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("{{ ")?;

			if let Some(fn_name) = self.name.as_ref().map(|b| SymbolName::new(b)) {
				f.write_str("fn: \"")?;
				Display::fmt(&fn_name, f)?;
				f.write_str("\"")?;
			} else {
				f.write_str("fn: <unknown>")?;
			}

			if let Some(fname) = self.filename.as_ref() {
				f.write_str("file: \"")?;
				Debug::fmt(fname, f)?;
				f.write_str("\"")?;
			}

			if let Some(line) = self.lineno {
				f.write_str("line: ")?;
			}

			f.write_str(" }}")
		}
	}

	enum BytesOrWide {
		Bytes(Vec<u8>),
		Wide(Vec<u16>),
	}

	impl Debug for BytesOrWide {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			output_filename(
				f,
				match self {
					Self::Bytes(c) => BytesOrWideString::Bytes(c),
					Self::Wide(w) => BytesOrWideString::Wide(w),
				},
				PrintFmt::Short,
				env::current_dir().as_ref().ok(),
			)
		}
	}

	struct LazilyResolvedCapture {
		sync: Once,
		capture: UnsafeCell<Capture>,
	}

	impl LazilyResolvedCapture {
		const fn new(capture: Capture) -> Self {
			Self {
				sync: Once::new(),
				capture: UnsafeCell::new(capture),
			}
		}

		fn force(&self) -> &Capture {
			self.sync.call_once(|| {
				unsafe { &mut *self.capture.get() }.resolve();
			});

			unsafe { &*self.capture.get() }
		}
	}

	unsafe impl Sync for LazilyResolvedCapture where Capture: Sync {}

	fn output_filename(
		fmt: &mut Formatter,
		bows: BytesOrWideString,
		print_fmt: PrintFmt,
		cwd: Option<&PathBuf>,
	) -> FmtResult {
		let file: Cow<Path> = match bows {
			#[cfg(unix)]
			BytesOrWideString::Bytes(bytes) => {
				use std::os::unix::ffi::OsStrExt;
				Path::new(std::ffi::OsStr::from_bytes(bytes)).into()
			}
			#[cfg(not(unix))]
			BytesOrWideString::Bytes(bytes) => {
				Path::new(std::str::from_utf8(bytes).unwrap_or("<unknown>")).into()
			}
			#[cfg(windows)]
			BytesOrWideString::Wide(wide) => {
				use std::os::windows::ffi::OsStringExt;
				Cow::Owned(std::ffi::OsString::from_wide(wide).into())
			}
			#[cfg(not(windows))]
			BytesOrWideString::Wide(_) => Path::new("<unknown>").into(),
		};

		if print_fmt == PrintFmt::Short && file.is_absolute() {
			if let Some(cwd) = cwd {
				if let Ok(stripped) = file.strip_prefix(&cwd) {
					if let Some(s) = stripped.to_str() {
						fmt.write_str(".")?;
						Display::fmt(&std::path::MAIN_SEPARATOR, fmt)?;
						return Display::fmt(&s, fmt);
					}
				}
			}
		}

		Display::fmt(&file.display(), fmt)
	}
}
