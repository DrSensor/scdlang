// TODO: finish this template when compound state is implemented

//#region {#each states as $state_
//@ts-ignore
export type $state_StateIn = StateInState<$state_>

//@ts-ignore
export type $state_EventIn = EventInState<$state_>
//#endregion {/each}

type StateInState<Machine extends any> = {
  readonly [source in keyof Machine["states"]]: keyof Machine["states"][source]["states"];
}

type EventInState<Machine extends any> = {
  readonly [source in keyof Machine["states"]]: keyof Machine["states"][source]["on"];
}
