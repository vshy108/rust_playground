#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

dry_run=0
mode="completed"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    --mode)
      mode="${2:-}"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      echo "Usage: $0 [--dry-run] [--mode completed|wip|planned]" >&2
      exit 1
      ;;
  esac
done

case "$mode" in
  completed)
    static_target_dir="projects/completed"
    ;;
  wip)
    static_target_dir="projects/wip"
    ;;
  planned)
    static_target_dir=""
    ;;
  *)
    echo "Unsupported mode: $mode (expected: completed|wip|planned)" >&2
    exit 1
    ;;
esac

if [[ -n "$static_target_dir" ]]; then
  mkdir -p "$static_target_dir"
elif [[ "$mode" == "planned" ]]; then
  for i in {1..10}; do
    mkdir -p "projects/planned/rating_${i}"
  done
fi

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

get_rating() {
  sed -nE '1s/.*\([^0-9]*([0-9]+)\/10\).*/\1/p' "$1"
}

is_wip_status() {
  local status_lc="$1"
  [[ "$status_lc" == "in progress" || "$status_lc" == "in-progress" || "$status_lc" == "wip" || "$status_lc" == "working" || "$status_lc" == "started" ]]
}

matches_mode() {
  local status_lc="$1"

  case "$mode" in
    completed)
      [[ "$status_lc" == "completed" ]]
      ;;
    wip)
      is_wip_status "$status_lc"
      ;;
    planned)
      [[ "$status_lc" != "completed" ]] && ! is_wip_status "$status_lc"
      ;;
  esac
}

# Returns 0 when the Rust file is only a scaffold/stub main (non-WIP), else 1.
is_stub_source() {
  local src="$1"
  local normalized line_count

  # Remove single-line comments and blank lines, trim whitespace.
  normalized="$(sed -E 's,//.*$,,' "$src" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//' | sed '/^$/d')"

  # Empty file or no remaining code after stripping comments is a stub.
  [[ -z "$normalized" ]] && return 0

  # One-line trivial forms.
  if printf '%s\n' "$normalized" | grep -Eq '^fn main\(\)[[:space:]]*\{[[:space:]]*\}$'; then
    return 0
  fi
  if printf '%s\n' "$normalized" | grep -Eq '^fn main\(\)[[:space:]]*\{[[:space:]]*(e?println!)!\(.+\);[[:space:]]*\}$'; then
    return 0
  fi

  # Common 3-line trivial form:
  # fn main() {
  #   println!(...);
  # }
  line_count="$(printf '%s\n' "$normalized" | wc -l | tr -d ' ')"
  if [[ "$line_count" -eq 3 ]]; then
    local l1 l2 l3
    l1="$(printf '%s\n' "$normalized" | sed -n '1p')"
    l2="$(printf '%s\n' "$normalized" | sed -n '2p')"
    l3="$(printf '%s\n' "$normalized" | sed -n '3p')"
    if printf '%s\n' "$l1" | grep -Eq '^fn[[:space:]]+main\(\)[[:space:]]*\{$' \
      && printf '%s\n' "$l2" | grep -Eq '^(e?println!)\(.*\);$' \
      && [[ "$l3" == "}" ]]; then
      return 0
    fi
  fi

  return 1
}

moved_count=0
moved_dir_count=0
declare -a cargo_old_paths=()
declare -a cargo_new_paths=()

for todo in projects/*_TODO.md; do
  [[ -f "$todo" ]] || continue

  status="$(get_status "$todo" || true)"
  status_lc="$(printf '%s' "$status" | tr '[:upper:]' '[:lower:]')"
  matches_mode "$status_lc" || continue

  project_target_dir="$static_target_dir"
  if [[ "$mode" == "planned" ]]; then
    rating="$(get_rating "$todo" || true)"
    if ! [[ "$rating" =~ ^([1-9]|10)$ ]]; then
      if [[ "$dry_run" -eq 1 ]]; then
        printf 'DRY-RUN skip unrated TODO: %s\n' "$todo"
      else
        printf 'Skip unrated TODO: %s\n' "$todo"
      fi
      continue
    fi
    project_target_dir="projects/planned/rating_${rating}"
  fi

  mkdir -p "$project_target_dir"

  todo_base="$(basename "$todo")"
  project_name="${todo_base%_TODO.md}"

  # In WIP mode, move only when the project has meaningful Rust code.
  # A comment-only file with just `fn main(){ println!(...) }` is treated as non-WIP.
  if [[ "$mode" == "wip" ]]; then
    has_non_stub=0
    for src_candidate in "projects/${project_name}.rs" "projects/${project_name}_cli.rs"; do
      [[ -f "$src_candidate" ]] || continue
      if ! is_stub_source "$src_candidate"; then
        has_non_stub=1
        break
      fi
    done

    if [[ "$has_non_stub" -eq 0 ]]; then
      if [[ "$dry_run" -eq 1 ]]; then
        printf 'DRY-RUN skip non-WIP stub: %s\n' "$project_name"
      fi
      continue
    fi
  fi

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
        module_dest="${project_target_dir}/${module_name}"

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

    dest="${project_target_dir}/$(basename "$src")"
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
    if [[ "$src" == projects/*.rs ]]; then
      cargo_old_paths+=("path = \"${src}\"")
      cargo_new_paths+=("path = \"${dest}\"")
    fi
  done
done

updated_paths=0
if [[ -f Cargo.toml ]]; then
  for i in "${!cargo_old_paths[@]}"; do
    old_path="${cargo_old_paths[$i]}"
    new_path="${cargo_new_paths[$i]}"

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
  if [[ "$mode" == "planned" ]]; then
    printf 'Dry run complete (planned -> projects/planned/rating_<1..10>). Files matched for move: %d, module dirs matched: %d\n' "$moved_count" "$moved_dir_count"
  else
    printf 'Dry run complete (%s -> %s). Files matched for move: %d, module dirs matched: %d\n' "$mode" "$static_target_dir" "$moved_count" "$moved_dir_count"
  fi
else
  if [[ "$mode" == "planned" ]]; then
    printf 'Done (planned -> projects/planned/rating_<1..10>). Moved files: %d, moved module dirs: %d, Cargo path updates: %d\n' "$moved_count" "$moved_dir_count" "$updated_paths"
  else
    printf 'Done (%s -> %s). Moved files: %d, moved module dirs: %d, Cargo path updates: %d\n' "$mode" "$static_target_dir" "$moved_count" "$moved_dir_count" "$updated_paths"
  fi
fi
