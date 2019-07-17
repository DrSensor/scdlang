workflow "Testing" {
	on = "push"
	resolves = ["Test all rust project"]
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
		"build -p scdlang",
		"build -p scdlang_xstate",
		"build -p s-crap",
	]
}

action "Test all rust project" {
	uses = "docker://rust:slim"
	runs = "./.github/entrypoint.sh"
	args = [
		"cargo install just",
		"just test",
	]
	env = { PWD = "/github/workspace" }
}

action "Build Release cli as musl" {
	needs = "On Push"
	uses = "docker://rust:slim"
	runs = "./.github/entrypoint.sh"
	args = [
		"rustup target add x86_64-unknown-linux-musl",
		"apt-get update && apt-get install -y musl-tools",
		"cargo build --target x86_64-unknown-linux-musl --release --bin ${BIN}",
		"mkdir -p ${HOME}/.bin/",
		"mv target/x86_64-unknown-linux-musl/release/${BIN} ${HOME}/.bin/${BIN}",
	]
	env = { BIN = "scrap" }
}

action "Perf CLI release" {
	needs = "Build Release cli as musl"
	uses = "docker://python:alpine"
	runs = "./.github/profiler.sh"
	args = [
		"${HOME}/.bin/scrap code examples/simple.scl --format xstate",
		"${HOME}/.bin/scrap code examples/simple.scl --format xstate --stream",
		"${HOME}/.bin/scrap code examples/simple.scl --format smcat",
		"${HOME}/.bin/scrap code examples/simple.scl --format smcat --stream",
	],
	env = { PREPARE = "./scripts/gensample.py 1000 > examples/simple.scl" }
}
# ---------------------------------------------------------------

# 👇 👍 😊
action "debug" {
	# needs = ""	#✍
	uses = "actions/bin/debug@master"
	# args = ""		#✍
}