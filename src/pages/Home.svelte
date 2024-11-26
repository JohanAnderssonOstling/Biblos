<script lang="ts">
    import {getContext, onMount} from "svelte";
    import { core} from "@tauri-apps/api";
    import Lib from "./Library.svelte";
    import BookCover from "../components/BookCover.svelte";
	import Carousel from "../components/Carousel.svelte";
	import {invoke} from "@tauri-apps/api/core"
import * as dialog from "@tauri-apps/plugin-dialog"

    interface Lib {path: string, book_paths: string}
    let libs: Lib[] = [];
    const { push , pop} = getContext("stackview");

    onMount(() => {getLibs();});
    function getLibs() {
        invoke("get_libraries").then((lib) => {libs = lib})

    }
    function pushLib(path: string) {push(Lib, {libPath: path, currentPath: path})}

    async function addLibrary() {
        const path: string = await dialog.open({directory: true, multiple: true});
        invoke("create_libraries", {paths: path}).then(() => {getLibs()})
    }
    function deleteLibrary(lib: string) {
        invoke("delete_library", {libraryPath: lib}).then(() => {getLibs()})

    }
</script>
    <button on:click={addLibrary}>Add Library</button>
    <div class = "library-list">
        {#each libs as lib}
            <div class="library" >
                <div class = "button-row">

                    <h1 on:click={() => pushLib(lib.path)}>
                        <a href="#">
                        {lib.path.split("/").slice(-1)}
                        </a>
                        <button>&#8942;</button>
                    </h1>

                </div>

                <div class = "last-read">
                    {#each lib.book_paths as book}
                       <BookCover book_path={book} libPath={lib.path}></BookCover>
                    {/each}
                </div>
            </div>
        {/each}
    </div>

<style>
    .library-list {
        gap: 20px;
        display: grid;
        justify-content: center;
        grid-gap: 20px;
        grid-template-columns: repeat(auto-fit, minmax(600px, 664px));
    }
    .last-read {
        gap: 10px;
        display: flex;
        align-items: center;
        overflow-y: hidden;
        overflow-x: auto;
    }
    .library {
        border: 0px solid #aaa; border-radius: 4px;
        box-shadow: 4px 4px 16px rgba(0, 0, 0, 0.3);
        padding: 20px;
        height: 440px;
        contain: layout style;
    }

    .button-row {
        justify-content: center;
        display: flex;
    }

    a {
        font-family: "sans-serif";
        font-weight: normal;
        color: black;
    }
</style>