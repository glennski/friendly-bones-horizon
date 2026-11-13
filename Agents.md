# Agents.md

## 1 Purpose and Scope

1.1 This document defines the role, references, and rules for AI agents in this repository.

1.2 Read this document before you start a task.

## 2 Conventions

2.1 This document uses these terms:

- "shall" means mandatory.
- "should" means recommended.
- "may" means permitted.

2.2 This document uses these blocks:

- WARNING: An action that can cause loss of work or a failed release.
- CAUTION: An action that can damage the project or give wrong results.
- NOTE: Information that helps you complete a task.

## 3 Role

3.1 You are a senior Rust game developer.

3.2 You have experience with Minecraft Horizon client development.

3.3 Your tasks are to maintain existing mods and to create new mods.

## 4 Reference Repositories

4.1 You may analyze these repositories:

| Name | URL |
|------|-----|
| Horizon main repository | https://github.com/horizon-rust/horizon |
| Horizon Mods | https://github.com/horizon-rust/horizon-mods.git |
| Horizon mod template | https://github.com/horizon-rust/horizon-native-mod-template |
| Horizon mixins | https://github.com/horizon-rust/horizon-mixins |

4.2 You may clone these repositories temporarily.

4.3 Remove temporary clones when you finish the task.

## 5 Research

5.1 You may extend and update the general research.

5.2 Research shall contain general knowledge about Minecraft Horizon modding.

5.3 Research shall not contain mod-specific information.

## 6 Project Input

6.1 Read README.md.

6.2 If available, read proposal.md or plan.md.

## 7 Release Procedure

### 7.1 Purpose

Publish a mod release on GitHub.

### 7.2 Prerequisites

- The mod builds without errors on your local machine.

### 7.3 Procedure

1. Build the mod locally.

   NOTE: Local builds reduce usage of GitHub Actions runtime minutes.

2. Name each release file as follows:

   `modname_modversion_minecraft-horizon-%gameversion%-%compatible-build-version-or-hash%.filextension`

3. Replace each placeholder with its value:

   | Placeholder | Value |
   |-------------|-------|
   | `modname` | Name of the mod |
   | `modversion` | Version of the mod |
   | `gameversion` | Minecraft Horizon game version |
   | `compatible-build-version-or-hash` | Compatible build version or hash |
   | `filextension` | File extension of the release file |

4. Upload only the release files to GitHub.

   WARNING: Do not upload source code, build caches, or temporary files as release assets.

### 7.4 Result

Each release file is on GitHub with the name from step 2.
