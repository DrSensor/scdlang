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

action "Cache dependencies" {
	needs = "On Push"
	uses = "docker://rust:latest"
	args = "cargo build"
	env = { CARGO_HOME = "/github/home/.cargo" }
}

action "Calculate cache size" {
	needs = "Cache dependencies"
	uses = "docker://alpine:latest"
	# args = "du -sh $HOME/.cargo/registry" # $HOME is unknown
	runs = ["sh", "-c", "du -sh $HOME/.cargo/registry"]
}

action "Perf [build]" {
	needs = "Cache dependencies"
	uses = "./.github/action/perf"
	args = [
		"build --all",
		"build -p scdlang-core",
		"build -p scrap",
	]
	env = { CARGO_HOME = "/github/home/.cargo" }
}

action "Perf [exec]" {
	needs = "Cache dependencies"
	uses = "./.github/action/perf"
	args = [
		"run -p scrap",
	]
	env = { CARGO_HOME = "/github/home/.cargo" }
}