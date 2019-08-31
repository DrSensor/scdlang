//@ts-ignore
type $name = $schema

export type $nameState = keyof $name["states"]

export type $nameEvent = {type: /*each*/EventIn}

export type $nameSchema = {
  states: {[source in $nameState]: {}}
}

export type EventIn = EventInState<$name>

type EventInState<Machine extends any> = {
  readonly [source in keyof Machine["states"]]: keyof Machine["states"][source]["on"];
}
