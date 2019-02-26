workflow "Measure Performance" {
	on = "pull_request"
	resolves = [
		"Summarize perf",
		"Save perf results",
	]
}

action "On Push" {
	uses = "actions/bin/filter@master"
	args = "action 'opened|synchronize'"
}

action "On Merged|Sync" {
	uses = "actions/bin/filter@master"
	args = "action 'closed|synchronize'"
}

action "Save perf results" {
	needs = ["Perf cargo"]
	uses = "./.github/action/summarize-perf"
	args = "query '{exec: .command, time: .mean}' | commit"
	secrets = ["GITHUB_TOKEN"]
}

action "Summarize perf" {
	needs = ["On Merged|Sync"]
	uses = "./.github/action/summarize-perf"
	args = "summary | ./scripts/perfsum.py | comment"
	secrets = ["GITHUB_TOKEN"]
}

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
	env = { PERF_PREPARE = "cargo clean" }
}