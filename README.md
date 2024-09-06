# Angler: efficient git hooks management

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


### Pre-commit

```
# project-level
pre-commit-config.yaml
.git/hooks/
    pre-commit      # Templated, runs `<python> -m pre-commit hook-impl <config_file> type=pre-commit`
    pre-push        # Templated, runs `<python> -m pre-commit hook-impl <config_file> type=pre-push`
```
