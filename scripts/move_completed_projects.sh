#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

dry_run=0
if [[ "${1:-}" == "--dry-run" ]]; then
  dry_run=1
fi

mkdir -p projects/completed

get_status() {
  awk '
    BEGIN { in_status = 0 }
    /^##[[:space:]]+Status[[:space:]]*$/ { in_status = 1; next }
    in_status && NF {
      gsub(/\r/, "")
      print
      exit
    }
  ' "$1"
}

moved_count=0
moved_dir_count=0
declare -a moved_rs_files=()

for todo in projects/*_TODO.md; do
  [[ -f "$todo" ]] || continue

  status="$(get_status "$todo" || true)"
  status_lc="$(printf '%s' "$status" | tr '[:upper:]' '[:lower:]')"
  [[ "$status_lc" == "completed" ]] || continue

  todo_base="$(basename "$todo")"
  project_name="${todo_base%_TODO.md}"

  declare -a candidates=(
    "$todo"
    "projects/${project_name}.rs"
    "projects/${project_name}_cli.rs"
  )

  for src in "${candidates[@]}"; do
    [[ -f "$src" ]] || continue

    # If this is a project entry source file, move sibling module directories
    # declared via `mod <name>;` so local module resolution keeps working.
    if [[ "$src" == projects/*.rs ]]; then
      while IFS= read -r module_name; do
        module_src="projects/${module_name}"
        module_dest="projects/completed/${module_name}"

        [[ -d "$module_src" ]] || continue
        [[ -e "$module_dest" ]] && continue

        if [[ "$dry_run" -eq 1 ]]; then
          printf 'DRY-RUN move dir: %s -> %s\n' "$module_src" "$module_dest"
        else
          mv "$module_src" "$module_dest"
          printf 'Moved dir: %s -> %s\n' "$module_src" "$module_dest"
        fi
        moved_dir_count=$((moved_dir_count + 1))
      done < <(sed -nE 's/^[[:space:]]*mod[[:space:]]+([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*;[[:space:]]*$/\1/p' "$src")
    fi

    dest="projects/completed/$(basename "$src")"
    if [[ -e "$dest" ]]; then
      continue
    fi

    if [[ "$dry_run" -eq 1 ]]; then
      printf 'DRY-RUN move: %s -> %s\n' "$src" "$dest"
    else
      mv "$src" "$dest"
      printf 'Moved: %s -> %s\n' "$src" "$dest"
    fi

    moved_count=$((moved_count + 1))
    if [[ "$src" == *.rs ]]; then
      moved_rs_files+=("$(basename "$src")")
    fi
  done
done

updated_paths=0
if [[ -f Cargo.toml ]]; then
  for rs_file in "${moved_rs_files[@]}"; do
    old_path="path = \"projects/${rs_file}\""
    new_path="path = \"projects/completed/${rs_file}\""

    if grep -Fq "$old_path" Cargo.toml; then
      if [[ "$dry_run" -eq 1 ]]; then
        printf 'DRY-RUN Cargo.toml update: %s -> %s\n' "$old_path" "$new_path"
      else
        sed -i.bak "s|$old_path|$new_path|g" Cargo.toml
        updated_paths=$((updated_paths + 1))
      fi
    fi
  done
fi

if [[ "$dry_run" -eq 0 && -f Cargo.toml.bak ]]; then
  rm -f Cargo.toml.bak
fi

if [[ "$dry_run" -eq 1 ]]; then
  printf 'Dry run complete. Files matched for move: %d, module dirs matched: %d\n' "$moved_count" "$moved_dir_count"
else
  printf 'Done. Moved files: %d, moved module dirs: %d, Cargo path updates: %d\n' "$moved_count" "$moved_dir_count" "$updated_paths"
fi
