workflow "Testing" {
	on = "push"
	resolves = ["Unit Test"]
}

action "Unit Test" {
	uses = "docker://rust:slim"
	args = "cargo test"
}