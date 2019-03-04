workflow "Testing" {
	on = "push"
	resolves = ["Unit Test cargo"]
}

workflow "Measure Performance" {
	on = "pull_request"
	resolves = [
		"Perf CLI release",
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
	needs = ["Perf cargo", "Perf CLI release"]
	uses = "./.github/action/summarize-perf"
	args = "query '{exec: .command, memory: .memory.peak, cpu: .cpu, time: .mean}' | commit"
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

action "Build Release cli as musl" {
	needs = "On Push"
	uses = "docker://rust:slim"
	runs = "./.github/entrypoint.sh"
	args = [
		"rustup target add x86_64-unknown-linux-musl",
		"cargo build --target x86_64-unknown-linux-musl --release -p ${BIN}",
		"mkdir -p ${HOME}/.bin/",
		"mv target/x86_64-unknown-linux-musl/release/${BIN} ${HOME}/.bin/${BIN}",
	]
	env = { BIN = "scrap" }
}

# TODO: Include perfquick.sh + profiler.sh for public consumption❗ (able to run multiple args)
action "Perf CLI release" {
	needs = "Build Release cli as musl"
	uses = "docker://alpine:latest"
	runs = "./.github/profiler.sh"
	args = ["${HOME}/.bin/${BIN}"]
	env = { BIN = "scrap" }
}
# ---------------------------------------------------------------

# 👇 👍 😊
action "debug" {
	# needs = ""	#✍
	uses = "actions/bin/debug@master"
	# args = ""		#✍
}