# Developer Guidelines

This file provides reference for developers throughout the workflow, including 

---

## PR Conventions

Vitium uses PRs to manage contributions from either the developer team or community contributors. The 

### Documentation

All code come with docs. Documentation are written directly in the source code files, and shall cover all exported symbols in a file.

### Licensing

All contributions are regarded as licensed under GNU General Public License version 3 (GPLv3) or later. Please clarify explicitly and contact the developer team if you would like some other option.

### Code Review

A code review from at least one member of the developer team is required before a PR could be merged.

Code style guidelines are currently under construction.

---

Every commit to the project is categorized into the following.

## Features

Vitium employs a feature-based workflow for adding new functionalities to the project. A *feature* is a certain, unitary standalone functionality that can be adopted by the project independent of other pending features.

### Creation

A feature could be initiated by anyone via Github or our QQ channel. The proposal would be reviewed by the developer team to determine the significance of the feature, falling into either of the following categories:

- To do in the upcoming version;

- Planned to do in some future version;

- Not in consideration yet.

### Development

If a feature is planned to be done in the current version, a branch named `feat/x` (`x` being some descriptive name of the feature, e.g. `support-image-in-messaging`)is created, and developers commit to this branch during the development stage of this feature.

During the process of development, the `feat/x` branch merges commits from no branches expect `main` .

### Publish

After development of a feature is done, it is published by merging to the `main` branch as a regular PR. See [PR Conventions](#pr-conventions) for more information. 

## Fix

A *fix* is some changes to the existent code that do not modify the function of the code, e.g. fixing bugs, optimizing performance or refactoring the code.

Unlike features, fixes could be initiated any time through a PR to the `main` branch, and follow the [PR Conventions](#pr-conventions) as a regular PR.

## Meta

Updates associated to the development of this project instead of the project itself, e.g. updating this guideline, are considered *meta* updates.

Meta updates are suggested anywhere, but can only be adopted after discussion of the developer team. To issue a meta update, the project administrator would directly modify the `main` branch.

---

## Versioning

Vitium is versioned with a [semantic version number](https://semver.org/). The developer team would determine a planned release time for a specific minor version, with the planned features implemented.

Vitium is NOT scheduled released. Despite the developer team has a planned release time, the interval between versions varies in consideration of multiple factors.
