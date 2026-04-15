# Decision: T14 BGE-small Inference + T34 musl Static Binary

**By:** Fry
**Date:** 2026-04-15
**Status:** IMPLEMENTED

## T14 — BGE-small-en-v1.5 Forward Pass

### Decision
Full Candle BERT forward pass implemented in `src/core/inference.rs`. The SHA-256 hash shim is retained as a runtime fallback when model files are unavailable.

### Architecture
- `EmbeddingModel` wraps `EmbeddingBackend` enum: `Candle { model, tokenizer, device }` or `HashShim`
- Model loading attempted at first `embed()` call via `OnceLock`; falls back to `HashShim` with stderr warning
- `--features online-model` enables `hf-hub` for HuggingFace Hub download; without it, checks `~/.gbrain/models/bge-small-en-v1.5/` and HF cache
- Forward pass: tokenize → BertModel::forward → mean pooling (broadcast_as) → L2 normalize → 384-dim Vec<f32>

### Known Issues
- **hf-hub 0.3.2 redirect bug:** HuggingFace now returns relative URLs in HTTP 307 Location headers. hf-hub 0.3.2's ureq-based client fails to resolve these. Workaround: manually download model files via `curl -sL`. Phase 2 should bump hf-hub or implement direct HTTP download.
- **Candle broadcast semantics:** Unlike PyTorch, Candle requires explicit `broadcast_as()` for shape-mismatched tensor ops. All three broadcast sites (mask×output, sum÷count, mean÷norm) are explicitly handled.

### Feature Flag Changes
- `embed-model` removed from `[features] default` (was never wired)
- `online-model = ["hf-hub"]` is the active download path (optional dependency)
- Default build has no download capability; requires pre-cached model files

### Phase 2 Recommendations
- Bump `hf-hub` when a fix for relative redirects lands, or implement a simple `ureq` direct download
- Implement `embed-model` feature with `include_bytes!()` for zero-network binary (~90MB)
- Add a `gbrain model download` command for explicit model fetch

---

## T34 — musl Static Binary

### Decision
`x86_64-unknown-linux-musl` static binary build succeeds. Binary is fully statically linked, 8.8MB stripped.

### Build Requirements
```bash
sudo apt-get install -y musl-tools
rustup target add x86_64-unknown-linux-musl

CC_x86_64_unknown_linux_musl=musl-gcc \
CXX_x86_64_unknown_linux_musl=g++ \
CFLAGS_x86_64_unknown_linux_musl="-Du_int8_t=uint8_t -Du_int16_t=uint16_t -Du_int64_t=uint64_t" \
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
cargo build --release --target x86_64-unknown-linux-musl
```

### Known Issues
- **sqlite-vec musl compat:** sqlite-vec 0.1.x uses glibc-specific `u_int8_t`/`u_int16_t`/`u_int64_t` type aliases not available in musl. Workaround: pass `-D` defines via CFLAGS.
- **C++ compiler:** gemm (candle dependency) requires a C++ compiler. `musl-g++` doesn't exist; using host `g++` with musl-gcc linker works.

### Verification
- `ldd`: "statically linked"
- `file`: "ELF 64-bit LSB pie executable, x86-64, static-pie linked, stripped"
- Size: 8.8MB (without embedded model weights)
