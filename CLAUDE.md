# Project rules

- Never commit on your own. Implementing a task (even when the user says "implement and commit") does not authorize the actual `git commit` — always stop and get explicit confirmation immediately before committing.
- Never hand-edit files under any `generated/` directory (e.g. `psql_syntax/src/generated`, `psql_factory/src/generated`, `mlang_syntax/src/generated`, `mlang_factory/src/generated`). These are produced by codegen from the `.ungram` grammar files in `codegen/` (`codegen/psql.ungram`, `codegen/m.ungram`). If generated code needs to change (including lint/warning fixes), fix the grammar or codegen tooling and regenerate — or flag the issue to the user instead of patching the generated output directly.
