workflow "Measure Performance" {
	on = "pull_request"
	resolves = [
		"Summarize benchmark",
	]
}

action "On Push" {
	uses = "actions/bin/filter@master"
	args = "action 'opened|synchronize'"
}

action "Calculate cache size" {
	needs = "Perf [build]"
	uses = "docker://alpine:latest"
	# args = "du -sh $HOME/.cargo/registry" # $HOME is unknown
	runs = ["sh", "-c", "du -sh $HOME/.cargo/registry"]
}

action "Summarize benchmark" {
	needs = ["Perf [build]", "Perf [exec]"]
	uses = "./.github/action/summarize-perf"
	args = "query '{exec: .command, time: .mean}' | commit"
	secrets = ["GITHUB_TOKEN"]
}

action "Perf [build]" {
	needs = "On Push"
	uses = "./.github/action/perf"
	args = [
		"build --all",
		"build -p scdlang-core",
		"build -p scrap",
	]
	env = {
		CARGO_HOME = "/github/home/.cargo",
		PERF_PREPARE = "cargo clean",
	}
}

action "Perf [exec]" {
	needs = "Calculate cache size"
	uses = "./.github/action/perf"
	args = [
		"run",
		"run -p scrap",
	]
	env = { CARGO_HOME = "/github/home/.cargo" }
}