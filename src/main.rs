use srcsrv::{SrcSrvStream, SourceRetrievalMethod};


fn main() -> Result<(), String> {
    let stream = SrcSrvStream::parse(r"SRCSRV: ini ------------------------------------------------
VERSION=2
INDEXVERSION=2
VERCTRL=http
SRCSRV: variables ------------------------------------------
HGSERVER=https://hg.mozilla.org/mozilla-central
HG_REV=1706d4d54ec68fae1280305b70a02cb24c16ff68
SRCSRVVERCTRL=http
HG_EXTRACT_TARGET=%hgserver%/raw-file/%hg_rev%/%var3%
S3_TARGET=https://gecko-generated-sources.s3.amazonaws.com/%var3%
RUST_GITHUB_TARGET=https://github.com/rust-lang/rust/raw/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/%var3%
SRCSRVTRG=%fnvar%(%var2%)
SRCSRV: source files ---------------------------------------
/builds/worker/checkouts/gecko/mozglue/baseprofiler/core/ProfilerBacktrace.cpp*HG_EXTRACT_TARGET*mozglue/baseprofiler/core/ProfilerBacktrace.cpp
/builds/worker/workspace/build/src/obj-firefox/dom/bindings/AddonManagerBinding.cpp*S3_TARGET*d89a77356015cf19cbe2488905a8e878d105c1d2b8beaf233409a976cce65a065b8319b87b7bb6e2a1dd4c5b7c0c594b9e2c171c47b6dbfe17ec0b08dc453730/dom/bindings/AddonManagerBinding.cpp
/rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/boxed.rs*RUST_GITHUB_TARGET*src/liballoc/boxed.rs
SRCSRV: end ------------------------------------------------
".as_bytes()).map_err(|e| e.to_string())?;

    for (source_path, expected_url) in &[
        // Firefox source file from hg.mo
        (r#"/builds/worker/checkouts/gecko/mozglue/baseprofiler/core/ProfilerBacktrace.cpp"#, "https://hg.mozilla.org/mozilla-central/raw-file/1706d4d54ec68fae1280305b70a02cb24c16ff68/mozglue/baseprofiler/core/ProfilerBacktrace.cpp"),
        // Rust stdlib source file from GitHub
        (r#"/rustc/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/boxed.rs"#, "https://github.com/rust-lang/rust/raw/625451e376bb2e5283fc4741caa0a3e8a2ca4d54/src/liballoc/boxed.rs"),
        // Firefox generated source file stored in S3
        (r#"/builds/worker/workspace/build/src/obj-firefox/dom/bindings/AddonManagerBinding.cpp"#, "https://gecko-generated-sources.s3.amazonaws.com/d89a77356015cf19cbe2488905a8e878d105c1d2b8beaf233409a976cce65a065b8319b87b7bb6e2a1dd4c5b7c0c594b9e2c171c47b6dbfe17ec0b08dc453730/dom/bindings/AddonManagerBinding.cpp"),
    ] {
        let url = expected_url.to_string();
        assert_eq!(Ok(Some(SourceRetrievalMethod::Download { url })),
                   stream.source_for_path(source_path, "/tmp"));
    }
    Ok(())
}
