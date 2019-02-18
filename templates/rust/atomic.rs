enum State {
{{foreach(states)}}
	{{@this}}
{{/each}}
}

enum Event {
{{foreach(events)}}
	{{@this}}
{{/each}}
}

struct Machine {
	state: State
}

impl Machine {
	fn new() -> Self {
		Machine {
			state: State::{{initial}}
		}
	}

	fn send(&mut self, event: Event) {
	{{if(events.length === states.length)}}
		self.state = match event {
		{{each(events)}}
			Event::{{@this.name}} => match self.state {
			{{each(@this.target)}}
				State::{{@this.current}} => State::{{@this.next}}
			{{/each}}
			}
		{{/each}}
		}
	{{#else}}
		self.state = match self.state {
		{{each(states)}}
			State::{{@this.name}} => match event {
			{{each(@this.transition)}}
				Event::{{@this.event}} => State::{{@this.next}}
			{{/each}}
			}
		{{/each}}
		}
	{{/if}}
	}
}
