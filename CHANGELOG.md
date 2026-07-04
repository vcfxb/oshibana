## [0.1.14-pre] - 2026-07-04

### 🐛 Bug Fixes

- *(CI)* Don't recompile the binaries while building the bundles
- *(CI)* Cancel in progress releases if a new one is issued
## [0.1.13-pre] - 2026-07-04

### 🚀 Features

- *(CI)* Add automatic testing for a few more platforms, and add build caching to the test workflow.
- Add build versioning into desktop app (ui pending).
- *(CI)* Add automatic creation of installation packages to release workflow
- Add "about" to help menu

### 💼 Other

- *(README)* You -> your

### ⚙️ Miscellaneous Tasks

- *(icons)* Generate icons using `npx @tauri-apps/cli icon`
- *(CI)* Ensure bundler job doesn't overwrite changelog
- Cargo fmt
- Clippy + fmt
## [0.1.12] - 2026-07-03

### 🐛 Bug Fixes

- *(ci)* Add `permissions: write-all` to publish-binaries job

### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.12
## [0.1.11] - 2026-07-03

### 🐛 Bug Fixes

- *(release)* Rename binary to oshibana, add prod feature to build args

### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.11
## [0.1.10] - 2026-07-03

### 🐛 Bug Fixes

- *(ci)* Publish binaries step was missing correct runs-on info

### ⚙️ Miscellaneous Tasks

- Bump version to 0.1.10
## [0.1.9] - 2026-07-03

### ⚙️ Miscellaneous Tasks

- Tell release workflow to wait for updated changelog
- Bump version to 0.1.9
## [0.1.8] - 2026-07-03

### ⚙️ Miscellaneous Tasks

- Tweak git cliff to not add any commits pre 0.1.3 to the changelog
- Bump version to 0.1.8
## [0.1.6] - 2026-07-03

### ⚙️ Miscellaneous Tasks

- Convert release protection rules to github rulesets
## [0.1.4] - 2026-07-03
