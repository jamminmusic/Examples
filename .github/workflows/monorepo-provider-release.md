# Using the Monorepo Provider Release Workflow

## Setup

Copy this workflow into your `.github/workflows` directory at the root of your project.

Providers should be created within a `${feature}/${feature}-provider` directory at the root of your project. When creating a provider use the `--no-git-init` flag with `wash new provider`, or delete the `.git` directory within the provider.

A GitHub repository or environment secret must be created for each provider in the format `WASH_$FEATURENAME_PROVIDER_SECRET`. This is the `WASH_SUBJECT_KEY`

The `WASH_ISSUER_KEY` is a repository secret that is defined for a user.

## Initiation

The release workflow is initiated by pushing a tag in the format `$featurename_provider-vx.x.x`.

The dev release workflow is initiated by pushing a tag in the format `$featurename_provider-dev`.

This tag naming schema is required because of the inability to use "-"'s in github environment secrets, required by `WASH_SUBJECT_KEY`.

## Jobs

The Monorepo release Workflow consists of X Jobs:

1. build_signed_actor - build your actor and uploads wasm files to github actions
2. github_release - downloads build, generates release notes, and releases to github
3. artifact_release (to github container repository) - uploads image to GHCR

github_release and artifact_release require build_signed_actor to complete before running.

The Monorepo dev release Workflow consists of X Jobs:

1. build_signed_actor - build your actor and uploads wasm files to github actions
2. artifact_release (to github container repository) - uploads image to GHCR

artifact_release requires build_signed_actor to complete before running.
