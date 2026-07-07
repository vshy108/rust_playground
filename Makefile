.PHONY: move-completed move-completed-dry move-wip move-wip-dry

move-completed:
	./scripts/move_completed_projects.sh

move-completed-dry:
	./scripts/move_completed_projects.sh --dry-run

move-wip:
	./scripts/move_completed_projects.sh --mode wip

move-wip-dry:
	./scripts/move_completed_projects.sh --mode wip --dry-run
