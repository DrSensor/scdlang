workflow "Measure Performance" {
  on = "pull_request"
  resolves = ["Perf [build]", "Perf [exec]"]
}

action "On Push" {
	uses = "actions/bin/filter@master"
	args = "action 'opened|synchronize'"
}

action "Perf [build]" {
	needs = "On Push"
  uses = "./.github/action/perf"
  args = [
		"build",
		"build -p scdlang-core",
		"build -p scrap",
	]
}

action "Perf [exec]" {
	needs = "On Push"
  uses = "./.github/action/perf"
  args = [
		"run --all",
		"run -p scrap",
	]
}