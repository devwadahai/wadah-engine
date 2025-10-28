# Pre-Release Checklist for Wadah v0.1.0

## ✅ Code Quality

- [x] All crates compile without errors
- [x] All tests passing (21 unit tests)
- [x] No critical warnings
- [x] Code formatted with `cargo fmt`
- [x] Clippy checks passing
- [x] Dependencies audited

## ✅ Functionality

- [x] CLI commands work (`init`, `pack`, `verify`, `run`, `push`, `pull`, `trace`, `plugins`)
- [x] Package lifecycle (init → pack → verify) functional
- [x] Model adapters implemented (OpenAI, Ollama, TGI)
- [x] Tracing system working
- [x] Budget tracking operational
- [x] Policy enforcement in place

## ✅ Documentation

- [x] README.md complete and accurate
- [x] CHANGELOG.md updated
- [x] RELEASE-v0.1.0.md created
- [x] All docs organized (docs/INDEX.md)
- [x] API documentation (WadahSpec, ToolCaps, etc.)
- [x] Quickstart guide ready
- [x] Contributing guidelines present
- [x] Security policy documented

## ✅ Testing

- [x] Unit tests passing
- [x] Integration test scripts created
- [x] Manual testing completed (init, pack, verify)
- [x] CI/CD pipeline functional
- [x] Multi-OS testing configured

## ✅ Infrastructure

- [x] GitHub repository set up
- [x] CI/CD with GitHub Actions
- [x] Security audits automated
- [x] Documentation builds working
- [x] Branch protection (optional)

## 🔄 Distribution (To Do)

### Cargo

- [ ] Verify Cargo.toml metadata
- [ ] Test `cargo package`
- [ ] Test `cargo publish --dry-run`
- [ ] Publish to crates.io: `cargo publish -p wadah-cli`

### GitHub Release

- [ ] Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
- [ ] Push tag: `git push origin v0.1.0`
- [ ] Create GitHub release
- [ ] Attach release notes (RELEASE-v0.1.0.md)
- [ ] Build and attach binaries:
  - [ ] Linux x86_64
  - [ ] macOS x86_64
  - [ ] macOS ARM64

### Docker

- [ ] Build Docker image: `docker build -t wadah:0.1.0 .`
- [ ] Tag for GHCR: `docker tag wadah:0.1.0 ghcr.io/devwadahai/wadah:0.1.0`
- [ ] Push to GHCR: `docker push ghcr.io/devwadahai/wadah:0.1.0`
- [ ] Tag as latest: `docker tag wadah:0.1.0 ghcr.io/devwadahai/wadah:latest`
- [ ] Push latest: `docker push ghcr.io/devwadahai/wadah:latest`

## 🔄 Announcement (To Do)

- [ ] Draft announcement blog post
- [ ] Prepare social media posts
- [ ] Update project website (if exists)
- [ ] Post to:
  - [ ] Hacker News
  - [ ] Reddit (r/rust, r/artificial)
  - [ ] Twitter/X
  - [ ] LinkedIn
  - [ ] Dev.to

## ✅ Post-Release

- [ ] Monitor for issues
- [ ] Respond to feedback
- [ ] Plan v0.1.1 (bug fixes)
- [ ] Begin v0.2.0 planning

## 🎯 Release Command Sequence

```bash
# 1. Final check
cargo test --workspace
cargo build --release

# 2. Update version in Cargo.toml if needed
# (Already at 0.1.0)

# 3. Commit any final changes
git add -A
git commit -m "chore: prepare v0.1.0 release"

# 4. Tag release
git tag -a v0.1.0 -m "Release v0.1.0: First public release"
git push origin v0.1.0

# 5. Build release binaries
./scripts/build-release.sh  # (to be created)

# 6. Create GitHub release
# Use GitHub UI or gh CLI:
gh release create v0.1.0 \
  --title "Wadah v0.1.0" \
  --notes-file RELEASE-v0.1.0.md \
  target/release/wadah-*

# 7. Publish to crates.io
cargo publish -p wadah-spec
cargo publish -p wadah-trace
cargo publish -p wadah-pack
cargo publish -p wadah-runtime
cargo publish -p wadah-oci
cargo publish -p wadah-cli

# 8. Build and push Docker image
docker build -t ghcr.io/devwadahai/wadah:0.1.0 .
docker push ghcr.io/devwadahai/wadah:0.1.0
docker tag ghcr.io/devwadahai/wadah:0.1.0 ghcr.io/devwadahai/wadah:latest
docker push ghcr.io/devwadahai/wadah:latest
```

## 📝 Notes

**Critical Items:** All ✅ complete!

**Nice to Have:** Distribution and announcement steps remain

**Blockers:** None! Ready to release.

**Estimated Time:** 2-3 hours for full distribution

---

**Checklist Status:** Core Complete ✅ | Distribution Ready ⏳ | Announcement Pending 📣

