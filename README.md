# MultiSnail

## About

MultiSnail is an instance and mod manager for Will You Snail.
It was originally intended to be a simple installer for the gmml mod loader, however it quickly grew to be intended to be a all-in-one installer and launcher for Will You Snail instances with their own mods, mod loaders, and save folders.

[If you want to keep up with the current plans, you can check out the trello!](https://trello.com/b/4qJhUSY5/multi-snail)

## Installation

Once MultiSnail is ready for a beta release, github actions will automatically publish releases every time there is a commit to the release branch. 

## Development

If you wish to help work on the development, or compile it on your own, this is the place to look.

### Code Structure

The source code for the front end is located in `src/`. The front end is written in astro.

The `src/components/` directory contains standard themed components such as buttons and input boxes.

The `src/layouts/` directory contains reuseable layouts of sets of components, usually for a specific task, such as a standard header or the layout for an instances data.

The `src/pages/` directory is the home of each page, usually using different layouts and components for some specific task, such as the main screen, the options menu, or the instance creation page.

The source code for the backend is located in `src-tauri/`. The back end is written in rust using the tauri framework.

The `src-tauri/src/` directory contains the rust source code.

The `src-tauri/icons/` directory contains the icons used for the app icon. These will be changed as soon as a custom logo for MultiSnail exists.

### Compilation

Compiling MultiSnail requires you to have cargo and npm installed.

#### Setup

First, clone this repository. If you are doing this in order to contribute to MultiSnail, please create a fork from the main branch and clone it.

Once you have cloned it, cd into the project's folder. In the root folder, run `npm init`.

After that finishes, cd into the `src-tauri/` folder and run `cargo fetch`.

#### Compilation

Once you have the source code and dependencies, run `cargo tauri build`. If you are contributing to MultiSnail, you may want to run `cargo tauri dev` instead to run it quickly and enable hot reload.

If you are building on Windows, build will create an msi installer (somewhere) in `src-tauri/target/release/`. Other platforms will also place the bundle in this directory.

#### Publishing Releases

If you are publishing a release, first create an empty commit (using `git commit --allow--empty`). The commit message should start with either: `MAJOR`, `MINOR`, `FIX`, or `BETA`. This represents the type of release, and is used to increment the version number. On the next line, a release message should be included, summing up the release in a short message. This is used to generate the changelog. After pushing this commit, open a pull request stating your reasoning for the release and reasoning for the release type. Once the pull request is merged, the release branch will be synced with the main branch, and a build will be created, with the version number incremented properly according to the release type. Beta releases should be used for testing fixes and features in normal usage, and once they are considered fully functional, a `FIX` or `MINOR` release should be created, depending on if it was a feature update or a bug fix. The changelog will be generated from the commit messages of the more minor releases before it.

2 example release commits are shown below.

```
BETA
Fixes issue where some instances would not appear in the list by checking json and folder structure.
```
If the above release showed no major related problems, you would then create a `FIX` commit.
```
FIX
Fixes issue where some instances would not appear in the list.
```

The changelog only includes messages from `FIX` and `MINOR` releases, unless the release is a `FIX` or `BETA` release, in which case the `BETA` release messages are shown.