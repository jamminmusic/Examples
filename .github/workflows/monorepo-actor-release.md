# Using the Monorepo Actor Release Workflow

## Setup

Copy this workflow into your `.github/workflows` directory at the root of your project.

Actors should be created within a `${feature}/${feature}-actors/actor` directory at the root of your project. When creating an actor use the `--no-git-init` flag with `wash new actor`, or delete the `.git` directory within the actor.

A GitHub repository or environment secret must be created for each actor in the format `WASH_$FEATURENAME_$ACTORNAME_SECRET`. This is the `WASH_SUBJECT_KEY`

The `WASH_ISSUER_KEY` is a repository secret that is defined for a user.

## Initiation

The release workflow is initiated by pushing a tag in the format `&featurename_$actorname-actor-vx.x.x`.

The dev release workflow is initiated by pushing a tag in the format `&featurename_$actorname-actor-dev`.

This tag naming schema is required because of the inability to use "-"'s in github environment secrets, required by `WASH_SUBJECT_KEY`.

## Jobs

The Monorepo release Workflow consists of 3 Jobs:

1. build_signed_actor - build your actor and uploads wasm files to github actions
2. github_release - downloads build, generates release notes, and releases to github
3. artifact_release (to github container repository) - uploads image to GHCR

github_release and artifact_release require build_signed_actor to complete before running.

The Monorepo dev release Workflow consists of 2 Jobs:

1. build_signed_actor - build your actor and uploads wasm files to github actions
2. artifact_release (to github container repository) - uploads image to GHCR

artifact_release requires build_signed_actor to complete before running.
