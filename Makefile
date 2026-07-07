.PHONY: move-completed move-completed-dry

move-completed:
	./scripts/move_completed_projects.sh

move-completed-dry:
	./scripts/move_completed_projects.sh --dry-run
