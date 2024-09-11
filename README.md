# (WIP) Angler: Manage git hooks and run them fast

## Plan

- [x] Simple pre-commit hook
- [x] Order steps below
- [ ] Shell script hook
- [ ] `xargs` on staged files
- [ ] Multiple hooks (sequential)
- [ ] Rust-based hook (system version)
- [ ] From repository
- [ ] YAML config
- [ ] `install` and `run` commands
- [ ] Store
- [ ] `uninstall`
- [ ] Python-based hook (system version)
- [ ] Python-based hook (dedicated venv)
- [ ] Multithread `xargs`
- [ ] Display
- [ ] More hook types
- [ ] Multithread hooks (non-modifying)
- [ ] Lock config
- [ ] More languages

## Key features

- Fast
    - Rust binary, no intermediate VM
    - Concurrency across and within hooks
    - Pre-computed exact commands to run
- Simple
    - Human-readable expansion of each hook
    - Local / global install
    - No need for a repo-side manifest
    - Display only failing hooks
    - Sourceable multi-part config (?)
    - CLI add/remove/stats + autocompletion (?)

## Similar tools

### Husky

```
package.json    # Modified on running `husky init` to auto-install `husky`
.husky/
    .gitignore      # Initialized with `*`
    pre-commit      # Human-writable shell script, initialized with `npm test`
    _/              # Configured git.config.core.hooksPath
        h               # Runs `.husky/<script>` if it exists
        pre-commit      # Only runs `./husky/_/h`, auto-created
        pre-push        # Only runs `./husky/_/h`, auto-created
```

- Configures core hooks path to `.husky/_/`
- Writes dummy files for all possible hooks
- Redirects all git hooks to a single dispatcher `h`, that runs the appropriate human-written script
- Purely shell-based
- Can be disabled using the `HUSKY` environment variable

### Overcommit

TODO


### Pre-commit

```
# project-level
pre-commit-config.yaml
.git/hooks/
    pre-commit      # Templated, runs `<python> -m pre-commit hook-impl <config> type=pre-commit`
    pre-push        # Templated, runs `<python> -m pre-commit hook-impl <config> type=pre-push`
```

```
# global
~/.cache/pre-commit/
    db.db                       # Store, sqlite DB, tables `repos` [repo, ref, path] and `configs` 
    repo-<cloned_repo_id>/      # Cached across configs, referenced in the Store
        <repo_content>              # Cloned from source
        .pre-commit-hooks.yaml      # Manifest describing how to run the hook, part of the source
        <venv>                      # Contains hook dependencies, created during install
```

- Structure
    - Git hook-level templated block that invokes a dispatcher
    - Project-level YAML config
    - Global-level cached repo hooks + venv
    - Global-level `config -> repos -> path` store for cleanup
    - Repo-level manifest (for hook-defining repos)
- Support
    - Many languages
    - Hooks can require additional dependencies
    - Extensible
    - Hook-defining repos must expose a manifest
    - Local hooks possible
- Install
    - Global cache is populated
    - Adds templated redirects to a shared dispatcher
    - Auto-install possible with template-dir
- Run
    - Dispatcher `<python> -m pre-commit hook-impl <config> type=<hook-type>` runs
        - Python process
        - Loads config, finds hooks
        - Installs missing hooks
        - Builds hook commands
        - Runs hooks within subprocesses (Python `xargs`)
    - Concurrency
        - `xargs` runs in multithreading
        - Serial execution can be explicitly required
    - Directly invokable with `pre-commit run`
    - Targeted skips possible with `SKIP=` environment variable
