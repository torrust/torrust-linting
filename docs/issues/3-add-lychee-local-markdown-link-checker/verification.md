# Verification Evidence

## V6 - Broken local link fails offline

A temporary directory was created with `broken.md` containing:

```markdown
[Broken local link](missing.md)
```

### Command

```sh
LINTER_BINARY="$(pwd)/target/debug/linter"
tmp_dir=$(mktemp -d)
printf '%s\n' '[Broken local link](missing.md)' > "$tmp_dir/broken.md"
cd "$tmp_dir"
set +e
"$LINTER_BINARY" lychee
exit_code=$?
set -e
printf '\nVerification exit code: %s\n' "$exit_code"
rm -rf "$tmp_dir"
test "$exit_code" -ne 0
```

### Output

```text
2026-09-08T11:06:39.299798Z  INFO lychee: Checking local Markdown links and fragments...
Issues found in 1 input. Find details below.

[broken.md]:
[ERROR] file:///tmp/<temporary-directory>/missing.md (at 1:1) | File not found. Check if file exists and path is correct

🔍 1 Total (in 0s) 🔗 1 Unique ✅ 0 OK 🚫 1 Error


  [WARN] .github/**/*.md: No files found for this input source


2026-09-08T11:06:39.310622Z ERROR lychee: Local Markdown link checking failed. Please fix the issues above. (0.011s)
Error: Local Markdown link checking failed

Verification exit code: 1
```

### Result

**Passed.** Lychee identified the missing local target, the linter forwarded the diagnostic, and
exited with status `1`. The temporary directory was removed after the check.
