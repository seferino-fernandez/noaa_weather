# Release Process

This document outlines the release process for the `noaa_weather` project.

## Overview

The release process is automated using [release-plz](https://github.com/MarcoIeni/release-plz) and GitHub Actions. It is designed to be triggered by specific types of commits to the `main` branch.

## Triggering a Release

To trigger a release, you must push a commit to the `main` branch with a message that follows the [Conventional Commits](https://www.conventionalcommits.org/) specification. The following commit types will trigger a release:

-   `feat`: A new feature
-   `fix`: A bug fix
-   `docs`: Documentation only changes
-   `perf`: A code change that improves performance
-   `refactor`: A code change that neither fixes a bug nor adds a feature
-   `revert`: Reverts a previous commit
-   `test`: Adding missing tests or correcting existing tests
-   `update`: Used for dependency updates

Commits with other types (e.g., `chore`, `style`, `build`) will **not** trigger a release.

## The Release Workflow

1.  **Push to `main`**: When a commit with a release-worthy message is pushed to `main`, the "Release" GitHub Action is triggered.

2.  **Create Release Pull Request**: The `release-pr` job runs `release-plz` to analyze the commit history. If a new release is warranted, `release-plz` will create a new pull request with the following changes:

    -   The version numbers of the updated packages in their respective `Cargo.toml` files are bumped.
    -   `CHANGELOG.md` files are updated with the relevant commit messages.
    -   The pull request is labeled with "release".

3.  **Merge the Release Pull Request**: The release pull request should be reviewed and merged.

4.  **Publish and Release**: Merging the release pull request triggers the `release` job, which performs the following actions:
    -   Publishes the `noaa_weather_client` crate to [Crates.io](https://crates.io/).
    -   Creates a new GitHub release with the changelog for the new version.
    -   Creates and pushes a git tag for the new version (e.g., `v0.1.0`).

## CLI Binary

The `noaa_weather_cli` binary is built and attached to the GitHub release as part of the `Continuous Deployment - Build Binaries` workflow. This workflow is triggered after the GitHub release is published.
