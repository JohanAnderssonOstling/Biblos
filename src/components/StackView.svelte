<!-- src/components/StackView.svelte -->
<script>
	import {derived, writable} from 'svelte/store';
	import {onMount, setContext} from "svelte";
	const stack = writable([])
	const STACK_CONTEXT_KEY = "stackview";
	export let root= null;
	const currentView = derived(stack, $stack => $stack[$stack.length - 1]);
	setContext(STACK_CONTEXT_KEY, { push, pop });
	let idCounter = 0;
	onMount(() => {if (root) {push(root);}});

	export function push(component, props = {}) {
		stack.update($stack => [
			...$stack,
			{ id: idCounter++, component, props },
		]);
	}

	export function pop() {
		stack.update($stack => $stack.slice(0, -1));
	}
</script>

<div class="stack-view">

    {#each $stack as view, index (view.id)}
        <div class:hidden={index < $stack.length - 1}>
            <svelte:component
                    this={view.component}
                    {...view.props}
            />
        </div>
    {/each}
</div>

<style>
    .stack-view {
        position: relative;
    }

    .stack-view > div {
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
    }

    .hidden {
        display: none;
    }
</style>
