workflow "Testing" {
	on = "push"
	resolves = ["Unit Test cargo"]
}

workflow "Measure Performance" {
	on = "pull_request"
	resolves = [
		"Save perf results",
		"Summarize perf"
	]
}

# -------------------- Control Flow ---------------------------
action "On Push" {
	uses = "actions/bin/filter@master"
	args = "action 'opened|synchronize'"
}

action "On Merged|Sync" {
	uses = "actions/bin/filter@master"
	args = "action 'closed|synchronize'"
}
# ---------------------------------------------------------------

# ----------------------- Make Report ----------------------------
action "Save perf results" {
	needs = ["Perf cargo"]
	uses = "./.github/action/summarize-perf"
	args = "query '{exec: .command, time: .mean}' | commit"
	secrets = ["GITHUB_TOKEN"]
}

action "Summarize perf" {
	needs = ["Save perf results"]
	uses = "./.github/action/summarize-perf"
	args = "summary | ./scripts/perfsum.py | comment"
	secrets = ["GITHUB_TOKEN"]
}
# ---------------------------------------------------------------

# ------------------------ Process ------------------------------
action "Perf cargo" {
	needs = "On Push"
	uses = "./.github/action/perf"
	args = [
		"build --all",
		"build -p scdlang-core",
		"build -p scrap",
		"run",
		"run -p scrap",
	]
}

action "Unit Test cargo" {
	uses = "docker://rust:slim"
	args = "cargo test"
}
# ---------------------------------------------------------------
