WAT - source code inline tasks analysis tool
============================================

- [X] Come up with source comment task definition format
- [X] Find specially formatted comments
- [X] Process them into internal task representation
- [X] Determine the source comment format (matchers heuristic)
- [X] Go through source code
- [X] Add file path to location info
- [ ] Specify context sections
- [ ] Introduce output streams for renderables
- [ ] Implement buffered output
- [ ] CLI arguments for:
	- [ ] Output format
	- [ ] Force matchers format
	- [ ] Filter for severity
	- [ ] Filter for task type
	- [ ] Custom task type
	- [ ] Additional context
- [ ] Allow actions to run on internal task representation
- [ ] Allow for configurable task triggering words
- [ ] Multiline comments


Quick Start
-----------

```
rustc ./src/main.rs -o wat
./wat
```


Source task comment
-------------------

Source task comment is defined by a task triggering word - by default, it's `TODO` and `FIXME`.

Task triggers can optionally be prefixed with a character (`@`) and suffixed with characters:

- Situational task creation character: `:` (optional, implicit)
- Severity indication character: `!` (optional)

A most minimal task can be a simple inline comment, like so: `// @TODO: implement`

A task can also have one or more sections associated with it, delimited by a delimiter line. Like so:

```
# TODO! remove stub
#
# This is a temporary stub implementation of the model.
# The model will need to fetch its data from the API.
#
# https://identity.jira.net/browse/PROJ-1136
```

Another example:

```
/**
 * TODO: determine source code comment style
 * ---
 * We will need to know the source code comment style, so that
 * we can strip out the leading characters from task context
 */
```

A task has context if the task definition line is immediately followed by a context delimiter line:

- `<cc><empty-line>`, or
- `<cc>-{3,}`
