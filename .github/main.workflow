workflow "Measure Performance" {
	on = "pull_request"
	resolves = [
		"Calculate cache size",
		"Perf [build]",
		"Perf [exec]",
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

action "Perf [build]" {
	needs = "On Push"
	uses = "./.github/action/perf"
	args = [
		"build --all",
		"build -p scdlang-core",
		"build -p scrap",
	]
	env = { CARGO_HOME = "/github/home/.cargo" }
}

action "Perf [exec]" {
	needs = "Calculate cache size"
	uses = "./.github/action/perf"
	args = [
		"run -p scrap",
	]
	env = { CARGO_HOME = "/github/home/.cargo" }
}