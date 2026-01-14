# Release Process

This document describes the process for creating releases of the BMSSP package.

## Version Numbering

We follow [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

For pre-release versions, use format: `v0.1.0` (first release), `v0.2.0` (next minor), etc.

## Release Checklist

Before creating a release, ensure:

- [ ] All tests pass (locally and in CI)
- [ ] Documentation is updated and accurate
- [ ] CHANGELOG.md is updated with new version entry
- [ ] Version numbers are updated in:
  - [ ] `python/pyproject.toml`
  - [ ] `python/bmssp/__init__.py` (__version__)
  - [ ] `rust/Cargo.toml` (workspace version)
- [ ] No sensitive information in repository
- [ ] Examples work correctly
- [ ] Performance benchmarks show no regressions (if applicable)

## Steps for Creating a Release

### 1. Update Version Numbers

Update version in all locations:

**python/pyproject.toml:**
```toml
version = "0.1.0"
```

**python/bmssp/__init__.py:**
```python
__version__ = "0.1.0"
```

**rust/Cargo.toml:**
```toml
[workspace.package]
version = "0.1.0"
```

### 2. Update CHANGELOG.md

Add a new section for the release version with all changes since the last release:

```markdown
## [0.1.0] - 2025-01-XX

### Added
- New features...

### Changed
- Changes...

### Fixed
- Bug fixes...
```

### 3. Commit Changes

```bash
git add python/pyproject.toml python/bmssp/__init__.py rust/Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.1.0"
```

### 4. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
```

### 5. Push Tag

```bash
git push origin main
git push origin v0.1.0
```

Pushing the tag triggers the `build-wheels.yml` workflow, which will:
- Build wheels for all target platforms
- Build source distribution
- Upload artifacts

### 6. Wait for Workflows

Monitor the GitHub Actions workflows:
- `Build wheels` workflow should complete successfully
- All wheels and source distribution should be built

### 7. Create GitHub Release

1. Go to GitHub repository: https://github.com/tripp-smith/bmssp-py
2. Click "Releases" → "Draft a new release"
3. Select the tag you just created (e.g., `v0.1.0`)
4. Title: `v0.1.0` (or add a descriptive title)
5. Description: Copy the relevant section from CHANGELOG.md
6. The `release.yml` workflow will automatically attach wheels and source distribution
7. Click "Publish release"

### 8. Verify Release

- Check that wheels are attached to the release
- Verify source distribution is included
- Test installation from a wheel (optional)

## Testing Procedures Before Release

### Local Testing

1. **Run all tests:**
```bash
# Rust tests
cd rust/bmssp-core && cargo test --all-features && cd ../..

# Python tests
cd python && pytest tests/ -v && cd ..
```

2. **Build and test wheel locally:**
```bash
cd python
maturin build --release
pip install dist/bmssp-*.whl
python -c "import bmssp; print(bmssp.__version__)"
pytest tests/ -v
```

3. **Test source distribution:**
```bash
cd python
python -m build --sdist
pip install dist/bmssp-*.tar.gz
python -c "import bmssp; print(bmssp.__version__)"
```

4. **Run examples:**
```bash
cd python
python examples/grid_pipeline.py
```

### CI Testing

Ensure all CI workflows pass:
- Tests on all platforms (Ubuntu, macOS, Windows)
- Tests on all Python versions (3.9, 3.10, 3.11, 3.12)

## PyPI Publishing (Future)

**Note**: PyPI publishing is deferred for the first release. When ready to publish:

### Prerequisites

1. Create PyPI account: https://pypi.org/account/register/
2. Create API token: https://pypi.org/manage/account/token/
3. Install twine: `pip install twine`

### Process

1. **Test on Test PyPI first:**
```bash
cd python
python -m build
twine upload --repository-url https://test.pypi.org/legacy/ dist/*
```

2. **Verify installation from Test PyPI:**
```bash
pip install --index-url https://test.pypi.org/simple/ bmssp
```

3. **Upload to PyPI:**
```bash
twine upload dist/*
```

If the upload fails with "File already exists", the filename/version has already been published.
In that case, either bump the package version and rebuild, or re-run the command with
`--skip-existing` to upload only the missing files.

4. **Verify installation from PyPI:**
```bash
pip install bmssp
```

### Automation

For future releases, consider automating PyPI upload in the release workflow (requires PyPI API token in GitHub secrets).

## GitHub Release Creation

The `release.yml` workflow automatically:
- Downloads wheel artifacts from `build-wheels.yml`
- Downloads source distribution
- Attaches all files to the GitHub release

Manual steps:
1. Create release on GitHub (triggers workflow)
2. Write release notes (copy from CHANGELOG.md)
3. Publish release

## Communication

After release:
- Update any project status pages
- Announce on relevant channels (if applicable)
- Update documentation if needed

## Rollback

If a critical issue is found after release:

1. Create a patch release (e.g., `v0.1.1`) with fixes
2. Document the issue in CHANGELOG.md
3. Follow the same release process

## Notes

- First release: v0.1.0 (beta)
- Future releases: v0.2.0, v0.3.0, etc.
- Major version (v1.0.0) when API is considered stable
- PyPI publishing can be added later when ready
